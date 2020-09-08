use super::private::AgentKey;

/// All of the structs in this module are safe to share with a trusted custodian
/// IE: The server. Note that we don't fully trust the custodian, and we take steps
/// to ensure that the custodian never directly holds any secrets

pub struct CustodialAgentKey {
    pub(crate) pubkey: [u8; 32],
    pub(crate) mask: KeyMask,
    pub(crate) check: [u8; 32],
}

/// KeyMask is a private key which has been XORed with a passkey
/// Such that a private key may be recoverable with the assistance of the custodian
/// but without disclosure of the actual private key
pub struct KeyMask {
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
