pub mod signature;

use mindbase_crypto::AgentKey;

use crate::{
    claim::{Alledgable, Claim},
    error::MBError,
    Artifact, MindBase,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct AgentId {
    #[serde(
        serialize_with = "crate::util::serde_helper::as_base64",
        deserialize_with = "crate::util::serde_helper::from_base64_32"
    )]
    pubkey: [u8; 32],
}

impl AgentId {
    pub fn pubkey_short(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.pubkey[0..12], STANDARD_NO_PAD)
    }

    pub fn from_base64(input: &str) -> Result<Self, MBError> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| MBError::Base64Error)?;
        let array: [u8; 32] = decoded[..].try_into().map_err(|_| mindbase_util::Error::TryFromSlice)?;
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
#[derive(Debug)]
pub struct Agent {
    agentkey: AgentKey,
}

impl Agent {
    pub fn new() -> Self {
        let agentkey = AgentKey::create();
        Self { agentkey }
    }

    pub fn id(&self) -> AgentId {
        AgentId {
            pubkey: self.agentkey.pubkey().clone(),
        }
    }

    pub fn agentkey(&self) -> &AgentKey {
        &self.agentkey
    }

    pub fn pubkey(&self) -> [u8; 32] {
        self.agentkey.pubkey()
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
        write!(
            f,
            "{}",
            base64::encode_config(&self.agentkey.pubkey()[0..10], STANDARD_NO_PAD)
        )
    }
}

// impl Into<crate::allegation::Body> for Agent {
//     fn into(self) -> crate::allegation::Body {
//         crate::allegation::Body::Artifact(Artifact::Agent(self.id()))
//     }
// }

impl Alledgable for &Agent {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let artifact_id = mb.put_artifact(self.id())?;
        let allegation = Claim::new(agent, crate::claim::Body::Artifact(artifact_id))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}

impl Into<Artifact> for AgentId {
    fn into(self) -> Artifact {
        Artifact::Agent(self)
    }
}
