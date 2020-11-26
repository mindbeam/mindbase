use mindbase_util::Error;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct AgentId {
    // #[serde(
    //     serialize_with = "mindbase_util::serde_helper::as_base64",
    //     deserialize_with = "mindbase_util::serde_helper::from_base64_32"
    // )]
    pub pubkey: [u8; 32],
}

impl AgentId {
    pub fn pubkey_short(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.pubkey[0..12], STANDARD_NO_PAD)
    }

    pub fn from_base64(input: &str) -> Result<Self, Error> {
        use std::convert::TryInto;
        let decoded = base64::decode(input).map_err(|_| Error::Base64Error)?;
        let array: [u8; 32] = decoded[..].try_into().map_err(|_| mindbase_util::Error::TryFromSlice)?;
        Ok(AgentId { pubkey: array.into() })
    }
}

impl mindbase_util::AsBytes for &AgentId {
    fn as_bytes(&self) -> Vec<u8> {
        self.pubkey[..].to_vec()
    }
}
impl std::convert::AsRef<[u8]> for AgentId {
    fn as_ref(&self) -> &[u8] {
        &self.pubkey[..]
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.pubkey_short())
    }
}
impl fmt::Debug for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AgentId:{}", &self.pubkey_short())
    }
}

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

#[cfg(test)]
mod test {
    use super::AgentId;

    #[test]
    fn serde() {
        let id = AgentId { pubkey: [0; 32] };
        let bytes = bincode::serialize(&id).unwrap();
        let id2: AgentId = bincode::deserialize(&bytes).unwrap();

        assert_eq!(id, id2);
    }
}
