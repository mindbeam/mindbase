use mindbase_store::Store;

use crate::{keys::private::AgentKey, AgentId, Error, PassKey};

pub struct KeyManager<S: Store> {
    store: S,
    agent_keys: S::Tree,
    agent_id_config: S::Tree,
}

impl<S> KeyManager<S>
where
    S: Store,
{
    pub fn new(store: S) -> Result<Self, Error> {
        Ok(Self {
            store,
            agent_keys: store.open_tree("agent_keys")?,
            agent_id_config: store.open_tree("agent_id_config")?,
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
        let key_ser: Vec<u8> = bincode::serialize(&agent_key).unwrap();

        let agent_id = agent_key.id();
        self.agent_keys.insert(&agent_id.pubkey, key_ser)?;
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
            Some(ivec) => Ok(Some(bincode::deserialize(&ivec)?)),
            None => Ok(None),
        }
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        KeyManager::new(Box::new(MemoryAdapter::new()))
    }
}

#[cfg(test)]
mod test {
    // use super::{keypair, KeyMask, PassKey};

    // #[test]
    // fn basic_keys() {
    //     let secret = keypair();
    //     let passkey = PassKey::new("My dog spot");
    //     let mask = KeyMask::new(&secret, &passkey);

    //     // The SecretKey is obviously not safe to send to the server, nor is the passkey.
    //     // But the keymask IS safe to send to the server, because the server cannot extract
    //     // the secret key from the KeyMask without the passkey.

    //     // This gives the user the ability to recover the secret key with the help of the
    //     // server without the server having the ability to read it, and thus compromise the
    //     // user's privacy

    //     println!("The mask is {}", mask.base64());

    //     let passkey2 = mask.reveal(&passkey);

    //     assert_eq!(secret)
    // }
}
