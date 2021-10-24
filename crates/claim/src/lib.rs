mod body;
mod relation;
// mod traits;

use mindbase_symbol::{
    analogy::Analogy,
    symbol::{Symbol, SymbolMember},
    traits::Entity,
    AssociativeAnalogy, CategoricalAnalogy,
};

use keyplace::{AgentId, AgentKey, Signature};
use mindbase_util::Error;

use rusty_ulid::generate_ulid_bytes;
use serde::{Deserialize, Serialize};
use std::fmt;
use traits::Artifact;

use self::body::Body;

// Any Vertex is always according to some observer(s) (Claim(s))
// Any Edge is always according to some observer(s) (Claim(s))
// Any Vertex may have one or more values (artifacts)
// Any Edge may have one or more weights (artifacts)

//          | Vertex | Edge
//           -------------
// Claim    |   X    |  X
// Artifact |   X    |  X

#[derive(Clone, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct ClaimId(
    #[serde(
        serialize_with = "mindbase_util::serde_helper::as_base64",
        deserialize_with = "mindbase_util::serde_helper::from_base64_16"
    )]
    pub(crate) [u8; 16],
);

impl ClaimId {
    pub fn new() -> Self {
        ClaimId(generate_ulid_bytes())
    }

    pub fn from_base64(input: &str) -> Result<Self, Error> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| Error::Base64Error)?;
        let array: [u8; 16] = decoded[..].try_into().map_err(|_| mindbase_util::Error::TryFromSlice)?;
        Ok(ClaimId(array.into()))
    }

    pub fn base64(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }

    // /// Create a "Narrow" Symbol which refers exclusively to this claim
    // /// As a general rule, we should avoid using narrow symbols whenever possible
    // /// This is because we want to be convergent with our neighbors. I am not an island.
    // /// Narrow symbols should be created ONLY when referring to some other entities we just
    // /// created, and no clustering is possible
    // pub fn subjective<E> (&self) -> Symbol<Entity> {
    //     unimplemented!("should probably remove this")
    //     // Symbol {
    //     //     atoms: vec![Atom::up(self.clone())],
    //     //     spread_factor: 0.0,
    //     // }
    // }

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

impl mindbase_util::AsBytes for &ClaimId {
    fn as_bytes(&self) -> Vec<u8> {
        self.0[..].to_vec()
    }
}

impl fmt::Display for ClaimId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0[12..16], STANDARD_NO_PAD))
    }
}
impl fmt::Debug for ClaimId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClaimId:{}", base64::encode(&self.0))
    }
}

impl std::convert::TryFrom<&[u8]> for ClaimId {
    type Error = Error;

    fn try_from(ivec: &[u8]) -> Result<Self, Error> {
        use std::convert::TryInto;
        Ok(Self((&ivec[..]).try_into().map_err(|_| mindbase_util::Error::TryFromSlice)?))
    }
}

/// # Claim
/// An allogation is kind of like an "Atom" of meaning. In the same way that you typically interact with molecules rather than
/// atoms in the physical world, so to do you interact with "Symbols" in the ontological world. These
/// molecules/symbols don't simply spring into existence however. Molecules must be built of atoms, and Symbols must be built of
/// Claims.
///
/// NOMENCLATURE QUESTION: Symbol/Symbol/Claim/Atom etc?
/// There must be a bifurcation between the Subjective and the Intersubjective.
/// The Subjective is actually more than just according to a person or agent.
/// It's also according to a context – A time, A place, A frame of mind, An intention.
/// As such, these subjective elements are really more like events. In this sense, the term Claim is nice, because it implies
/// that provenance is involved. The question is: Is "Symbol" a nicer term for Claim? In some sense its nicer, because it
/// symbolizes some occurrent. Unfortunately it also muddies the idea of a Symbol. A symbol is also kind of a symbol, because it
/// symbolizes an "idea" (Some ontologists quibble about Symbols being bad, as "Symbol" implies that it's a thought about a
/// thing rather than a symbol of that thing)
///
/// For this reason, I've been thinking about nomenclature like:
///    Proto-Symbol (Claim) and Symbol (Symbol)
///       * Unfortunately this is muddy, because both claims and symbols are symbols
/// or Subjective-Symbol and Intersubjective Symbol
///       * unfortunately this is muddy, because Subjectivity might be wrongly taken to mean "Person"-al, rather than situational.
///
/// To the degree that you possess a "Self", so too do you possess agency over your thoughts, feelings, and perceptions.
/// Unfortunately, that's about the extent if your agency, epistemologically speaking. Much To the chagrin of narcisists
/// everywhere, they (and you) possess no agency hatsoever over objectivity or objective truth.
///
/// So whatever shall we do to make sense of the world?
///
/// In MindBase an Claim is essentially an opinion of, or measurement about the world which is attributable to a specific
/// Agent. Agents may then form `Symbols` from a collection of claims which are believed to one degree of confidence or
/// another to be referring to approximately the "same" thing
/// See [`mindbase::symbol::Symbol`][Symbol] for more details

// #[derive(Serialize, Deserialize)]
pub struct Claim<E, A>
where
    E: Entity,
    A: mindbase_hypergraph::traits::Value,
{
    /// TODO 3 - Consider renaming "Claim*" to "Symbol*"
    pub id: ClaimId,
    pub agent_id: AgentId,
    // TODO 3 - Context (Date, time, place, etc)
    pub body: Body<E, A>,
    pub signature: Signature,
}

// TODO - deal with claimant genericization for simplicity of testing

pub enum ArtifactList<'a, A: traits::Artifact> {
    None,
    One(&'a A),
    Many(Vec<A>),
}

trait ClaimStore<E: Entity> {}

impl<E, A> Claim<E, A>
where
    E: Entity,
    A: Artifact,
{
    pub fn new<T>(agentkey: &AgentKey, body: T) -> Result<Self, Error>
    where
        T: Into<Body<E, A>>,
    {
        let body: Body = body.into();
        let id = ClaimId::new();
        let agent_id = agentkey.id();

        let signature = Signature::new(agentkey, (&id, &agent_id, &body))?;

        Ok(Claim {
            id,
            agent_id,
            body,
            signature,
        })
    }

    /// Create a "Narrow" Symbol which refers exclusively to this Claim
    /// As a general rule, we should avoid using narrow symbols whenever possible
    /// This is because we want to be convergent with our neighbors. I am not an island.
    /// Narrow symbols should be created ONLY when referring to some other entities we just
    /// created, and no clustering is possible
    pub fn subjective(&self) -> Symbol<E> {
        unimplemented!()
        // should probably remove this

        // Symbol {
        //     atoms: vec![Atom::up(self.id().clone())],
        //     spread_factor: 0.0,
        // }
    }

    pub fn id(&self) -> &ClaimId {
        &self.id
    }

    // Get all artifacts referenced by this claim
    pub fn referenced_artifacts<AS>(&self, cs: SS) -> Result<ArtifactList<A>, Error>
    where
        CS: ClaimStore<A>,
    {
        match self.body {
            Body::Artifact(ref artifact_id) => Ok(ArtifactList::One(artifact_id)),
            Body::Unit => Ok(ArtifactList::None),
            Body::AssociativeAnalogy(ref analogy) => {
                // TODO 1 - This is a little strange.
                // Think about whether we actually want to do this

                let mut v: Vec<ArtifactId> = Vec::with_capacity(10);

                // Forward
                for atom in analogy.left.atoms.iter() {
                    match mb.get_claim(atom.id())? {
                        Some(claim) => {
                            // TODO 1 - need to put some upper bound on how much we want to recurse here
                            // QUESTION: What are the consequences of this uppper bound enforcement?
                            // TODO 2 - Encode in the number of levels removed?
                            // What about the trust score / weight of the agents who alledged them?
                            // NOTE: I think we may only need to include those claims which are authored by ground symbol
                            // agents
                            match claim.referenced_artifacts(mb)? {
                                ArtifactList::None => {}
                                ArtifactList::One(id) => v.push(id.clone()),
                                ArtifactList::Many(many) => v.extend(many),
                            }
                        }
                        None => return Err(MBError::ClaimNotFound),
                    }
                }

                // Backward
                for atom in analogy.right() {
                    match mb.get_claim(atom.id())? {
                        Some(claim) => match claim.referenced_artifacts(mb)? {
                            ArtifactList::None => {}
                            ArtifactList::One(id) => v.push(id.clone()),
                            ArtifactList::Many(many) => v.extend(many),
                        },
                        None => return Err(MBError::ClaimNotFound),
                    }
                }
                Ok(ArtifactList::Many(v))
            }
            _ => unimplemented!(),
        }
    }
}

impl std::convert::AsRef<[u8]> for ClaimId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Claim<E, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.body)
    }
}
