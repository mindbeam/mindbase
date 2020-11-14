use mindbase_crypto::AgentId;
use mindbase_util::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::symbol::Symbol;
use sha2::{Digest, Sha512Trunc256};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactId(#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")] pub(crate) [u8; 32]);

pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    use base64::STANDARD_NO_PAD;
    serializer.serialize_str(&base64::encode_config(v.as_ref(), STANDARD_NO_PAD))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use std::convert::TryInto;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| D::Error::custom(err.to_string())))
        .map(|bytes| bytes[..].try_into())
        .and_then(|opt| opt.map_err(|_| D::Error::custom("failed to deserialize")))
}

impl fmt::Display for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use base64::STANDARD_NO_PAD;
        write!(f, "{}", base64::encode_config(&self.0, STANDARD_NO_PAD))
    }
}
impl fmt::Debug for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArtifactId:{}", base64::encode(&self.0))
    }
}

impl std::convert::AsRef<[u8]> for ArtifactId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ArtifactId {
    pub fn from_base64(input: &str) -> Result<Self, Error> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| Error::Base64Error)?;
        let array: [u8; 32] = decoded[..].try_into().map_err(|_| Error::TryFromSlice)?;
        Ok(ArtifactId(array.into()))
    }
}

// impl<T> std::convert::TryFrom<T> for ArtifactId
// where
//     T: AsRef<[u8]>,
// {
//     type Error = Error;

//     fn try_from(slice: T) -> Result<Self, Error> {
//         use std::convert::TryInto;
//         Ok(Self(
//             (&slice.as_ref()[..])
//                 .try_into()
//                 .map_err(|_| mindbase_util::Error::TryFromSlice)?,
//         ))
//     }
// }
