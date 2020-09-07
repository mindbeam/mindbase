use crate::{
    agent::{
        signature::Signature,
        AgentId,
    },
    analogy::Analogy,
    artifact::ArtifactId,
    error::MBError,
    symbol::{
        Atom,
        Symbol,
    },
    Agent,
    MindBase,
};

use rusty_ulid::generate_ulid_bytes;
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;
#[derive(Clone, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct AllegationId(#[serde(serialize_with = "crate::util::serde_helper::as_base64",
                                deserialize_with = "crate::util::serde_helper::from_base64_16")]
                        pub(crate) [u8; 16]);

impl AllegationId {
    pub fn new() -> Self {
        AllegationId(generate_ulid_bytes())
    }

    pub fn from_base64(input: &str) -> Result<Self, MBError> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| MBError::Base64Error)?;
        let array: [u8; 16] = decoded[..].try_into().map_err(|_| MBError::TryFromSlice)?;
        Ok(AllegationId(array.into()))
    }

    pub fn base64(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }

    /// Create a "Narrow" Symbol which refers exclusively to this Allegation
    /// As a general rule, we should avoid using narrow symbols whenever possible
    /// This is because we want to be convergent with our neighbors. I am not an island.
    /// Narrow symbols should be created ONLY when referring to some other entities we just
    /// created, and no clustering is possible
    pub fn subjective(&self) -> Symbol {
        Symbol { atoms:         vec![Atom::up(self.clone())],
                 spread_factor: 0.0, }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_b16(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl std::convert::TryFrom<sled::IVec> for AllegationId {
    type Error = MBError;

    fn try_from(ivec: sled::IVec) -> Result<Self, MBError> {
        use std::convert::TryInto;
        Ok(Self((&ivec[..]).try_into().map_err(|_| MBError::TryFromSlice)?))
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
        write!(f, "{}", base64::encode_config(&self.0[12..16], STANDARD_NO_PAD))
    }
}
impl fmt::Debug for AllegationId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AllegationId:{}", base64::encode(&self.0))
    }
}

/// # Allegation
/// An allogation is kind of like an "Atom" of meaning. In the same way that you typically interact with molecules rather than
/// atoms in the physical world, so to do you interact with "Symbols" in the ontological world. These
/// molecules/symbols don't simply spring into existence however. Molecules must be built of atoms, and Symbols must be built of
/// Allegations.
///
/// NOMENCLATURE QUESTION: Symbol/Symbol/Allegation/Atom etc?
/// There must be a bifurcation between the Subjective and the Intersubjective.
/// The Subjective is actually more than just according to a person or agent.
/// It's also according to a context – A time, A place, A frame of mind, An intention.
/// As such, these subjective elements are really more like events. In this sense, the term Allegation is nice, because it implies
/// that provenance is involved. The question is: Is "Symbol" a nicer term for Allegation? In some sense its nicer, because it
/// symbolizes some occurrent. Unfortunately it also muddies the idea of a Symbol. A symbol is also kind of a symbol, because it
/// symbolizes an "idea" (Some ontologists quibble about Symbols being bad, as "Symbol" implies that it's a thought about a
/// thing rather than a symbol of that thing)
///
/// For this reason, I've been thinking about nomenclature like:
///    Proto-Symbol (Allegation) and Symbol (Symbol)
///       * Unfortunately this is muddy, because both allegations and symbols are symbols
/// or Subjective-Symbol and Intersubjective Symbol
///       * unfortunately this is muddy, because Subjectivity might be wrongly taken to mean "Person"-al, rather than situational.
///
/// To the degree that you possess a "Self", so too do you possess agency over your thoughts, feelings, and perceptions.
/// Unfortunately, that's about the extent if your agency, epistemologically speaking. Much To the chagrin of narcisists
/// everywhere, they (and you) possess no agency hatsoever over objectivity or objective truth.
///
/// So whatever shall we do to make sense of the world?
///
/// In MindBase an Allegation is essentially an opinion of, or measurement about the world which is attributable to a specific
/// Agent. Agents may then form `Symbols` from a collection of allegations which are believed to one degree of confidence or
/// another to be referring to approximately the "same" thing
/// See [`mindbase::symbol::Symbol`][Symbol] for more details
#[derive(Serialize, Deserialize)]
pub struct Allegation {
    /// TODO 3 - Consider renaming "Allegation*" to "Symbol*"
    pub id:        AllegationId,
    pub agent_id:  AgentId,
    // TODO 3 - Context (Date, time, place, etc)
    pub body:      Body,
    pub signature: Signature,
}

pub enum ArtifactList<'a> {
    None,
    One(&'a ArtifactId),
    Many(Vec<ArtifactId>),
}

impl Allegation {
    pub fn new<T>(agent: &Agent, body: T) -> Result<Self, MBError>
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

    /// Create a "Narrow" Symbol which refers exclusively to this Allegation
    /// As a general rule, we should avoid using narrow symbols whenever possible
    /// This is because we want to be convergent with our neighbors. I am not an island.
    /// Narrow symbols should be created ONLY when referring to some other entities we just
    /// created, and no clustering is possible
    pub fn subjective(&self) -> Symbol {
        Symbol { atoms:         vec![Atom::up(self.id().clone())],
                 spread_factor: 0.0, }
    }

    pub fn id(&self) -> &AllegationId {
        &self.id
    }

    // Get all artifacts referenced by this allegation
    pub fn referenced_artifacts(&self, mb: &MindBase) -> Result<ArtifactList, MBError> {
        match self.body {
            Body::Artifact(ref artifact_id) => Ok(ArtifactList::One(artifact_id)),
            Body::Unit => Ok(ArtifactList::None),
            Body::Analogy(ref analogy) => {
                // TODO 1 - This is a little strange.
                // Think about whether we actually want to do this

                let mut v: Vec<ArtifactId> = Vec::with_capacity(10);

                // Forward
                for atom in analogy.left.atoms.iter() {
                    match mb.get_allegation(atom.id())? {
                        Some(allegation) => {
                            // TODO 1 - need to put some upper bound on how much we want to recurse here
                            // QUESTION: What are the consequences of this uppper bound enforcement?
                            // TODO 2 - Encode in the number of levels removed?
                            // What about the trust score / weight of the agents who alledged them?
                            // NOTE: I think we may only need to include those allegations which are authored by ground symbol
                            // agents
                            match allegation.referenced_artifacts(mb)? {
                                ArtifactList::None => {},
                                ArtifactList::One(id) => v.push(id.clone()),
                                ArtifactList::Many(many) => v.extend(many),
                            }
                        },
                        None => return Err(MBError::AllegationNotFound),
                    }
                }

                // Backward
                for atom in analogy.right.atoms.iter() {
                    match mb.get_allegation(atom.id())? {
                        Some(allegation) => {
                            match allegation.referenced_artifacts(mb)? {
                                ArtifactList::None => {},
                                ArtifactList::One(id) => v.push(id.clone()),
                                ArtifactList::Many(many) => v.extend(many),
                            }
                        },
                        None => return Err(MBError::AllegationNotFound),
                    }
                }
                Ok(ArtifactList::Many(v))
            },
        }
    }
}

impl std::convert::AsRef<[u8]> for AllegationId {
    fn as_ref(&self) -> &[u8] {
        &self.0
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
            Body::Analogy(a) => write!(f, "Analogy({})", a),
            Body::Artifact(a) => write!(f, "Artifact({})", a),
        }
    }
}

// TODO 1 - Rename this to Symbolize
pub trait Alledgable: std::fmt::Debug {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, MBError>;
}
