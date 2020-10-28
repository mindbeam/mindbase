// use web_sys::{window, Storage};
use crate::{keys::private::AgentKey, PassKey};

pub struct KeyManager {
    agentkeys: Vec<AgentKey>,
}

impl KeyManager {
    pub fn new() -> Self {
        let agentkeys = Vec::new();
        Self { agentkeys }
    }

    // pub fn get_key(&self) {}
    pub fn insert_key(&mut self, agentkey: AgentKey) -> bool {
        match self
            .agentkeys
            .binary_search_by(|x| x.pubkey().cmp(&agentkey.pubkey()))
        {
            Ok(_) => false,
            Err(i) => {
                self.agentkeys.insert(i, agentkey);
                true
            }
        }
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        KeyManager::new()
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
