use crate::{Artifact, NodeType};
use mindbase_hypergraph::traits::Weight;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TestWeight<T> {
    Artifact(Artifact<T>),
    Type(T),
}
impl<T> Weight for TestWeight<T> where T: NodeType {}

impl<A, T> From<A> for TestWeight<T>
where
    A: Into<Artifact<T>>,
    T: NodeType,
{
    fn from(artifact: A) -> Self {
        TestWeight::Artifact(artifact.into())
    }
}

// impl<T> From<T> for TestWeight<T>
// where
//     T: Serialize + DeserializeOwned,
// {
//     fn from(t: T) -> Self {
//         TestWeight::Type(t)
//     }
// }
