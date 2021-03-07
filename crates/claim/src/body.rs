use std::fmt;

use mindbase_symbol::{traits::Entity, AssociativeAnalogy, CategoricalAnalogy};

use crate::traits;

// #[derive(Serialize, Deserialize)]
pub enum Body<E: Entity, A: traits::Artifact> {
    /// A Unit Claim a globally unique entity with no payload
    Unit,
    Artifact(A),
    Relation{
        // Do we want the claim graph to be thinking about fuzzysets?
        // Or should it be generic over that? I'm not sure how to make this generic
        // Should probably just press ahead without generics for now
        set: PolarFuzzySet,
        type: Symbol
    }


    /// An Agent Claim is a globally unique entity which references to an actual Agent
    /// one could construct other Claims which were distinct in their identity, but reference the same AgentId
    // AssociativeAnalogy(AssociativeAnalogy<E>),
    // CategoricalAnalogy(CategoricalAnalogy<E>),
    // Artifact(A),
}

// impl mindbase_util::AsBytes for &ClaimBody {
//     fn as_bytes(&self) -> Vec<u8> {
//         bincode::serialize(self).unwrap()
//     }
// }

impl<E, A> From<A> for Body<E, A>
where
    A: traits::Artifact,
{
    fn from(id: A) -> Self {
        Body::Artifact(id)
    }
}

impl<E, A> fmt::Display for Body<E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Unit => write!(f, "Unit()"),
            Body::AssociativeAnalogy(a) => write!(f, "Assoc({})", a),
            Body::CategoricalAnalogy(c) => write!(f, "Cat({})", c),
            Body::Artifact(a) => write!(f, "Artifact({})", a),
        }
    }
}
