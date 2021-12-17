pub mod cas;

use std::convert::TryInto;

use chrono::{Date, DateTime, Utc};
pub use mindbase_util::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

// when querying a property+value index, we must know the type in advance
// but the problem is that the property symbol might be fuzzy
// We could ground these out in genesis symbols
// BUT: Are these symbols composed with the property symbol?

// TODO: implement this as zerocopy BE bytes, with a tag
// this way we can use it for index writing/querying and serialization
pub struct MBValue(Inner);

enum Inner {
    IVec(sled::IVec),
    Vec(Vec<u8>),
    U8_16([u8; 16]),
}

impl AsRef<[u8]> for Inner {
    fn as_ref(&self) -> &[u8] {
        match self {
            Inner::IVec(v) => v.as_ref(),
            Inner::Vec(v) => v.as_ref(),
            Inner::U8_16(v) => v.as_ref(),
        }
    }
}

pub enum MBType {
    // Agent,
    String,
    DateTime,
    Uint32,
    // Struct()
    Json,
    Bytes,
    // Extended()
}

impl MBType {
    fn to_byte(self) -> u8 {
        match self {
            MBType::String => 1,
            MBType::DateTime => 2,
            MBType::Uint32 => 3,
            MBType::Json => 4,
            MBType::Bytes => 5,
        }
    }
    fn from_byte(byte: u8) -> Self {
        match byte {
            1 => MBType::String,
            2 => MBType::DateTime,
            3 => MBType::Uint32,
            4 => MBType::Json,
            5 => MBType::Bytes,
            _ => panic!(),
        }
    }
}

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub enum MBValue {
//     // Agent(keyplace::AgentId),
//     String(String),
//     DateTime([u8; 12]),
//     Uint32(u32),
//     // Struct()
//     Json(Vec<u8>),
//     Bytes(Vec<u8>),
//     // Blob() - presumably we need a dedicated blob store with various pages?
//     // UserImplementedType()
// }

impl MBValue {
    pub fn string(s: &str) -> Self {
        let s: &[u8] = s.as_ref();
        let mut v = Vec::with_capacity(s.len() + 1);
        v.push(MBType::String.to_byte());
        v.extend(s);

        MBValue(Inner::Vec(v))
    }
    pub fn dateTime(secs: i64, nanos: u32) -> Self {
        let mut a = [0u8; 16];
        a[0] = MBType::DateTime.to_byte();
        a[1..9].clone_from_slice(&secs.to_be_bytes());
        a[9..13].clone_from_slice(&nanos.to_be_bytes());
        MBValue(Inner::U8_16(a))
    }
    pub fn uint32(i: u32) -> Self {
        let mut a = [08; 16];
        a[0] = MBType::Uint32.to_byte();
        a[1..5].clone_from_slice(&i.to_be_bytes());
        MBValue(Inner::U8_16(a))
    }
    // pub fn idx_bytes(&self) -> &[u8] {
    //     match self {
    //         Self::Agent(id) => &id.pubkey,
    //         Self::String(s) => s.as_ref(),
    //         Self::DateTime(v) => v.as_ref(),
    //         Self::Uint32(u32) => u.as_ref(),
    //         // Self::Struct()
    //         Self::Json(v) => {}
    //         Self::Bytes(v) => {}
    //     }
    // }
}

impl std::fmt::Display for MBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // match self {
        //     MBValue::Agent(a) => todo!(),
        //     MBValue::String(s) => todo!(),
        //     MBValue::DateTime(d) => todo!(),
        //     MBValue::Uint32(v) => todo!(),
        //     MBValue::Json(j) => todo!(),
        //     MBValue::Bytes(b) => todo!(),
        // }
    }
}
