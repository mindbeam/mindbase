use mindbase_store::{Store, Tree};

use crate::{keys::private::AgentKey, AgentId, Error};

pub struct KeyManager<S: Store> {
    _store: S,
    agent_keys: S::Tree,
    agent_id_config: S::Tree,
}

impl<S> KeyManager<S>
where
    S: Store,
{
    pub fn new(store: S) -> Result<Self, Error> {
        Ok(Self {
            agent_keys: store.open_tree("agent_keys")?,
            agent_id_config: store.open_tree("agent_id_config")?,
            _store: store,
        })
    }
    pub fn list_agents(&self) -> Result<Vec<AgentId>, Error> {
        Ok(self
            .agent_keys
            .iter()
            .map(|kv| {
                let kv = kv.unwrap();
                let agentkey: AgentKey = bincode::deserialize(&kv.1).unwrap();
                AgentId {
                    pubkey: agentkey.pubkey(),
                }
            })
            .collect())
    }
    pub fn get_agent_key(&self, agent_id: &AgentId) -> Result<Option<AgentKey>, Error> {
        match self.agent_keys.get(&agent_id.pubkey)? {
            Some(ivec) => {
                let agentkey: AgentKey = bincode::deserialize(&ivec)?;
                Ok(Some(agentkey))
            }
            None => Ok(None),
        }
    }
    pub fn put_agent_key(&self, agentkey: AgentKey) -> Result<(), Error> {
        let key_ser: Vec<u8> = bincode::serialize(&agentkey).unwrap();

        let agent_id = agentkey.id();
        self.agent_keys.insert(&agent_id.pubkey[..], key_ser)?;
        self.agent_keys.flush()?;

        Ok(())
    }
    pub fn remove_all_agent_keys(&self) -> Result<(), Error> {
        self.agent_keys.clear()?;
        Ok(())
    }

    pub fn set_current_agent(&self, id: AgentId) -> Result<(), Error> {
        let id_ser: Vec<u8> = bincode::serialize(&id).unwrap();
        self.agent_id_config.insert("current", id_ser)?;
        Ok(())
    }
    pub fn current_agent_key(&self) -> Result<Option<AgentKey>, Error> {
        match self.agent_id_config.get("current")? {
            Some(value) => {
                let id: AgentId = bincode::deserialize(&value[..])?;

                match self.get_agent_key(&id)? {
                    Some(a) => Ok(Some(a)),
                    None => Err(Error::InvalidReferent),
                }
            }
            None => Ok(None),
        }
    }
}

impl<T> Default for KeyManager<T>
where
    T: Store + Default,
{
    fn default() -> Self {
        KeyManager::new(Default::default())
            .expect("should not fail for store which implements default")
    }
}

#[cfg(test)]
mod test {
    use mindbase_store::MemoryStore;

    use crate::{AgentKey, KeyManager};

    #[test]
    fn init() -> Result<(), std::io::Error> {
        let keymanager = KeyManager::new(MemoryStore::new())?;

        let agentkey = AgentKey::create(None);
        let id = agentkey.id();
        keymanager.put_agent_key(agentkey)?;
        keymanager.set_current_agent(id.clone())?;

        assert_eq!(
            keymanager
                .current_agent_key()
                .expect("is good")
                .expect("is some")
                .id(),
            id
        );

        Ok(())
    }
}
