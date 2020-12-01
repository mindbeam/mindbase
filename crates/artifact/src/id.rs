use core::fmt;
use mindbase_util::Error;
use serde::{Deserialize, Serialize};

use crate::ArtifactId;

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
