use std::{convert::TryInto, fmt::Display, marker::PhantomData, sync::atomic::AtomicU64};

use inverted_index_util::entity_list::insert_entity_mut;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};
use traits::GraphInterface;

use crate::{
    adapter::StorageAdapter,
    entity::{undirected, Entity, EntityInner},
    index,
    traits::{self, Value},
    Error,
};

use std::io::Write;

use rusty_ulid::generate_ulid_bytes;

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
struct Hypergraph<Stor, Sym, Val, Prov> {
    adapter: Stor,
}

// TODO: invert the factorization such that we implement put_* and get_* for Sled directly
impl<Stor, Sym, Val, Prov> Hypergraph<Stor, Sym, Val, Prov>
where
    Stor: StorageAdapter<Sym, Val, Prov>,
    Sym: Serialize + DeserializeOwned,
    Val: Serialize + DeserializeOwned,
    Prov: traits::Provenance,
{
    pub fn new(adapter: Stor) {
        Hypergraph { adapter }
    }
    /// Insert an entity into the hypergraph
    /// ```
    /// use mindbase_hypergraph::{HyperGraph,entity};
    /// use toboggan_kv::adapter::BTreeAdapter;
    /// let graph : HyperGraph<BTreeAdapter,&'str,()> = HyperGraph::memory();
    /// graph.insert(entity::vertex("123")).unwrap()
    /// ```
    pub fn insert(&self, entity: Entity<Sym, Val>) -> Result<EntityId, Error> {
        self.adapter.insert(entity)
    }

    fn get(&self, entity_id: &EntityId) -> Result<Entity<Key, Val>, Error> {
        match self.entity_storage.get(entity_id.0)? {
            Some(entity_bytes) => {
                let sv: StoredEntity = bincode::deserialize(&entity_bytes)?;

                // let properties: Vec::new();
                Ok(Entity { properties, inner: sv.1 })
            }
            None => Err(Error::NotFound),
        }
    }

    fn get_adjacencies(&self, entity_id: &EntityId) -> Result<Vec<EntityId>, Error> {
        match self.idx_entity_to_hyperedge.get(entity_id.0)? {
            Some(bytes) => Ok(bytes.chunks_exact(16).map(|b| EntityId(b.try_into().unwrap())).collect()),
            None => Ok(vec![]),
        }
    }

    fn get_adjacencies_matching<F>(&self, entity_id: &EntityId, filter: F) -> Result<Vec<EntityId>, Error>
    where
        F: Fn(&W) -> Result<bool, Error>,
    {
        // TODO - How do we index by weight AND symbol - decompose?
        // weights might contain multiple symbols in the future, but for
        // now they may only contain one.
        // Maybe a way to interrogate the weight to get its symbol(s), and index by that
        // How do we index by fuzzy symbols?

        let entity_ids = self.get_adjacencies(entity_id)?;
        println!("Adjacencies unfiltered: {:?}", entity_ids);

        let mut out = Vec::new();

        for entity_id in entity_ids {
            let entity = self.get(&entity_id)?;
            if filter(&entity.weight)? {
                out.push(entity_id)
            }
        }
        Ok(out)
    }
}

#[derive(Serialize, Deserialize)]
struct StoredProperty(SymbolRef, ValueRef);

#[derive(Serialize, Deserialize)]
struct StoredEntity(Vec<StoredProperty>, EntityInner);

#[derive(Serialize, Deserialize)]
enum SymbolRef {
    Inline(Vec<u8>),
    Remote(StoredValueId),
}

#[derive(Serialize, Deserialize)]
enum ValueRef {
    Inline(Vec<u8>),
    Remote(StoredValueId),
}

type ValueId = [u8; 32];
/// The hash of the weight which was stored
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredValueId(ValueId);

impl ValueRef {
    fn id(&self) -> ValueId {
        match self {
            ValueRef::Inline(bytes) => {
                let mut hasher = Sha512Trunc256::default();
                hasher.update(bytes);
                let id: ValueId = hasher.finalize().into();
                id
            }
            ValueRef::Remote(StoredValueId(id)) => id.clone(),
        }
    }
}

type SymbolId = [u8; 32];
/// The hash of the weight which was stored
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredSymbolId(SymbolId);

impl SymbolRef {
    fn id(&self) -> ValueId {
        match self {
            SymbolRef::Inline(bytes) => {
                let mut hasher = Sha512Trunc256::default();
                hasher.update(bytes);
                let id: ValueId = hasher.finalize().into();
                id
            }
            SymbolRef::Remote(StoredValueId(id)) => id.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn insert() -> Result<(), std::io::Error> {
        use crate::{entity, Hypergraph};
        use adapter::BTreeAdapter;
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
