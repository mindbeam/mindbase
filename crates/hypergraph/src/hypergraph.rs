use std::{fmt::Display, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};
use traits::GraphInterface;

use crate::{
    entity::{undirected, Entity, EntityInner},
    traits, Error,
};

use std::io::Write;

use rusty_ulid::generate_ulid_bytes;
use toboggan_kv::{adapter::BTreeAdapter, Tree};

use crate::entity::EntityId;
/// ?? Claims are sometimes artifact instances, but illegal instance values are possible to represent
/// Mixed Hypergraph ( undirected edges are categories, directed are analogies? )

/// How do Hypergraphs contend with edges referencing edges? (This seems exactly how weight
/// composition ought to work, and there is likely a body of literature on recursion here)
///
/// In this case, an edge would be a symbol, and...
/// the distinction between edges and nodes starts to break down >_>

/// # HyperGraph
/// What's special vs a garden variety graph?
/// * deduplicated weights (Artifacts)
/// * indexed lookup of nodes and edges by weight
/// * provenance (and filtration by same)
#[derive(Debug)]
pub struct Hypergraph<S, W, P = ()>
where
    S: toboggan_kv::Toboggan,
    W: traits::Weight,
    P: traits::Provenance,
{
    // Need to store this due to RAI
    #[doc(hidden)]
    _store: S,
    /// Keyed on UUID for now, but this is ripe for optimization
    entity_storage: S::Tree,
    /// Smaller weights may be stored directly in the hyper entity
    /// But larger weights should be stored separately
    /// Keyed by hash of weight
    weight_storage: S::Tree,

    /// Index the raw weight value directly to the hyperedge
    entity_by_weight_index: S::Tree,

    _w: PhantomData<W>,
    _p: PhantomData<P>,
}

impl<S, W, P> Hypergraph<S, W, P>
where
    S: toboggan_kv::Toboggan,
    W: traits::Weight,
    P: traits::Provenance,
{
    pub fn new(store: S) -> Result<Self, Error> {
        // Store weights separately from entities
        let weight_storage = store.open_tree("hypergraph::weight_storage")?;
        weight_storage.set_merge_operator(op_write_once);

        let entity_storage = store.open_tree("hypergraph::entity_storage")?;
        let entity_by_weight_index = store.open_tree("hypergraph::entity_by_weight_index")?;

        Ok(Hypergraph {
            _w: PhantomData,
            _p: PhantomData,
            _store: store,
            weight_storage,
            entity_storage,
            entity_by_weight_index,
        })
    }
    pub fn get_weight(&self, entity_id: &EntityId) -> Result<W, Error> {
        match self.entity_storage.get(entity_id.0)? {
            Some(entity_bytes) => {
                let sv: StoredEntity = deserialize(&entity_bytes)?;
                Ok(self.get_weight_by_ref(sv.0)?)
            }
            None => Err(Error::NotFound),
        }
    }

    fn put_weight<T: Into<W>>(&self, into_weight: T) -> Result<WeightRef, Error> {
        let weight: W = into_weight.into();

        let bytes = serialize(&weight)?;
        let mut hasher = Sha512Trunc256::default();
        hasher.update(&bytes);
        let hash = hasher.finalize();

        let wr = if bytes.len() < 32 {
            WeightRef::Inline(bytes.clone())
        } else {
            WeightRef::Remote(StoredWeightId(hash.into()))
        };

        // Only store it if we haven't seen this one before
        self.weight_storage.merge(hash, bytes)?;

        Ok(wr)
    }
    fn get_weight_by_ref(&self, wr: WeightRef) -> Result<W, Error> {
        match wr {
            WeightRef::Inline(ref bytes) => Ok(deserialize(bytes)?),
            WeightRef::Remote(id) => match self.weight_storage.get(id.0)? {
                Some(ref bytes) => Ok(deserialize(bytes)?),
                None => return Err(Error::NotFound),
            },
        }
    }

    pub fn dump_entities<O: Write>(&self, mut writer: O) -> Result<(), Error>
    where
        W: std::fmt::Debug,
    {
        for entity_rec in self.entity_storage.iter() {
            let (id_bytes, bytes) = entity_rec?;
            let entity_id = EntityId::from_slice(&id_bytes[0..16])?;
            let entity: StoredEntity = deserialize(&bytes)?;
            write!(
                writer,
                "{} = {:?}: {:?}\n",
                entity_id,
                self.get_weight_by_ref(entity.0)?,
                entity.1
            )?;
        }
        Ok(())
    }
}

fn serialize<T: Serialize>(thing: &T) -> Result<Vec<u8>, Error> {
    Ok(bincode::serialize(thing)?)
}
fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    Ok(bincode::deserialize(bytes)?)
}

impl<S, W, P> GraphInterface<W> for Hypergraph<S, W, P>
where
    S: toboggan_kv::Toboggan,
    W: traits::Weight,
    P: traits::Provenance,
{
    /// Insert an entity into the hypergraph
    /// ```
    /// use mindbase_hypergraph::{HyperGraph,entity};
    /// use toboggan_kv::adapter::BTreeAdapter;
    /// let graph : HyperGraph<BTreeAdapter,&'str,()> = HyperGraph::memory();
    /// graph.insert(entity::vertex("123")).unwrap()
    /// ```
    fn insert(&self, entity: Entity<W>) -> Result<EntityId, Error> {
        let w = self.put_weight(entity.weight)?;

        let id_bytes = generate_ulid_bytes();

        self.entity_storage
            .insert(id_bytes, bincode::serialize(&StoredEntity(w, entity.inner))?)?;

        Ok(EntityId(id_bytes).into())
    }

    fn get(&self, entity_id: &EntityId) -> Result<Entity<W>, Error> {
        match self.entity_storage.get(entity_id.0)? {
            Some(entity_bytes) => {
                let sv: StoredEntity = bincode::deserialize(&entity_bytes)?;
                Ok(Entity {
                    weight: self.get_weight_by_ref(sv.0)?,
                    inner: sv.1,
                })
            }
            None => Err(Error::NotFound),
        }
    }
}

impl<W, P> Hypergraph<BTreeAdapter, W, P>
where
    W: traits::Weight,
    P: traits::Provenance,
{
    pub fn memory() -> Self {
        Self::new(BTreeAdapter::new()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct StoredEntity(WeightRef, EntityInner);

#[derive(Serialize, Deserialize)]
enum WeightRef {
    Inline(Vec<u8>),
    Remote(StoredWeightId),
}
/// The hash of the weight which was stored
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredWeightId([u8; 32]);

fn op_write_once(
    _key: &[u8],               // the key being merged
    last_bytes: Option<&[u8]>, // the previous value, if one existed
    op_bytes: &[u8],           /* the new bytes being merged in */
) -> Option<Vec<u8>> {
    match last_bytes {
        Some(_) => None,
        None => Some(op_bytes.to_vec()),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn insert() -> Result<(), std::io::Error> {
        use crate::{entity, Hypergraph};
        use toboggan_kv::adapter::BTreeAdapter;
        let graph = Hypergraph::<BTreeAdapter, String>::memory();
        use crate::traits::GraphInterface;

        let a = graph.insert(entity::vertex(
            "This is the weight associated with an entity to be created (of type vertex)",
        ))?;
        let b = graph.insert(entity::vertex(
            "This is a different weight associated with a DIFFERENT entity to be created (of type vertex)",
        ))?;
        let c = graph.insert(entity::vertex("This weight is shared by multiple (vertex) entities"))?;
        let d = graph.insert(entity::vertex("This weight is shared by multiple (vertex) entities"))?;

        let x = graph.insert(entity::undirected(
            "This weight is associated with an entity of type undirected (which is a hyperedege)",
            [a, b, c, d],
        ))?;
        let y = graph.insert(entity::directed(
            "This weight is associated with an entity of type directed (which is a hyperedege)",
            [a, b], // This is the "From" side of the hyperedge
            [c, d], // This is the "To" side of the hyperedge
        ))?;

        let _z = graph.insert(entity::directed(
            "Crucially, hyperedges can also include other hyperedges",
            [x, y],       // From some hyperedge entities
            [a, b, c, d], // To some other entities
        ))?;

        // let mut out: Vec<u8> = Vec::new();
        // graph.dump_entities(&mut out).unwrap();
        // println!("{}", String::from_utf8(out).unwrap());
        Ok(())
    }
}

// /// Convenience function, equivalent to
// /// ```
// /// # use mindbase_hypergraph::{HyperGraph,entity};
// /// # use toboggan_kv::adapter::BTreeAdapter;
// /// # let graph : HyperGraph<BTreeAdapter,&'str,()> = HyperGraph::memory();
// /// graph.insert(entity::vertex("123")).unwrap();
// /// ```
// pub fn insert_vertex<IW: Into<W>>(&self, weight: IW) -> Result<EntityId, Error> {
//     self.insert(crate::entity::vertex(weight))
// }
// ///
// pub fn insert_directed<WI, F, T>(&self, weight: WI, from: F, to: T) -> Result<EntityId, Error>
// where
//     WI: Into<W>,
//     F: Into<Vec<EntityId>>,
//     T: Into<Vec<EntityId>>,
// {
//     self.insert(crate::entity::directed(weight, from, to))
// }
// pub fn insert_undirected<WI, M>(&self, weight: WI, members: M) -> Result<EntityId, Error>
// where
//     WI: Into<W>,
//     M: Into<Vec<EntityId>>,
// {
//     self.insert(crate::entity::undirected(weight, members))
// }
