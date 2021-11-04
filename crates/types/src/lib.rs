pub mod cas;

use chrono::{DateTime, Utc};
pub use mindbase_util::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

// when querying a property+value index, we must know the type in advance
// but the problem is that the property symbol might be fuzzy
// We could ground these out in genesis symbols
// BUT: Are these symbols composed with the property symbol?

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MBValue {
    Agent(keyplace::AgentId),
    String(String),
    DateTime(DateTime<Utc>),
    Uint32(u32),
    // Struct()
    Json(Vec<u8>),
    Bytes(Vec<u8>),
}

impl std::fmt::Display for MBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MBValue::Agent(a) => todo!(),
            MBValue::String(s) => todo!(),
            MBValue::DateTime(d) => todo!(),
            MBValue::Uint32(v) => todo!(),
            MBValue::Json(j) => todo!(),
            MBValue::Bytes(b) => todo!(),
        }
    }
}
