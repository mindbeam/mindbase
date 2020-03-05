use crate::{
    agent::Agent,
    error::Error,
};
use serde::{
    Deserialize,
    Serialize,
};
use sha2::{
    Digest,
    Sha512,
};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Signature(#[serde(serialize_with = "crate::util::array64::ser_as_base64",
                             deserialize_with = "crate::util::array64::de_from_base64")]
                     pub(crate) [u8; 64]);

impl Signature {
    pub fn new<T>(agent: &Agent, content: T) -> Result<Self, Error>
        where T: HashHelper
    {
        match agent.keypair() {
            None => Err(Error::SignatureError),
            Some(keypair) => {
                let hasher: Sha512 = Sha512::default();
                content.hash(&hasher);

                let sig = keypair.sign_prehashed(hasher, Some(b"allegation"));
                Ok(Signature(sig.to_bytes()))
            },
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0[..], STANDARD_NO_PAD))
    }
}
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArtifactId:{}", base64::encode(&self.0[..]))
    }
}

trait HashHelper {
    fn hash(&self, hasher: &Sha512) {}
}

impl<T> HashHelper for (T,) where T: AsRef<[u8]>
{
    fn hash(&self, hasher: &Sha512) {
        hasher.input(self.0.as_ref());
    }
}
impl<T> HashHelper for (T, T) where T: AsRef<[u8]>
{
    fn hash(&self, hasher: &Sha512) {
        hasher.input(self.0.as_ref());
        hasher.input(self.1.as_ref());
    }
}
impl<T> HashHelper for (T, T, T) where T: AsRef<[u8]>
{
    fn hash(&self, hasher: &Sha512) {
        hasher.input(self.0.as_ref());
        hasher.input(self.1.as_ref());
        hasher.input(self.2.as_ref());
    }
}
impl<T> HashHelper for (T, T, T, T) where T: AsRef<[u8]>
{
    fn hash(&self, hasher: &Sha512) {
        hasher.input(self.0.as_ref());
        hasher.input(self.1.as_ref());
        hasher.input(self.2.as_ref());
        hasher.input(self.3.as_ref());
    }
}
