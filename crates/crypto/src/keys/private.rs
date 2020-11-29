use crate::AgentId;

use super::{
    custodian::{CustodialAgentKey, KeyMask, UserAuthKey},
    AgentIdentity,
};
use ed25519_dalek::Keypair;
use hmac::{Hmac, Mac, NewMac};
use rand::rngs::OsRng;
use scrypt::{scrypt, ScryptParams};
use serde::{Deserialize, Serialize};
use sha2::Sha512Trunc256;
use zeroize::Zeroize;

// dalek::Keypair already derives Zeroize
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentKey {
    pub keypair: Keypair,
    pub email: Option<String>,
}

// impl Eq for AgentKey{
//     fn assert_receiver_is_total_eq(&self) {}
// }
impl PartialEq for AgentKey {
    fn eq(&self, other: &Self) -> bool {
        self.keypair.secret.as_bytes().eq(other.keypair.secret.as_bytes())
            && self.keypair.public.as_bytes().eq(other.keypair.public.as_bytes())
    }
}

impl AgentKey {
    pub fn create(email: Option<String>) -> AgentKey {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);

        // TODO 2 - Reconcile differences between the above
        // and the old code removed from minbase core (below)
        // let mut csprng: OsRng = OsRng::new().unwrap();
        // let keypair: Keypair = Keypair::generate::<Sha512>(&mut csprng);

        AgentKey { keypair, email }
    }
    pub fn hmac(&self) -> [u8; 32] {
        let mut mac = Hmac::<Sha512Trunc256>::new_varkey(b"agentkey").unwrap();
        mac.update(self.keypair.secret.as_bytes());
        mac.update(self.keypair.public.as_bytes());
        let result = mac.finalize();

        result.into_bytes().into()
    }
    pub fn id(&self) -> AgentId {
        let pubkey = self.keypair.public.as_bytes().clone();
        AgentId { pubkey }
    }
    pub fn identity(&self) -> AgentIdentity {
        let pubkey = self.keypair.public.as_bytes().clone();
        AgentIdentity {
            pubkey,
            email: self.email.clone(),
        }
    }
    pub fn pubkey(&self) -> [u8; 32] {
        self.keypair.public.as_bytes().clone()
    }
    pub fn keymask(&self, passkey: &PassKey) -> KeyMask {
        // use std::convert::TryInto;

        let mut mask = [0u8; 32];

        self.keypair
            .secret
            .as_bytes()
            .iter()
            .zip(passkey.c.iter())
            .enumerate()
            .for_each(|(i, (bk, bc))| mask[i] = bk ^ bc);

        KeyMask { mask }
    }
    pub fn custodial_key(&self, passkey: PassKey) -> CustodialAgentKey {
        // Consume the PassKey to discourage the implementer from storing it

        CustodialAgentKey {
            pubkey: self.keypair.public.as_bytes().clone(),
            mask: self.keymask(&passkey),
            check: self.hmac(),
            email: self.email.clone(),
        }
    }
    pub fn from_custodial_key(custodial_key: CustodialAgentKey, passkey: PassKey) -> Result<Self, crate::Error> {
        // Consume the PassKey to discourage the implementer from storing it

        let mut secret = [0u8; 32];

        custodial_key
            .mask
            .as_bytes()
            .iter()
            .zip(passkey.c.iter())
            .enumerate()
            .for_each(|(i, (m, p))| secret[i] = m ^ p);

        let mut mac = Hmac::<Sha512Trunc256>::new_varkey(b"agentkey").unwrap();
        mac.update(&secret);
        mac.update(&custodial_key.pubkey);

        mac.verify(&custodial_key.check)?;

        Ok(Self {
            keypair: Keypair {
                secret: ed25519_dalek::SecretKey::from_bytes(&secret)?,
                public: ed25519_dalek::PublicKey::from_bytes(&custodial_key.pubkey)?,
            },
            email: custodial_key.email.clone(),
        })
    }
}

// DO NOT ALLOW SERIALIZATION
#[derive(Zeroize)]
#[zeroize(drop)]
pub struct PassKey {
    c: [u8; 32],
}

impl PassKey {
    pub fn new(passphrase: &str) -> PassKey {
        let salt = b"mindbase passkey";
        let params = ScryptParams::recommended();
        let mut dk = [0u8; 32];
        scrypt(passphrase.as_bytes(), salt, &params, &mut dk).expect("32 bytes always satisfy output length requirements");

        PassKey { c: dk }
    }
    // Use this to authenticate with the server
    pub fn auth(&self) -> UserAuthKey {
        let salt = b"mindbase authkey";
        let params = ScryptParams::recommended();

        let mut auth = [0u8; 32];
        scrypt(&self.c, salt, &params, &mut auth).expect("32 bytes always satisfy output length requirements");

        UserAuthKey { auth }
    }
}
