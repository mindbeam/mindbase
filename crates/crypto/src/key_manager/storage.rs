TODO - delete this file in favor of mindbase_store

use crate::{keys::AgentId, AgentKey, Error};

pub trait StorageAdapter {
    fn list_agents(&self) -> Result<Vec<AgentId>, Error>;
    fn get_agent_key(&self, agent_id: &AgentId) -> Result<Option<AgentKey>, Error>;
    fn put_agent_key(&self, key: AgentKey) -> Result<(), Error>;
    fn set_labeled_agent(&self, label: &str, id: AgentId) -> Result<(), Error>;
    fn get_labeled_agent(&self, label: &str) -> Result<Option<AgentId>, Error>;
    fn remove_all_agent_keys(&self) -> Result<(), Error>;
}

pub mod memory {
    use std::{collections::HashMap, sync::Mutex};

    use super::StorageAdapter;
    use crate::{AgentId, AgentKey, Error};
    pub struct MemoryAdapter {
        agent_keys: Mutex<HashMap<[u8; 32], AgentKey>>,
        agent_id_config: Mutex<HashMap<Vec<u8>, AgentId>>,
    }
    impl MemoryAdapter {
        pub fn new() -> Self {
            Self {
                agent_keys: Default::default(),
                agent_id_config: Default::default(),
            }
        }
    }

    impl StorageAdapter for MemoryAdapter {
        fn list_agents(&self) -> Result<Vec<AgentId>, Error> {
            Ok(self.agent_keys.lock().unwrap().values().map(|k| k.id().clone()).collect())
        }
        fn get_agent_key(&self, agent_id: &AgentId) -> Result<Option<crate::AgentKey>, Error> {
            // Proba
        }

        fn put_agent_key(&self, agent_key: crate::AgentKey) -> Result<(), Error> {}
        fn set_labeled_agent(&self, label: &str, id: AgentId) -> Result<(), Error> {
            self.agent_id_config.lock().unwrap().insert(label.as_bytes().to_owned(), id);
            Ok(())
        }
        fn get_labeled_agent(&self, label: &str) -> Result<Option<AgentId>, Error> {
            match self.agent_id_config.lock().unwrap().get(label.as_bytes()) {
                Some(id) => Ok(Some(id.clone())),
                None => Ok(None),
            }
        }

        fn remove_all_agent_keys(&self) -> Result<(), Error> {
            // IN THEORY ed25519_dalek should be doing the right thing here with zeroization
            self.agent_keys.lock().unwrap().clear();
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod sled {
    use std::convert::TryInto;

    use super::StorageAdapter;
    use crate::{AgentId, AgentKey, Error};
    pub struct SledAdapter {
        agent_keys: sled::Tree,
        agent_id_config: sled::Tree,
    }

    impl SledAdapter {
        pub fn open(basedir: &std::path::Path) -> Result<Box<dyn StorageAdapter>, Error> {
            let pathbuf = basedir.join(format!("./mindbase_key_manager.sled"));
            let db = sled::open(pathbuf.as_path())?;

            Ok(Box::new(SledAdapter {
                agent_keys: db.open_tree("agent_keys")?,
                agent_id_config: db.open_tree("agent_id_config")?,
            }))
        }
    }

    impl StorageAdapter for SledAdapter {
        fn list_agents(&self) -> Result<Vec<AgentId>, Error> {}
        fn get_agent_key(&self, agent_id: &AgentId) -> Result<Option<crate::AgentKey>, Error> {}

        fn put_agent_key(&self, agent_key: crate::AgentKey) -> Result<(), Error> {}
        fn set_labeled_agent(&self, label: &str, id: AgentId) -> Result<(), Error> {}
        fn get_labeled_agent(&self, label: &str) -> Result<Option<AgentId>, Error> {}
        fn remove_all_agent_keys(&self) -> Result<(), Error> {}
    }
}
