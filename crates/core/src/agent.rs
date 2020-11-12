use mindbase_crypto::{AgentKey, Signature};
use mindbase_hypergraph::Claim;

use crate::{error::Error, MindBase};

use serde::{Deserialize, Serialize};
use std::fmt;

mod symbolize;

/// Arguably an Agent is also an Artifact, but this probably isn't crucial
#[derive(Debug)]
pub struct Agent {
    agentkey: AgentKey,
    ground_symbol_agents: Arc<Mutex<Vec<AgentId>>>,
    mindbase: MindBase,
}

/// The active form of an agent - Lets actually do some stuff
/// AgentKey and AgentIdent are not the active forms of an agent,
/// but rather the keepers of information about an agent
impl Agent {
    pub fn new(mindbase: Mindbase) -> Self {
        let agentkey = AgentKey::create(None);
        let ground_symbol_agents = Arc::new(Mutex::new(vec![self.id()]));

        Self {
            agentkey,
            ground_symbol_agents,
            mindbase,
        }
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
