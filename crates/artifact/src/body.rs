use mindbase_crypto::AgentId;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{ArtifactId, NodeType};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Artifact<T>
where
    T: NodeType,
{
    Agent(AgentId),
    Url(Url),
    FlatText(Text),
    DataGraph(DataGraph<T>),
    DataNode(DataNode<T>),
}

use sha2::{Digest, Sha512Trunc256};
impl<T> Artifact<T>
where
    T: NodeType,
{
    pub fn id(&self) -> ArtifactId {
        let mut hasher = Sha512Trunc256::new();

        // TODO 5 switch to CapnProto or similar. Artifact storage and wire representation should be identical
        // Therefore we should hash that
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        hasher.update(&encoded);
        let result = hasher.finalize();
        ArtifactId(result.into())
    }

    /// Might as well Serialize and hash in one go. Remove this when switching to CapnProto
    pub fn get_id_and_bytes(&self) -> (ArtifactId, Vec<u8>) {
        let mut hasher = Sha512Trunc256::new();

        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        hasher.update(&encoded);

        let result = hasher.finalize();

        (ArtifactId(result.into()), encoded)
    }
}

impl<T> std::fmt::Display for Artifact<T>
where
    T: NodeType,
{
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

impl<T> Into<Artifact<T>> for Url
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
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

impl<T> Into<Artifact<T>> for Text
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::FlatText(self)
    }
}

impl<T> Into<Artifact<T>> for &str
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::FlatText(Text::new(self))
    }
}

// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataGraph<T> {
    pub graph_type: T,
    pub bytes: u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes: Vec<ArtifactId>,
}

impl<T> Into<Artifact<T>> for DataGraph<T>
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::DataGraph(self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode<T> {
    pub data_type: T,
    pub data: Option<Vec<u8>>,
}

impl<T> Into<Artifact<T>> for DataNode<T>
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::DataNode(self)
    }
}

impl<T> Into<Artifact<T>> for AgentId
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::Agent(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataNodeRelation<T> {
    pub to: ArtifactId,
    pub relation_type: T,
}
