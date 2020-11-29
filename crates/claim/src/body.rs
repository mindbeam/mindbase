use std::fmt;

use mindbase_symbol::{AssociativeAnalogy, CategoricalAnalogy, Entity};

use crate::artifact::ArtifactId;
pub trait Artifact {}
// #[derive(Serialize, Deserialize)]
pub enum ClaimBody<E: Entity, A: Artifact> {
    /// A Unit Claim a globally unique entity with no payload
    Unit,

    /// An Agent Claim is a globally unique entity which references to an actual Agent
    /// one could construct other Claims which were distinct in their identity, but reference the same AgentId
    AssociativeAnalogy(AssociativeAnalogy<E>),
    CategoricalAnalogy(CategoricalAnalogy<E>),
    Artifact(A),
}

// impl mindbase_util::AsBytes for &ClaimBody {
//     fn as_bytes(&self) -> Vec<u8> {
//         bincode::serialize(self).unwrap()
//     }
// }

impl<E, A> From<ArtifactId> for ClaimBody<E, A> {
    fn from(id: ArtifactId) -> Self {
        ClaimBody::Artifact(id)
    }
}

impl<E, A> fmt::Display for ClaimBody<E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClaimBody::Unit => write!(f, "Unit()"),
            ClaimBody::AssociativeAnalogy(a) => write!(f, "Assoc({})", a),
            ClaimBody::CategoricalAnalogy(c) => write!(f, "Cat({})", c),
            ClaimBody::Artifact(a) => write!(f, "Artifact({})", a),
        }
    }
}
