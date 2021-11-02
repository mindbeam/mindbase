pub mod body;
pub mod id;
pub mod test;

use chrono::{DateTime, Utc};
pub use mindbase_util::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

pub trait ArtifactNodeType: Serialize + DeserializeOwned {}

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
pub enum Artifact {
    Agent(keyplace::AgentId),
    String(String),
    Date(DateTime<Utc>),
    Uint32(u32),
    // Struct()
    Json(Vec<u8>),
    Bytes(Vec<u8>),
}

// impl Artifact
// where
//     T: ArtifactNodeType,
// {
//     pub fn id(&self) -> ArtifactId {
//         let mut hasher = Sha512Trunc256::default();

//         // TODO 5 switch to CapnProto or similar. Artifact storage and wire representation should be identical
//         // Therefore we should hash that
//         let encoded: Vec<u8> = bincode::serialize(self).unwrap();
//         hasher.update(&encoded);
//         let result = hasher.finalize();
//         ArtifactId(result.into())
//     }
// }

// impl<T> mindbase_hypergraph::traits::Value for Artifact
// where
//     T: ArtifactNodeType + std::fmt::Debug,
// {
// type Symbol = T;
// fn compare_sym<G, W>(&self, symbol: Self::Symbol, graph: &G) -> Result<f64, Error> {
//     match self {
//         Artifact::Agent(_) => unimplemented!(),
//         Artifact::Url(_) => unimplemented!(),
//         Artifact::FlatText(_) => unimplemented!(),
//         Artifact::Node(DataType) => {}
//         Artifact::Type(_) => {}
//     }
// }
// }

impl<T> std::fmt::Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Artifact::Agent(a) => todo!(),
            Artifact::String(s) => todo!(),
            Artifact::Date(d) => todo!(),
            Artifact::Uint32(v) => todo!(),
            Artifact::Json(j) => todo!(),
            Artifact::Bytes(b) => todo!(),
            // Self::Agent(_a) => unimplemented!(),
            // Self::Url(_u) => unimplemented!(),
            // Self::FlatText(t) => write!(f, "Artifact::FlatText({})", t),
            // Self::Node(n) => write!(f, "Artifact::Node({})", n),
            // Self::Type(s) => write!(f, "Artifact::Type({:?})", s),
        }
    }
}
