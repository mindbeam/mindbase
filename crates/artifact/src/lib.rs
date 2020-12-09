pub mod body;
pub mod id;

pub use mindbase_util::Error;
use serde::{Deserialize, Serialize};
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
pub enum Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    Agent(mindbase_crypto::AgentId),
    Url(body::Url),
    FlatText(body::Text),
    Graph(body::SubGraph<T, E>),
    Node(body::DataNode<T>),
    Relation(body::DataRelation<T, E>),
}

impl<T, E> Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
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

impl<T, E> mindbase_graph::traits::Artifact for Artifact<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    type ID = ArtifactId;

    fn id(&self) -> Self::ID {
        self.id()
    }
    /// Might as well Serialize and hash in one go. Remove this when switching to CapnProto
    fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>) {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
