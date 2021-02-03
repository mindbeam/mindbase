pub mod body;
pub mod id;

use body::SubGraph;
pub use mindbase_util::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

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
pub enum Artifact<T>
where
    T: NodeType,
{
    Agent(mindbase_crypto::AgentId),
    Url(body::Url),
    FlatText(body::Text),
    Node(body::DataNode<T>),
    SubGraph(body::SubGraph<T>),
}

impl<T> Artifact<T>
where
    T: NodeType,
{
    pub fn id(&self) -> ArtifactId {
        let mut hasher = Sha512Trunc256::default();

        // TODO 5 switch to CapnProto or similar. Artifact storage and wire representation should be identical
        // Therefore we should hash that
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        hasher.update(&encoded);
        let result = hasher.finalize();
        ArtifactId(result.into())
    }
}

impl<T> mindbase_hypergraph::traits::Weight for Artifact<T>
where
    T: NodeType + Serialize + DeserializeOwned,
{
    fn get_bytes(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        encoded
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl<T> std::fmt::Display for Artifact<T>
where
    T: NodeType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Agent(_a) => unimplemented!(),
            Self::Url(_u) => unimplemented!(),
            Self::FlatText(t) => write!(f, "Artifact({})", t),
            Self::Node(_n) => unimplemented!(),
            Self::SubGraph(_s) => unimplemented!(),
        }
    }
}
