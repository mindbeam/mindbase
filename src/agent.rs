pub mod signature;

use ed25519_dalek::{
    Keypair,
    PublicKey,
};

use crate::{
    allegation::{
        Alledgable,
        Allegation,
    },
    error::Error,
    Artifact,
    MindBase,
};
use rand::rngs::OsRng;
use serde::{
    Deserialize,
    Serialize,
};
use sha2::Sha512;
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AgentId {
    #[serde(serialize_with = "crate::util::serde_helper::as_base64",
            deserialize_with = "crate::util::serde_helper::from_base64_32")]
    pubkey: [u8; 32],
}

impl AgentId {
    pub fn pubkey_short(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.pubkey[0..12], STANDARD_NO_PAD)
    }

    pub fn from_base64(input: &str) -> Result<Self, Error> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| Error::Base64Error)?;
        let array: [u8; 32] = decoded[..].try_into().map_err(|_| Error::TryFromSlice)?;
        Ok(AgentId { pubkey: array.into() })
    }
}

impl crate::util::AsBytes for &AgentId {
    fn as_bytes(&self) -> Vec<u8> {
        self.pubkey[..].to_vec()
    }
}
impl std::convert::AsRef<[u8]> for AgentId {
    fn as_ref(&self) -> &[u8] {
        &self.pubkey[..]
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
pub struct Agent {
    keypair: Keypair,
}

impl Agent {
    pub fn new() -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        Self { keypair }
    }

    pub fn id(&self) -> AgentId {
        AgentId { pubkey: self.keypair.public.as_bytes().clone(), }
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    pub fn pubkey(&self) -> &PublicKey {
        &self.keypair.public
    }

    /// Returns a list of AgentIDs to ascribe to for ground symbols
    /// We have to have charismatic subordination for at least some of our symbols, otherwise we'll never converge
    /// Mindbase intends to ship with at least one, and maybe multiple genesis agents, each with predefined symbols in the
    /// database for this purpose. This list should always contain the agent's own ID
    pub fn ground_agents(&self) -> Vec<AgentId> {
        // TODO 2 - pre-register a "genesis_en" agent, and include it here
        //
        vec![self.id()]
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f,
               "{}",
               base64::encode_config(&self.keypair.public.as_bytes()[0..10], STANDARD_NO_PAD))
    }
}

// impl Into<crate::allegation::Body> for Agent {
//     fn into(self) -> crate::allegation::Body {
//         crate::allegation::Body::Artifact(Artifact::Agent(self.id()))
//     }
// }

impl Alledgable for &Agent {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, Error> {
        let artifact_id = mb.put_artifact(self.id())?;
        let allegation = Allegation::new(agent, crate::allegation::Body::Artifact(artifact_id))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}

impl Into<Artifact> for AgentId {
    fn into(self) -> Artifact {
        Artifact::Agent(self)
    }
}
