use crate::allegation::AllegationId;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

use crate::{
    agent::Agent,
    allegation::{
        Alledgable,
        Allegation,
    },
    error::MBError,
    symbol::Symbol,
    AgentId,
    MindBase,
};
use sha2::{
    Digest,
    Sha512Trunc256,
};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactId(#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")] pub(crate) [u8; 32]);

pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
    where T: AsRef<[u8]>,
          S: Serializer
{
    use base64::STANDARD_NO_PAD;
    serializer.serialize_str(&base64::encode_config(v.as_ref(), STANDARD_NO_PAD))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where D: Deserializer<'de>
{
    use serde::de::Error;
    use std::convert::TryInto;
    String::deserialize(deserializer).and_then(|string| base64::decode(&string).map_err(|err| D::Error::custom(err.to_string())))
                                     .map(|bytes| bytes[..].try_into())
                                     .and_then(|opt| opt.map_err(|_| D::Error::custom("failed to deserialize")))
}

impl fmt::Display for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0, STANDARD_NO_PAD))
    }
}
impl fmt::Debug for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArtifactId:{}", base64::encode(&self.0))
    }
}

impl std::convert::AsRef<[u8]> for ArtifactId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Into<crate::allegation::Body> for ArtifactId {
    fn into(self) -> crate::allegation::Body {
        crate::allegation::Body::Artifact(self)
    }
}

impl ArtifactId {
    pub fn alledge(self, agent: &Agent, mb: &MindBase) -> Result<AllegationId, MBError> {
        let allegation = Allegation::new(agent, self)?;
        mb.put_allegation(&allegation)
    }

    pub fn from_base64(input: &str) -> Result<Self, MBError> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| MBError::Base64Error)?;
        let array: [u8; 32] = decoded[..].try_into().map_err(|_| MBError::TryFromSlice)?;
        Ok(ArtifactId(array.into()))
    }
}

impl std::convert::TryFrom<sled::IVec> for ArtifactId {
    type Error = MBError;

    fn try_from(ivec: sled::IVec) -> Result<Self, MBError> {
        use std::convert::TryInto;
        Ok(Self((&ivec[..]).try_into().map_err(|_| MBError::TryFromSlice)?))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Artifact {
    Agent(AgentId),
    Url(Url),
    FlatText(Text),
    DataGraph(DataGraph),
    DataNode(DataNode),
}

impl Artifact {
    pub fn id(&self) -> ArtifactId {
        let mut hasher = Sha512Trunc256::new();

        // TODO 5 switch to CapnProto or similar. Artifact storage and wire representation should be identical
        // Therefore we should hash that
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        hasher.input(&encoded);

        let result = hasher.result();

        ArtifactId(result.into())
    }

    /// Might as well Serialize and hash in one go. Remove this when switching to CapnProto
    pub fn get_id_and_bytes(&self) -> (ArtifactId, Vec<u8>) {
        let mut hasher = Sha512Trunc256::new();

        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        hasher.input(&encoded);

        let result = hasher.result();

        (ArtifactId(result.into()), encoded)
    }
}

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent(_a) => unimplemented!(),
            Self::Url(_u) => unimplemented!(),
            Self::FlatText(t) => write!(f, "Artifact({})", t),
            Self::DataGraph(_d) => unimplemented!(),
            Self::DataNode(_n) => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Url {
    pub url: String,
}

impl Into<Artifact> for Url {
    fn into(self) -> Artifact {
        Artifact::Url(self)
    }
}

pub fn text(text: &str) -> Text {
    Text::new(text)
}

/// Text of nonspecific structure, origin, and language
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Text { text: text.to_string() }
    }

    pub fn string(text: String) -> Self {
        Text { text }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Into<Artifact> for Text {
    fn into(self) -> Artifact {
        Artifact::FlatText(self)
    }
}

impl Into<Artifact> for &str {
    fn into(self) -> Artifact {
        Artifact::FlatText(Text::new(self))
    }
}

// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataGraph {
    pub graph_type: Symbol,
    pub bytes:      u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes:      Vec<AllegationId>,
}

impl Into<Artifact> for DataGraph {
    fn into(self) -> Artifact {
        Artifact::DataGraph(self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode {
    pub data_type: Symbol,
    pub data:      Option<Vec<u8>>,
}

impl Into<Artifact> for DataNode {
    fn into(self) -> Artifact {
        Artifact::DataNode(self)
    }
}

impl Alledgable for &ArtifactId {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, MBError> {
        let allegation = Allegation::new(agent, crate::allegation::Body::Artifact(self.clone()))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
impl Alledgable for ArtifactId {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, MBError> {
        let allegation = Allegation::new(agent, crate::allegation::Body::Artifact(self))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}

impl<T> Alledgable for T where T: Into<Artifact> + std::fmt::Debug
{
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, MBError> {
        let artifact_id = mb.put_artifact(self)?;
        let allegation = Allegation::new(agent, crate::allegation::Body::Artifact(artifact_id))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNodeRelation {
    pub to:            AllegationId,
    pub relation_type: Symbol,
}
