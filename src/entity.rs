use rusty_ulid::generate_ulid_bytes;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::fmt;
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityId(#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")] pub(crate) [u8; 16]);

pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: AsRef<[u8]>,
          S: Serializer
{
    use base64::STANDARD_NO_PAD;
    serializer.serialize_str(&base64::encode_config(v.as_ref(), STANDARD_NO_PAD))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
    where D: Deserializer<'de>
{
    use serde::de::Error;
    use std::convert::TryInto;
    String::deserialize(deserializer).and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
                                     .map(|bytes| bytes[..].try_into())
                                     .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize")))
}

impl EntityId {
    pub fn new() -> Self {
        EntityId(generate_ulid_bytes())
    }

    pub fn base64(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }
}

impl std::convert::AsRef<[u8]> for EntityId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0, STANDARD_NO_PAD))
    }
}
impl fmt::Debug for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntityID:{}", base64::encode(&self.0))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Allegation {
    pub id:      EntityId,
    pub by:      EntityId, // Tighten this up to be agent specific?
    pub analogy: Analogy,
    // TODO 3: signature
}

impl Allegation {
    pub fn id(&self) -> EntityId {
        self.id.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{} Alleges that {}", self.by, self.analogy.to_string()).to_string()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Analogy {
    pub id:         EntityId,
    pub concept:    Concept,
    pub confidence: f32,
    pub memberof:   Concept,
}

impl Analogy {
    pub fn declare(concept: Concept, memberof: Concept) -> Self {
        Analogy { concept,
                  confidence: 1.0,
                  memberof,
                  id: EntityId::new() }
    }

    pub fn declare_neg(concept: Concept, memberof: Concept) -> Self {
        Analogy { concept,
                  confidence: -1.0,
                  memberof,
                  id: EntityId::new() }
    }

    pub fn to_string(&self) -> String {
        format!("{} is in the category of {} ({})",
                self.concept.to_string(),
                self.memberof.to_string(),
                self.confidence).to_string()
    }
}

/// Pointer to a region within Semantic/Knowledge-Space
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Concept {
    // # how would the agent know which entities they are referring to?
    // # I suppose the UI could remember a list of entities which are being
    // # converged in the rendering. Not really in love with the fact that
    // # the Agent has to pick specific entities as a representative sample
    // # Of the cluster they're actually referring to, but it will suffice
    // # for now I think.
    /// A list of entities which serve as a representative sample of the K-Space cluster
    pub members:       Vec<EntityId>,
    pub spread_factor: f32,
    /* # Here's a slightly different way, but still not great
     * # median_entity: Entity,
     * # radius: Float */
}

impl Concept {
    pub fn to_string(&self) -> String {
        let parts: Vec<String> = self.members.iter().map(|e| format!("{}", e)).collect();
        format!("[{}]", parts.join(","))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Entity {
    /// Unit is a uniquely identifiable 0-tuple, which is nameless, and occupies a region in K-Space which is indeterminate at
    /// the time of its creation
    Unit(EntityId),
    Agent(AgentId),
    Allegation(Allegation),
    Artifact(Artifact),
}

impl Entity {
    /// Create a concept which points exclusively to this entity
    /// Narrow concepts should be created ONLY when referring to some other entities we just created
    /// Otherwise it is lazy, and will result in a non-convergent graph
    pub fn narrow_concept(&self) -> Concept {
        Concept { members:       vec![self.id().clone()],
                  spread_factor: 0.0, }
    }

    pub fn id(&self) -> EntityId {
        match self {
            Entity::Unit(ref id) => id.clone(),
            Entity::Agent(ref a) => a.id(),
            Entity::Allegation(ref a) => a.id(),
            Entity::Artifact(ref a) => a.id(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Entity::Unit(id) => format!("Unit({})", id.to_string()),
            Entity::Agent(a) => a.id().to_string(),
            Entity::Allegation(a) => a.to_string(),
            Entity::Artifact(a) => a.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AgentId {
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

    pub fn id(&self) -> EntityId {
        use std::convert::TryInto;
        match self {
            Self::Genesis => EntityId(b"genesis\0\0\0\0\0\0\0\0\0".clone()),
            Self::Keyed { pubkey } => {
                let hack: [u8; 16] = pubkey[0..16].try_into().unwrap();
                EntityId(hack)
            },
        }
    }

    pub fn to_string(&self) -> String {
        format!("Agent({})", self.pubkey_short()).to_string()
    }
}

// TODO 2 - split Artifact out into its own module
// QUESTION - How do we save space on storing attributes, while allowing Agents to potentially treat them as different Entities?
// Just because the artifacts are identical doesn't mean the meaning behind them is.
// Does this mean we should store the artifacts multiple times? or Should we store artifacts separately, and reference them from
// an Entity? (seems like the case, but not sure about the extra layer of indirection)
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Artifact {
    pub(crate) id:   EntityId,
    pub(crate) kind: ArtifactKind,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ArtifactKind {
    Url(Url),
    FlatText(FlatText),
    DataGraph(DataGraph),
    DataNode(DataNode),
}

impl Artifact {
    pub fn id(&self) -> EntityId {
        self.id.clone()
    }

    pub fn to_string(&self) -> String {
        format!("Artifact({}:{})", self.id(), self.kind.to_string())
    }
}
impl ArtifactKind {
    pub fn to_string(&self) -> String {
        match self {
            Self::Url(_u) => unimplemented!(),
            Self::FlatText(t) => t.to_string(),
            Self::DataGraph(_d) => unimplemented!(),
            Self::DataNode(_n) => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Url {
    pub url: String,
}

/// Text of nonspecific structure, origin, and language
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FlatText {
    pub text: String,
}

impl FlatText {
    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}

// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataGraph {
    pub graph_type: Concept,
    pub bytes:      u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes:      Vec<EntityId>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode {
    pub node_type: Concept,
    pub data:      Vec<u8>,
    pub relations: Vec<DataNodeRelation>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNodeRelation {
    pub to:            EntityId,
    pub relation_type: Concept,
}
