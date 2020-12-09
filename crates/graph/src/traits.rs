pub trait Artifact {
    type ID: AsRef<[u8]>;
    fn id(&self) -> Self::ID;
    fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>);
}
pub trait ArtifactInstance<A: Artifact>: Clone + std::fmt::Display + std::cmp::Ord {
    type ID: AsRef<[u8]>;

    fn instantiate(artifact: &A::ID) -> Self;
    fn id(&self) -> Self::ID;
    fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>);
    fn from_id_and_bytes<B: AsRef<u8>>(id: Self::ID, bytes: B);
}
