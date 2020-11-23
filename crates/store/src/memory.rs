use crate::{Error, MergeOperator, Store, Tree};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

#[derive(Default, Clone)]
pub struct MemoryStore {
    trees: Arc<Mutex<BTreeMap<Vec<u8>, MemoryStoreTree>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Clone)]
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
        let mut trees = self.trees.lock().unwrap();
        match trees.entry(name.as_ref().to_owned()) {
            std::collections::btree_map::Entry::Vacant(v) => Ok(v.insert(Default::default()).clone()),
            std::collections::btree_map::Entry::Occupied(o) => Ok(o.get().clone()),
        }
    }
}

impl Tree for MemoryStoreTree {
    type OutValue = Vec<u8>;
    type Iter = StupidIterator<(Self::OutValue, Self::OutValue)>;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), crate::Error> {
        self.inner.lock().unwrap().btree.insert(key.into(), value.as_ref().into());
        Ok(())
    }

    fn set_merge_operator(&self, merge_operator: impl crate::MergeOperator + 'static) {
        self.inner.lock().unwrap().merge_operator = Some(Box::new(merge_operator));
    }

    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), crate::Error> {
        let mut inner = self.inner.lock().unwrap();
        match inner.merge_operator {
            Some(op) => {}
            None => {}
        }
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::OutValue>, crate::Error> {
        todo!()
    }

    fn flush(&self) -> Result<(), crate::Error> {
        todo!()
    }

    fn clear(&self) -> Result<(), crate::Error> {
        todo!()
    }

    fn iter(&self) -> StupidIterator<(Self::OutValue, Self::OutValue)> {
        let inner = self.inner.lock().unwrap();
        StupidIterator {
            list: inner.btree.iter().map(|v| (v.0.to_owned(), v.1.to_owned())).collect(),
            offset: 0,
        }
    }
}

pub struct StupidIterator<T> {
    offset: usize,
    list: Vec<T>,
}

impl<T> Iterator for StupidIterator<T>
where
    T: Clone,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.get(self.offset) {
            Some(v) => {
                self.offset += 1;
                Some(Ok(v.clone()))
            }
            None => None,
        }
    }
}
