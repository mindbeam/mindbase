// use web_sys::{window, Storage};
use crate::keys::private::AgentKey;

pub struct KeyManager {
    namespace: &'static str,
    agentkeys: Vec<AgentKey>,
}
impl KeyManager {
    pub fn new(namespace: &'static str) -> Self {
        let agentkeys = Vec::new();

        Self {
            namespace,
            agentkeys,
        }
    }

    pub fn get_key(&mut self) {}
    pub fn set_key(&self) {}
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
