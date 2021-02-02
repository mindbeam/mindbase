pub trait Weight {
    fn get_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}
pub trait Provenance {}
// pub trait Entity<A: Weight>: Clone + std::fmt::Display + std::cmp::Ord {
//     type ID: AsRef<[u8]> + Ord;

//     fn id(&self) -> Self::ID;
//     fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>);
//     fn from_id_and_bytes<B: AsRef<[u8]>>(id: Self::ID, bytes: B) -> Self;
// }
