use super::private::AgentKey;
use serde::{Deserialize, Serialize};

use mindbase_util::serde_helper;
/// All of the structs in this module are safe to share with a trusted custodian
/// IE: The server. Note that we don't fully trust the custodian, and we take steps
/// to ensure that the custodian never directly holds any secrets

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustodialAuthKey {
    #[serde(
        serialize_with = "serde_helper::as_base64",
        deserialize_with = "serde_helper::from_base64_32"
    )]
    pub(crate) auth: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustodialAgentKey {
    #[serde(
        serialize_with = "serde_helper::as_base64",
        deserialize_with = "serde_helper::from_base64_32"
    )]
    pub(crate) pubkey: [u8; 32],
    pub(crate) mask: KeyMask,
    #[serde(
        serialize_with = "serde_helper::as_base64",
        deserialize_with = "serde_helper::from_base64_32"
    )]
    pub(crate) check: [u8; 32],
    pub(crate) auth: CustodialAuthKey,
}

/// KeyMask is a private key which has been XORed with a passkey
/// Such that a private key may be recoverable with the assistance of the custodian
/// but without disclosure of the actual private key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyMask {
    #[serde(
        serialize_with = "serde_helper::as_base64",
        deserialize_with = "serde_helper::from_base64_32"
    )]
    pub(crate) mask: [u8; 32],
}

impl KeyMask {
    pub fn base64(&self) -> String {
        base64::encode(self.mask)
    }
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.mask
    }
}
