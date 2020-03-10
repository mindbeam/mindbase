use crate::{
    agent::{
        signature::Signature,
        AgentId,
    },
    analogy::Analogy,
    artifact::ArtifactId,
    concept::Concept,
    error::Error,
    Agent,
    MindBase,
};

use rusty_ulid::generate_ulid_bytes;
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct AllegationId(#[serde(serialize_with = "crate::util::serde_helper::as_base64",
                                deserialize_with = "crate::util::serde_helper::from_base64_16")]
                        pub(crate) [u8; 16]);

pub(crate) const ALLEGATION_ID_SERIALIZED_SIZE: usize = 16;

impl AllegationId {
    pub fn new() -> Self {
        AllegationId(generate_ulid_bytes())
    }

    pub fn from_base64(input: &str) -> Result<Self, Error> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| Error::Base64Error)?;
        let array: [u8; 16] = decoded[..].try_into().map_err(|_| Error::TryFromSlice)?;
        Ok(AllegationId(array.into()))
    }

    pub fn base64(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }

    /// Create a "Narrow" Concept which refers exclusively to this Allegation
    /// As a general rule, we should avoid using narrow concepts whenever possible
    /// This is because we want to be convergent with our neighbors. I am not an island.
    /// Narrow concepts should be created ONLY when referring to some other entities we just
    /// created, and no clustering is possible
    pub fn subjective(&self) -> Concept {
        Concept { members:       vec![self.clone()],
                  spread_factor: 0.0, }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
impl std::convert::TryFrom<sled::IVec> for AllegationId {
    type Error = Error;

    fn try_from(ivec: sled::IVec) -> Result<Self, Error> {
        use std::convert::TryInto;
        Ok(Self((&ivec[..]).try_into().map_err(|_| Error::TryFromSlice)?))
    }
}

impl crate::util::AsBytes for &AllegationId {
    fn as_bytes(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }
}

impl fmt::Display for AllegationId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0, STANDARD_NO_PAD))
    }
}
impl fmt::Debug for AllegationId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntityID:{}", base64::encode(&self.0))
    }
}

/// # Allegation
/// To the degree that you possess a "Self", so too do you possess agency over your thoughts, feelings, and perceptions.
/// Unfortunately, that's about the extent if your agency, epistemologically speaking. Much To the chagrin of narcisists
/// everywhere, they (and you) possess no agency hatsoever over objectivity or objective truth.
///
/// So whatever shall we do to make sense of the world?
///
/// In MindBase an Allegation is essentially an opinion of, or measurement about the world which is attributable to a specific
/// Agent. Agents may then form `Concepts` from a collection of allegations which are believed to one degree of confidence or
/// another to be referring to approximately the "same" thing
/// See [`mindbase::concept::Concept`][Concept] for more details
#[derive(Serialize, Deserialize)]
pub struct Allegation {
    /// TODO 1 - Rename "Allegation*" to "Symbol*"
    pub id:        AllegationId,
    pub agent_id:  AgentId,
    // TODO 2 - Context (Date, time, place, etc)
    pub body:      Body,
    pub signature: Signature,
}

impl Allegation {
    pub fn new<T>(agent: &Agent, body: T) -> Result<Self, Error>
        where T: Into<Body>
    {
        let body: Body = body.into();
        let id = AllegationId::new();
        let agent_id = agent.id();

        let signature = Signature::new(agent, (&id, &agent_id, &body))?;

        Ok(Allegation { id,
                        agent_id,
                        body,
                        signature })
    }

    /// Create a "Narrow" Concept which refers exclusively to this Allegation
    /// As a general rule, we should avoid using narrow concepts whenever possible
    /// This is because we want to be convergent with our neighbors. I am not an island.
    /// Narrow concepts should be created ONLY when referring to some other entities we just
    /// created, and no clustering is possible
    pub fn subjective(&self) -> Concept {
        Concept { members:       vec![self.id().clone()],
                  spread_factor: 0.0, }
    }

    pub fn id(&self) -> &AllegationId {
        &self.id
    }

    pub fn reverse_lookup(&self) -> Option<Vec<u8>> {
        // TODO need to add prefixing for ArtifactId vs other stuff

        // Returns
        match self.body {
            // AgentId(32 bytes) AgentID(32 bytes)
            Body::Agent(ref _agent_id) => None,

            // AgentID(32 bytes) ArtifactId(16 bytes)
            Body::Artifact(ref artifact_id) => {
                use crate::util::AsBytes;
                let mut parts: Vec<u8> = Vec::with_capacity(2);
                parts.extend((&self.agent_id).as_bytes());
                parts.extend(artifact_id.as_ref());
                Some(parts)
            },

            // AgentId(32 bytes) AllegationId(16 bytes)? (need something to indicate this is a unit)
            Body::Unit => None,

            // Iterator of: AgentId(32 bytes) AllegationId(16 bytes)
            // Most likely this will have to be converted into an iterator so we can index this allegation under
            // all of its concept AllegationIDs AND its MemberOf Allegation IDs
            Body::Analogy(ref _a) => None,
        }
    }
}

impl fmt::Display for Allegation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.body)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Body {
    /// A Unit Allegation a globally unique entity with no payload
    Unit,

    /// An Agent Allegation is a globally unique reference to an actual Agent
    /// It is conceivabe that someone could want to construct different Allegations referencing the same AgentId
    /// Which are otherwise identical except for their AllegationId.
    Agent(AgentId),
    Analogy(Analogy),
    Artifact(ArtifactId),
}

impl crate::util::AsBytes for &Body {
    fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Unit => write!(f, "Unit()"),
            Body::Agent(a) => write!(f, "Agent({})", a),
            Body::Analogy(a) => write!(f, "Analogy({})", a),
            Body::Artifact(a) => write!(f, "Artifact({})", a),
        }
    }
}

pub trait Alledgable {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, Error>;
}
