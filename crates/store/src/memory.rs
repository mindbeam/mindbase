use crate::{MergeOperator, Store, Tree};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub struct MemoryStore {
    trees: BTreeMap<Vec<u8>, Arc<MemoryStoreTree>>,
}

#[derive(Default)]
pub struct MemoryStoreTree {
    inner: Arc<Mutex<TreeInner>>,
}

#[derive(Default)]
struct TreeInner {
    btree: BTreeMap<Vec<u8>, Vec<u8>>,
    merge_operator: Option<Box<dyn MergeOperator>>,
}

impl Store for MemoryStore {
    type Tree = MemoryStoreTree;

    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, crate::Error> {
        Ok(Default::default())
    }
}

impl Tree for MemoryStoreTree {
    type Value = Vec<u8>;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>>(&self, key: K, value: Vec<u8>) -> Result<(), crate::Error> {
        self.inner.lock().unwrap().btree.insert(key.into(), value);
        Ok(())
    }

    fn set_merge_operator(&self, merge_operator: impl crate::MergeOperator + 'static) {
        todo!()
    }

    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), crate::Error> {
        todo!()
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::Value>, crate::Error> {
        todo!()
    }

    fn flush(&self) -> Result<(), crate::Error> {
        todo!()
    }

    fn clear(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
