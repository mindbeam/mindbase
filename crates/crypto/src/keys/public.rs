use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub pubkey: [u8; 32],
    pub email: Option<String>,
}

impl AgentIdentity {
    pub fn pubkey_base64(&self) -> String {
        base64::encode_config(&self.pubkey[..], base64::STANDARD_NO_PAD)
    }
}

impl fmt::Display for AgentIdentity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = base64::encode_config(&self.pubkey[..], base64::STANDARD_NO_PAD);
        write!(
            f,
            "{} ({})",
            &string,
            self.email.as_ref().map(|v| &v[..]).unwrap_or("-" as &str)
        )
    }
}
