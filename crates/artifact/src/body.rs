use mindbase_crypto::AgentId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};
use std::fmt;

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
pub struct SubGraph<T, I>
where
    T: NodeType,
    I: NodeInstance,
{
    pub graph_type: T,
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes: Vec<I>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode<T>
where
    T: NodeType,
{
    pub data_type: T,
    pub data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataRelation<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    pub relation_type: T,
    pub from: E,
    pub to: E,
    //  pub amendment: RelationAmendment
}

// pub enum RelationAmendment{
//     Remove
// }

impl<T, E> Into<Artifact<T, E>> for Url
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
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

impl<T, E> Into<Artifact<T, E>> for Text
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::FlatText(self)
    }
}

impl<T, E> Into<Artifact<T, E>> for &str
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::FlatText(Text::new(self))
    }
}

impl<T, E> Into<Artifact<T, E>> for SubGraph<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::Graph(self)
    }
}

impl<T, E> Into<Artifact<T, E>> for DataNode<T>
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::Node(self)
    }
}
impl<T, E> Into<Artifact<T, E>> for DataRelation<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::Relation(self)
    }
}

impl<T, E> Into<Artifact<T, E>> for AgentId
where
    T: NodeType,
    E: NodeInstance,
{
    fn into(self) -> Artifact<T, E> {
        Artifact::Agent(self)
    }
}
