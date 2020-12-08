use mindbase_crypto::AgentId;
use mindbase_graph::traits::{NodeInstance, NodeType};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};
use std::fmt;

use crate::{Artifact, ArtifactId};

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

impl<T, E> Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
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

impl<T, E> std::fmt::Display for Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent(_a) => unimplemented!(),
            Self::Url(_u) => unimplemented!(),
            Self::FlatText(t) => write!(f, "Artifact({})", t),
            Self::Graph(_d) => unimplemented!(),
            Self::Node(_n) => unimplemented!(),
            Artifact::Relation(_r) => unimplemented!(),
        }
    }
}

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
