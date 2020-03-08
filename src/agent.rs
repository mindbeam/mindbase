pub mod signature;

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
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum AgentId {
    // TODO 1 - get rid of Genesis. This should just be a known pubkey.
    // There may in fact be several geneses (AKA charismatically offered sets)
    Genesis,
    Keyed { pubkey: [u8; 32] },
}

impl AgentId {
    pub fn pubkey_short(&self) -> String {
        match self {
            Self::Genesis => "genesis".to_string(),
            Self::Keyed { pubkey } => {
                use base64::STANDARD_NO_PAD;
                base64::encode_config(&pubkey[0..12], STANDARD_NO_PAD)
            },
        }
    }
}
impl crate::util::AsBytes for &AgentId {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            AgentId::Genesis => b"genesis"[..].to_vec(),
            AgentId::Keyed { pubkey } => pubkey[..].to_vec(),
        }
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.pubkey_short())
    }
}
impl fmt::Debug for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AgentId:{}", &self.pubkey_short())
    }
}

/// Arguably an Agent is also an Artifact, but this probably isn't crucial
#[derive(Serialize, Deserialize, Debug)]
pub enum Agent {
    Genesis,
    Keyed { keypair: Keypair },
}

impl Agent {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        Self::Keyed { keypair }
    }

    pub fn genesis() -> Self {
        Self::Genesis
    }

    pub fn id(&self) -> AgentId {
        match self {
            Self::Genesis => AgentId::Genesis,
            Self::Keyed { keypair } => AgentId::Keyed { pubkey: keypair.public.as_bytes().clone(), },
        }
    }

    pub fn is_genesis(&self) -> bool {
        match self {
            Self::Genesis => true,
            _ => false,
        }
    }

    pub fn keypair(&self) -> Option<&Keypair> {
        match self {
            Self::Genesis => None,
            Self::Keyed { keypair } => Some(&keypair),
        }
    }

    pub fn pubkey(&self) -> Option<&PublicKey> {
        match self {
            Self::Genesis => None,
            Self::Keyed { keypair } => Some(&keypair.public),
        }
    }
}

impl std::fmt::Display for Agent {
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
