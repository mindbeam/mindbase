use crate::entity::{
    AgentId,
    Entity,
};
use ed25519_dalek::{
    Keypair,
    PublicKey,
};

use rand::rngs::OsRng;
use serde::{
    Deserialize,
    Serialize,
};
use sha2::Sha512;

// TODO 1 rename Agent to AgentId and AgentHandle to Agent
/// Arguably an Agent is also an Artifact, but this probably isn't crucial
#[derive(Serialize, Deserialize, Debug)]
pub enum AgentHandle {
    Genesis,
    Keyed { keypair: Keypair },
}

impl AgentHandle {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        Self::Keyed { keypair }
    }

    pub fn genesis() -> Self {
        Self::Genesis
    }

    pub fn is_genesis(&self) -> bool {
        match self {
            Self::Genesis => true,
            _ => false,
        }
    }

    pub fn pubkey(&self) -> Option<&PublicKey> {
        match self {
            Self::Genesis => None,
            Self::Keyed { keypair } => Some(&keypair.public),
        }
    }

    pub fn entity(&self) -> Entity {
        Entity::Agent(match self {
                          Self::Genesis => AgentId::Genesis,
                          Self::Keyed { keypair } => AgentId::Keyed { pubkey: keypair.public.as_bytes().clone(), },
                      })
    }
}

impl std::fmt::Display for AgentHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        match self {
            Self::Genesis => write!(f, "GenesisAgent()"),
            Self::Keyed { keypair } => {
                write!(f,
                       "{}",
                       base64::encode_config(&keypair.public.as_bytes()[0..10], STANDARD_NO_PAD))
            },
        }
    }
}
