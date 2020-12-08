use serde::Serialize;
pub trait Artifact: Serialize + AsRef<[u8]> {
    type ID: ArtifactInstance;
    fn id(&self) -> Self::ID;
}
pub trait ArtifactInstance: Clone + std::fmt::Display + std::cmp::Ord + Serialize + AsRef<[u8]> {}
