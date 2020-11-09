pub mod storage;

// use web_sys::{window, Storage};
use crate::{keys::private::AgentKey, keys::AgentIdentity, Error, PassKey};

use self::storage::{memory::MemoryAdapter, StorageAdapter};

pub struct KeyManager {
    storage_adapter: Box<dyn StorageAdapter>,
}

impl KeyManager {
    pub fn new(storage_adapter: Box<dyn StorageAdapter>) -> Self {
        Self { storage_adapter }
    }
    pub fn list_agents(&self) -> Result<Vec<AgentIdentity>, Error> {
        self.storage_adapter.list_agents()
    }
    pub fn get_agent_key(&self, agent_id: &AgentIdentity) -> Result<Option<AgentKey>, Error> {
        self.storage_adapter.get_agent_key(agent_id)
    }
    pub fn put_agent_key(&self, agentkey: AgentKey) -> Result<(), Error> {
        self.storage_adapter.put_agent_key(agentkey)?;
        Ok(())
    }
    pub fn remove_all_agent_keys(&self) -> Result<(), Error> {
        self.storage_adapter.remove_all_agent_keys()?;
        Ok(())
    }

    pub fn set_current_agent(&self, id: AgentIdentity) -> Result<(), Error> {
        self.storage_adapter.set_labeled_agent("current", id)?;
        Ok(())
    }
    pub fn current_agent_key(&self) -> Result<Option<AgentKey>, Error> {
        match self.storage_adapter.get_labeled_agent("current")? {
            Some(agent_id) => self.storage_adapter.get_agent_key(&agent_id),
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
