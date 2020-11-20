pub mod error;
pub mod memory;

#[cfg(not(target_arch = "wasm32"))]
pub mod sled;

pub use self::sled::SledStore;
pub use error::Error;
pub use memory::MemoryStore;

pub trait Store {
    type Tree: self::Tree;
    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, Error>;
}
pub trait MergeOperator: Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {}
impl<F> MergeOperator for F where F: Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {}

pub trait Tree {
    type Value: AsRef<[u8]>;
    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>>(&self, key: K, value: Vec<u8>) -> Result<(), Error>;
    fn set_merge_operator(&self, merge_operator: impl MergeOperator + 'static);
    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::Value>, Error>;
    fn flush(&self) -> Result<(), Error>;
    fn clear(&self) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
