pub mod error;
pub mod memory;

#[cfg(not(target_arch = "wasm32"))]
pub mod sled;

use std::fmt::Debug;

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
    type OutValue: AsRef<[u8]> + std::borrow::Borrow<[u8]> + PartialEq<Vec<u8>> + std::ops::Deref<Target = [u8]> + Debug;
    type Iter: Iterator<Item = Result<(Self::OutValue, Self::OutValue), Error>>;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;
    fn set_merge_operator(&self, merge_operator: impl MergeOperator + 'static);
    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::OutValue>, Error>;
    fn iter(&self) -> Self::Iter;
    fn flush(&self) -> Result<(), Error>;
    fn clear(&self) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use crate::{MemoryStore, SledStore, Store, Tree};

    struct Tester<S: Store>(S);
    impl<S: Store> Tester<S> {
        pub fn test(&self) {
            let foo = self.0.open_tree("foo").unwrap();
            foo.insert("meow", "cat").unwrap();
            foo.insert("woof", "dog").unwrap();
            // let items: Vec<_> = foo.iter().map(|r| r.unwrap()).collect();
            // assert_eq!(items, [(b"meow", b"cat"), (b"woof", b"dog")]);
            // assert_eq!(items, &[(&b"meow"[..], &b"cat"[..]), (&b"woof"[..], &b"dog"[..])][..]);

            // assert_eq!(
            //     foo.iter()
            //         .map(|r| r.unwrap())
            //         .map(|(k, v)| (k.into(), v.into() as Vec<u8>))
            //         .collect::<Vec<(Vec<u8>, Vec<u8>)>>(),
            //     [(b"meow".to_vec(), b"cat".to_vec()), (b"woof".to_vec(), b"dog".to_vec())]
            // );
            let mut iter = foo.iter();

            // TODO 4 - fix trait bounds so we don't need to do such rediculous wrangling
            let item = iter.next().unwrap().unwrap();
            assert_eq!((&item.0 as &[u8], &item.1 as &[u8]), (&b"meow"[..], &b"cat"[..]));

            let item = iter.next().unwrap().unwrap();
            assert_eq!((&item.0 as &[u8], &item.1 as &[u8]), (&b"woof"[..], &b"dog"[..]));

            assert!(iter.next().is_none());
        }
    }

    #[test]
    fn sled() {
        let store = SledStore::open_temp().unwrap();
        let tester = Tester(store);
        tester.test()
    }

    #[test]
    fn memory() {
        let store = MemoryStore::new();
        let tester = Tester(store);
        tester.test()
    }
}
