use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum EntityId {
    Vertex(VertexId),
    Hyperedge(HyperedgeId),
}

/// VertexId is a ULID
#[derive(Serialize, Deserialize, Clone)]
pub struct VertexId(pub(crate) [u8; 16]);

/// HyperedgeId is a ULID
#[derive(Serialize, Deserialize, Clone)]
pub struct HyperedgeId(pub(crate) [u8; 16]);

impl Into<EntityId> for VertexId {
    fn into(self) -> EntityId {
        EntityId::Vertex(self)
    }
}
impl Into<EntityId> for HyperedgeId {
    fn into(self) -> EntityId {
        EntityId::Hyperedge(self)
    }
}
