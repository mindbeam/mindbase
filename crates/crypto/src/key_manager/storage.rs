use crate::{keys::AgentIdentity, AgentKey, Error};

pub trait StorageAdapter {
    fn get_agent_key(&self, agent_id: &AgentIdentity) -> Result<Option<AgentKey>, Error>;
    fn put_agent_key(&self, key: AgentKey) -> Result<(), Error>;
    fn get_labeled_agent_id(&self, label: &str) -> Result<Option<AgentIdentity>, Error>;
    fn remove_all_agent_keys(&self) -> Result<(), Error>;
}

pub mod memory {
    use std::{collections::HashMap, sync::Mutex};
    use zeroize::Zeroizing;

    use super::StorageAdapter;
    use crate::{keys::AgentIdentity, AgentKey, Error};
    pub struct MemoryAdapter {
        agent_keys: Mutex<HashMap<Vec<u8>, AgentKey>>,
        agent_id_config: Mutex<HashMap<Vec<u8>, AgentIdentity>>,
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
        fn get_agent_key(&self, agent_id: &AgentIdentity) -> Result<Option<crate::AgentKey>, Error> {
            // Proba
            let pubkey = &agent_id.pubkey;
            match self.agent_keys.lock().unwrap().get(&pubkey[..]) {
                Some(v) => {
                    // TODO 2 - This is NOT secure, as the buffer will be freed without being
                    let danger_non_securely_deleted_buffer = v.keypair.to_bytes();

                    Ok(Some(AgentKey {
                        // Doesn't implement clone, so we have to fake it
                        keypair: ed25519_dalek::Keypair::from_bytes(&danger_non_securely_deleted_buffer).unwrap(),
                    }))
                }
                None => Ok(None),
            }
        }

        fn put_agent_key(&self, agent_key: crate::AgentKey) -> Result<(), Error> {
            let pubkey = agent_key.pubkey();

            self.agent_keys.lock().unwrap().insert(pubkey.to_vec(), agent_key);

            // TODO - implement put_labeled_agent_id
            self.agent_id_config
                .lock()
                .unwrap()
                .insert(b"latest".to_vec(), AgentIdentity { pubkey });

            Ok(())
        }

        fn get_labeled_agent_id(&self, label: &str) -> Result<Option<AgentIdentity>, Error> {
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
    use crate::{keys::AgentIdentity, AgentKey, Error};
    pub struct SledAdapter {
        agent_keys: sled::Tree,
        agent_id_config: sled::Tree,
    }

    impl SledAdapter {
        pub fn new(basedir: &std::path::Path) -> Result<Self, Error> {
            let pathbuf = basedir.join(format!("./mindbase_key_manager.sled"));
            let db = sled::open(pathbuf.as_path())?;

            Ok(SledAdapter {
                agent_keys: db.open_tree("agent_keys")?,
                agent_id_config: db.open_tree("agent_id_config")?,
            })
        }
    }

    impl StorageAdapter for SledAdapter {
        fn get_agent_key(&self, agent_id: &AgentIdentity) -> Result<Option<crate::AgentKey>, Error> {
            match self.agent_keys.get(&agent_id.pubkey)? {
                Some(ivec) => {
                    let agentkey: AgentKey = bincode::deserialize(&ivec)?;
                    Ok(Some(agentkey))
                }
                None => Ok(None),
            }
        }

        fn put_agent_key(&self, agent_key: crate::AgentKey) -> Result<(), Error> {
            let encoded: Vec<u8> = bincode::serialize(&agent_key).unwrap();

            let agent_id = agent_key.pubkey();
            self.agent_keys.insert(&agent_id, encoded)?;
            self.agent_id_config.insert(b"latest", &agent_id)?;
            self.agent_keys.flush()?;

            Ok(())
        }
        fn get_labeled_agent_id(&self, label: &str) -> Result<Option<AgentIdentity>, Error> {
            match self.agent_id_config.get(label.as_bytes())? {
                Some(pubkey) => Ok(Some(AgentIdentity {
                    pubkey: pubkey[0..32].try_into().unwrap(),
                })),
                None => Ok(None),
            }
        }
        fn remove_all_agent_keys(&self) -> Result<(), Error> {
            self.agent_keys.clear()?;
            Ok(())
        }

        // fn _default_agent(my_agents: &sled::Tree) -> Result<Agent, MBError> {
        //     match my_agents.get(b"latest")? {
        //         None => _create_agent(my_agents),
        //         Some(pubkey) => match my_agents.get(pubkey)? {
        //             None => Err(MBError::AgentHandleNotFound),
        //             Some(v) => {
        //                 let agenthandle = bincode::deserialize(&v)?;
        //                 Ok(agenthandle)
        //             }
        //         },
        //     }
        // }
    }
}
