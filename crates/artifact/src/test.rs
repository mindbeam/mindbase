// use crate::{Artifact, ArtifactNodeType};
// use mindbase_hypergraph::traits::{GraphInterface, Symbol, Weight};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum TestWeight<T> {
//     Artifact(Artifact<T>),
//     Type(T),
// }

// impl<T> Weight for TestWeight<T> {
//     type Symbol = T;
// }

// impl<IA, T, G> From<IA> for TestWeight<T>
// where
//     IA: Into<Artifact<T>>,
//     T: ArtifactNodeType + Symbol<G>,
//     G: GraphInterface<Artifact<T>>,
// {
//     fn from(into_artifact: IA) -> Self {
//         TestWeight::Artifact(into_artifact.into())
//     }
// }
