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
};

use rusty_ulid::generate_ulid_bytes;
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct AllegationId(#[serde(serialize_with = "crate::util::serde_helper::as_base64",
                                deserialize_with = "crate::util::serde_helper::from_base64")]
                        pub(crate) [u8; 16]);

impl AllegationId {
    pub fn new() -> Self {
        AllegationId(generate_ulid_bytes())
    }

    pub fn base64(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }

    pub fn narrow_concept(&self) -> Concept {
        Concept { members:       vec![self.clone()],
                  spread_factor: 0.0, }
    }
}

impl std::convert::AsRef<[u8]> for AllegationId {
    fn as_ref(&self) -> &[u8] {
        &self.0
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

    /// Create a concept which points exclusively to this allegation
    /// Narrow concepts should be created ONLY when referring to some other entities we just created
    /// Otherwise it is lazy, and will result in a non-convergent graph
    pub fn to_narrow_concept(&self) -> Concept {
        Concept { members:       vec![self.id().clone()],
                  spread_factor: 0.0, }
    }

    pub fn id(&self) -> &AllegationId {
        &self.id
    }
}

impl fmt::Display for Allegation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.body)
    }
}

impl Into<Concept> for Allegation {
    fn into(self) -> Concept {
        self.to_narrow_concept()
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

impl Into<Vec<u8>> for Body {
    fn into(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Unit => write!(f, "Unit()"),
            Body::Agent(a) => write!(f, "Agent({})", a),
            Body::Artifact(a) => write!(f, "Artifact({})", a),
        }
    }
}
