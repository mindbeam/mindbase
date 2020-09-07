use crate::{
    agent::Agent,
    error::MBError,
    util::AsBytes,
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
    pub(crate) fn new<T>(agent: &Agent, content: T) -> Result<Self, MBError>
        where T: HashHelper
    {
        let mut hasher: Sha512 = Sha512::default();
        content.hash(&mut hasher);

        let sig = agent.keypair.sign_prehashed(hasher, Some(b"allegation"));

        Ok(Signature(sig.to_bytes()))
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

// TODO 3 - switch back to AsRef<[u8]> after CapnProto implementation
pub(crate) trait HashHelper {
    fn hash(&self, _hasher: &mut Sha512) {}
}

impl<A> HashHelper for (A,) where A: AsBytes
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.input(self.0.as_bytes());
    }
}
impl<A, B> HashHelper for (A, B)
    where A: AsBytes,
          B: AsBytes
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.input(self.0.as_bytes());
        hasher.input(self.1.as_bytes());
    }
}
impl<A, B, C> HashHelper for (A, B, C)
    where A: AsBytes,
          B: AsBytes,
          C: AsBytes
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.input(self.0.as_bytes());
        hasher.input(self.1.as_bytes());
        hasher.input(self.2.as_bytes());
    }
}
impl<A, B, C, D> HashHelper for (A, B, C, D)
    where A: AsBytes,
          B: AsBytes,
          C: AsBytes,
          D: AsBytes
{
    fn hash(&self, hasher: &mut Sha512) {
        hasher.input(self.0.as_bytes());
        hasher.input(self.1.as_bytes());
        hasher.input(self.2.as_bytes());
        hasher.input(self.3.as_bytes());
    }
}
