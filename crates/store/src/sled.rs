use std::collections::BTreeMap;

use sled::IVec;

use crate::{Error, Store, Tree};
pub struct SledStore {
    db: sled::Db,
}
pub struct SledStoreTree(sled::Tree);

impl Store for SledStore {
    type Tree = SledStoreTree;
    fn open_tree<V: AsRef<[u8]>>(&self, name: V) -> Result<Self::Tree, Error> {
        Ok(SledStoreTree(self.db.open_tree(name)?))
    }
}

impl Tree for SledStoreTree {
    type Value = sled::IVec;

    fn insert<K: AsRef<[u8]> + Into<Vec<u8>>>(&self, key: K, value: Vec<u8>) -> Result<(), Error> {
        self.0.insert(key, value)?;
        Ok(())
    }

    fn set_merge_operator(&self, merge_operator: impl crate::MergeOperator + 'static) {
        self.0.set_merge_operator(merge_operator);
    }

    fn merge<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error> {
        self.0.merge(key, value)?;
        Ok(())
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Self::Value>, Error> {
        Ok(self.0.get(key)?)
    }

    fn flush(&self) -> Result<(), Error> {
        self.0.flush()?;
        Ok(())
    }

    fn clear(&self) -> Result<(), Error> {
        self.0.clear()?;
        Ok(())
    }
}

impl SledStore {
    pub fn open_temp() -> Result<Self, Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();

        Self::open(tmpdirpath)
    }

    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        Ok(SledStore { db })
    }
}
