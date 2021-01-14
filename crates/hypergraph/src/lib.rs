use std::marker::PhantomData;

use error::Error;
pub mod error;
pub mod traits;

pub struct Vertex {
    weight: WeightId,
}

pub struct Hyperedge {
    weight: WeightId,
    inner: HyperedgeInner,
}
pub enum HyperedgeInner {
    Undirected(Vec<EntityId>),
    Directed(Vec<EntityId>, Vec<EntityId>),
}

pub enum EntityId {
    Vertex(VertexId),
    Hyperedge(HyperedgeId),
}

pub struct VertexId(usize);
pub struct HyperedgeId(usize);
pub struct WeightId(usize);

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
#[derive(Default, Debug)]
pub struct HyperGraph<S, W, P>
where
    S: mindbase_store::Store,
    W: traits::Weight,
    P: traits::Provenance,
{
    // Need to store this due to RAI
    _store: S,
    vertex_storage: S::Tree,
    hyperedge_storage: S::Tree,
    weight_storage: S::Tree,
    hyperedge_by_weight_index: S::Tree,

    _w: PhantomData<W>,
    _p: PhantomData<P>,
}

// pub enum Weight {
//     Inline(u8, Inner),
//     Remote(Arc<[u8]>),
// }

impl<S, W, P> HyperGraph<S, W, P>
where
    S: mindbase_store::Store,
    W: traits::Weight,
    P: traits::Provenance,
{
    pub fn new(store: S) -> Result<Self, Error> {
        // Separate out the weights to be stored
        let weight_storage = store.open_tree("hypergraph::weight_storage")?;

        weight_storage.set_merge_operator(op_write_once);

        let vertexes = store.open_tree("hypergraph::vertex_storage")?;
        let hyperedges = store.open_tree("hypergraph::hyperedge_storage")?;
        let hyperedge_by_weight_index = store.open_tree("hypergraph::hyperedge_by_weight_index")?;

        Ok(HyperGraph {
            _w: PhantomData,
            _p: PhantomData,
            _store: store,
            weight_storage,
            vertex_storage: vertexes,
            hyperedge_storage: hyperedges,
            hyperedge_by_weight_index,
        })
    }
    // vertices don't currently support weights
    pub fn add_vertex<IW: Into<W>>(&self, weight: IW) -> Result<VertexId, Error> {
        let weight_id = self.put_weight(weight.into())?;

        self.
        //     // self.entities.insert(instance_id.as_ref(), bytes)?;
    }
    pub fn add_hyperedge<V, IVI, IW>(&self, hyperedge: IH ) -> Result<HyperedgeId, Error>
    where

        IH: Into<Hyperedge>,

    {
        VI: Iterator<Item = VertexId>,
        IW: Into<W>,
        let vertex_iter: VI = vertices.into();
        let weight_handle = ;

        let he = Hyperedge {
            weight: self.put_weight(weight)?,
            inner: 
        };

        self.hyperedge_storage.insert(he.as_ref(), bytes)?;

        unimplemented!()
    }
    pub fn put_weight<T: Into<W>>(&mut self, into_weight: T) -> Result<(), Error> {
        //     let weight: W = into_weight.into();

        //     let (hash, bytes) = weight.get_hash_and_bytes();
        //     // Only store it if we haven't seen this one before

        //     self.weight_storage.merge(hash, bytes)?;

        //     // either way we want to create an instance
        //     // let instance = W::instantiate(&hash);
        //     // let (instance_id, bytes) = instance.get_id_and_bytes();
        //     // self.entities.insert(instance_id.as_ref(), bytes)?;
        unimplemented!();
        Ok(())
    }
    // pub fn get_weight(&self, instance: WeightHandle) -> Result<W, Error> {
    //     let artifact_id = instance.artifact_id();

    //     match self.weight_storage.get(&artifact_id)? {
    //         Some(bytes) => Ok(VertexWeight::from_id_and_bytes(artifact_id, bytes)),
    //         None => return Err(Error::ArtifactNotFound),
    //     }
    // }
}

use mindbase_store::{MemoryStore, Tree};
impl<W, P> HyperGraph<MemoryStore, W, P>
where
    W: traits::Weight,
    P: traits::Provenance,
{
    pub fn memory() -> Self {
        Self::new(MemoryStore::new()).unwrap()
    }
}

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
