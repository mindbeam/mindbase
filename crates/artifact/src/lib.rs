pub mod artifact;
pub mod id;

pub use mindbase_util::Error;
use serde::{Deserialize, Serialize};

pub trait NodeType: Serialize {}
pub trait NodeInstance: Clone + std::fmt::Display + std::cmp::Ord + Serialize {}

///! == Artifact subcrate
///! The intention behind this crate is to be able to represent any arbitrary datastructure
///! within the system.
///!
///! With the exception of a few convenience types (Url, FlatText) all of the raw data is
///! captured within the DataNode type. We want to ensure that unique portions of data are stored
///! only once, thus we utilize node hashing solely on the content of that node. Notably this hash
///! does NOT include the identities of nodes which originating documents might consider "children".
///! `{Relationships}` utilize a layer of indirection via `(Entities)` in order to create a unique identity for
///! each contextual usage of the `[DataNode]` in question.
///!
///! ([Dog]) -------------->   {Best friend Of}
///!   / \                         |
///!    |                          V
///! {Best friend of}  <------ ([Human])  

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ArtifactId(
    #[serde(
        serialize_with = "mindbase_util::serde_helper::as_base64",
        deserialize_with = "mindbase_util::serde_helper::from_base64_32"
    )]
    pub(crate) [u8; 32],
);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    Agent(mindbase_crypto::AgentId),
    Url(artifact::Url),
    FlatText(artifact::Text),
    Graph(artifact::SubGraph<T, E>),
    Node(artifact::DataNode<T>),
    Relation(artifact::DataRelation<T, E>),
}
