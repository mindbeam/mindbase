use crate::{Artifact, ArtifactNodeType};
use mindbase_hypergraph::traits::{Symbol, Weight};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TestWeight<T>
where
    T: Symbol,
{
    Artifact(Artifact<T>),
    Type(T),
}

impl<T> Weight for TestWeight<T>
where
    T: ArtifactNodeType + Symbol,
{
    type Symbol = T;
}

impl<IA, T> From<IA> for TestWeight<T>
where
    IA: Into<Artifact<T>>,
    T: ArtifactNodeType + Symbol,
{
    fn from(into_artifact: IA) -> Self {
        TestWeight::Artifact(into_artifact.into())
    }
}
