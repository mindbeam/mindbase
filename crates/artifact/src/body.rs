use mindbase_crypto::AgentId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};
use std::fmt::{self, Display};

use crate::{Artifact, ArtifactId, NodeInstance, NodeType};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Url {
    pub url: String,
}

/// Text of nonspecific structure, origin, and language
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Text {
    text: String,
}

// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubGraph<T>
where
    T: NodeType,
{
    pub graph_type: T,
}

impl<T> Display for SubGraph<T>
where
    T: NodeType + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataNode({:?})", self.graph_type,)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode<T>
where
    T: NodeType,
{
    pub data_type: T,
    pub data: Option<Vec<u8>>,
}
impl<T> Display for DataNode<T>
where
    T: NodeType + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DataNode({:?}:{})",
            self.data_type,
            String::from_utf8_lossy(self.data.as_ref().map_or(b"", |d| &d[..]))
        )
    }
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

impl<T> Into<Artifact<T>> for DataNode<T>
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::Node(self)
    }
}
impl<T> Into<Artifact<T>> for SubGraph<T>
where
    T: NodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::SubGraph(self)
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
