use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl Display for VertexId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "v:{}", b)
    }
}
impl Debug for VertexId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "vertex_id:{}", b)
    }
}
impl Display for HyperedgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "h:{}", b)
    }
}
impl Debug for HyperedgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "hyperedge_id:{}", b)
    }
}
