use keyplace::AgentId;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

use crate::{Artifact, ArtifactNodeType};

/// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
/// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Type<T>(pub T);

impl<T> Display for Type<T>
where
    T: ArtifactNodeType + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type({:?})", self.0,)
    }
}

// impl<T> Into<Artifact> for Type<T>
// where
//     T: ArtifactNodeType,
// {
//     fn into(self) -> Artifact {
//         Artifact::Type(self)
//     }
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Url {
    pub url: String,
}

/// Text of nonspecific structure, origin, and language
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Text {
    text: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct DataNode<T> {
    pub data_type: T,
    pub data: Option<Vec<u8>>,
}

// TODO - determine if the datanode should understand the serialization scheme
// and output types, or if that should be externalized to the user

impl<T> Display for DataNode<T>
where
    T: std::fmt::Debug,
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
impl<T> Debug for DataNode<T>
where
    T: std::fmt::Debug,
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

// impl<T> Into<Artifact<T>> for Url
// where
//     T: ArtifactNodeType,
// {
//     fn into(self) -> Artifact<T> {
//         Artifact::Url(self)
//     }
// }

// pub fn text(text: &str) -> Text {
//     Text::new(text)
// }

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
    T: ArtifactNodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::FlatText(self)
    }
}

impl<T> Into<Artifact<T>> for &str
where
    T: ArtifactNodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::FlatText(Text::new(self))
    }
}

impl<T> Into<Artifact<T>> for DataNode<T>
where
    T: ArtifactNodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::Node(self)
    }
}

impl<T> Into<Artifact<T>> for AgentId
where
    T: ArtifactNodeType,
{
    fn into(self) -> Artifact<T> {
        Artifact::Agent(self)
    }
}
