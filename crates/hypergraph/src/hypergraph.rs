use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

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
pub struct HyperGraph<S, V, H, P = ()>
where
    S: mindbase_store::Store,
    V: traits::Weight,
    H: traits::Weight,
    P: traits::Provenance,
{
    // Need to store this due to RAI
    #[doc(hidden)]
    _store: S,
    /// Keyed on UUID for now, but this is ripe for optimization
    vertex_storage: S::Tree,
    /// Keyed on UUID for now, but this is ripe for optimization
    hyperedge_storage: S::Tree,
    /// Smaller weights may be stored directly in the hyper entity
    /// But larger weights should be stored separately
    /// Keyed by hash of weight
    vertex_weight_storage: S::Tree,
    hyperedge_weight_storage: S::Tree,

    /// Index the raw weight value directly to the hyperedge
    hyperedge_by_weight_index: S::Tree,

    _v: PhantomData<V>,
    _h: PhantomData<H>,
    _p: PhantomData<P>,
}

impl<S, V, H, P> HyperGraph<S, V, H, P>
where
    S: mindbase_store::Store,
    V: traits::Weight,
    H: traits::Weight,
    P: traits::Provenance,
{
    pub fn new(store: S) -> Result<Self, Error> {
        // Separate out the weights to be stored
        let vertex_weight_storage = store.open_tree("hypergraph::vertex_weight_storage")?;
        vertex_weight_storage.set_merge_operator(op_write_once);

        let hyperedge_weight_storage = store.open_tree("hypergraph::hyperedge_weight_storage")?;
        hyperedge_weight_storage.set_merge_operator(op_write_once);

        let vertexes = store.open_tree("hypergraph::vertex_storage")?;
        let hyperedges = store.open_tree("hypergraph::hyperedge_storage")?;
        let hyperedge_by_weight_index = store.open_tree("hypergraph::hyperedge_by_weight_index")?;

        Ok(HyperGraph {
            _v: PhantomData,
            _h: PhantomData,
            _p: PhantomData,
            _store: store,
            vertex_weight_storage,
            hyperedge_weight_storage,
            vertex_storage: vertexes,
            hyperedge_storage: hyperedges,
            hyperedge_by_weight_index,
        })
    }
    pub fn add_vertex<IV: Into<V>>(&self, weight: IV) -> Result<VertexId, Error> {
        let w = self.put_vertex_weight(weight.into())?;

        let id_bytes = generate_ulid_bytes();

        self.vertex_storage.insert(id_bytes, bincode::serialize(&StoredVertex(w))?)?;

        Ok(VertexId(id_bytes))
    }
    pub fn add_hyperedge(&self, hyperedge: Hyperedge<H>) -> Result<HyperedgeId, Error> {
        let w = self.put_hyperedge_weight(hyperedge.weight)?;

        let id_bytes = generate_ulid_bytes();

        self.hyperedge_storage
            .insert(id_bytes, bincode::serialize(&StoredHyperEdge(w, hyperedge.inner))?)?;

        Ok(HyperedgeId(id_bytes))
    }
    fn put_vertex_weight<T: Into<V>>(&self, into_weight: T) -> Result<VertexWeightRef, Error> {
        let weight: V = into_weight.into();

        let bytes = weight.get_bytes();
        let mut hasher = Sha512Trunc256::default();
        hasher.update(&bytes);
        let hash = hasher.finalize();

        let wr = if bytes.len() < 32 {
            VertexWeightRef::Inline(bytes.clone())
        } else {
            VertexWeightRef::Remote(StoredWeightId(hash.into()))
        };

        // Only store it if we haven't seen this one before
        self.vertex_weight_storage.merge(hash, bytes)?;

        Ok(wr)
    }
    fn put_hyperedge_weight<T: Into<H>>(&self, into_weight: T) -> Result<HyperedgeWeightRef, Error> {
        let weight: H = into_weight.into();

        let bytes = weight.get_bytes();
        let mut hasher = Sha512Trunc256::default();
        hasher.update(&bytes);
        let hash = hasher.finalize();

        let wr = if bytes.len() < 32 {
            HyperedgeWeightRef::Inline(bytes.clone())
        } else {
            HyperedgeWeightRef::Remote(StoredWeightId(hash.into()))
        };

        // Only store it if we haven't seen this one before
        self.vertex_weight_storage.merge(hash, bytes)?;

        Ok(wr)
    }
    // pub fn get_weight(&self, instance: WeightHandle) -> Result<W, Error> {
    //     let artifact_id = instance.artifact_id();

    //     match self.weight_storage.get(&artifact_id)? {
    //         Some(bytes) => Ok(VertexWeight::from_id_and_bytes(artifact_id, bytes)),
    //         None => return Err(Error::ArtifactNotFound),
    //     }
    // }
}

use std::marker::PhantomData;

use mindbase_store::{MemoryStore, Tree};
use rusty_ulid::generate_ulid_bytes;
use traits::Weight;

use crate::{
    entity::{HyperedgeId, VertexId},
    error::Error,
    hyperedge::HyperedgeInner,
    traits, Hyperedge,
};
impl<V, H, P> HyperGraph<MemoryStore, V, H, P>
where
    V: traits::Weight,
    H: traits::Weight,
    P: traits::Provenance,
{
    pub fn memory() -> Self {
        Self::new(MemoryStore::new()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredVertex(pub(crate) VertexWeightRef);

#[derive(Serialize, Deserialize)]
struct StoredHyperEdge(HyperedgeWeightRef, HyperedgeInner);

#[derive(Serialize, Deserialize)]
enum HyperedgeWeightRef {
    Inline(Vec<u8>),
    Remote(StoredWeightId),
}
/// The hash of the weight which was stored
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredWeightId([u8; 32]);

#[derive(Serialize, Deserialize)]
pub(crate) enum VertexWeightRef {
    Inline(Vec<u8>),
    Remote(StoredWeightId),
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
