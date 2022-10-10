use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha512Trunc256};

use crate::{
    entity::{EntityInner, EntityIx},
    traits::{Provenance, TSymbol, TValue},
    Entity, EntityId, Error,
};

pub mod sled;

pub trait StorageAdapter<Sym, Val, Prov = ()>
where
    Sym: TSymbol,
    Val: TValue,
    Prov: Provenance,
{
    fn insert(&self, entity: Entity<Sym, Val>) -> Result<(EntityIx, EntityId), Error>;
    fn get_by_ix(&self, entity_ix: &EntityIx) -> Result<Entity<Sym, Val>, Error>;
    // fn put_symbol<T: Into<Sym>>(&mut self, into_sym: T) -> Result<(SymbolRef, SymbolId), Error>;
}

// #[derive(Serialize, Deserialize)]
// struct StoredProperty(SymbolId, ValueRef);
#[derive(Serialize, Deserialize)]
struct StoredProperty(Vec<u8>, Vec<u8>);

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredEntity(EntityId, Vec<StoredProperty>, EntityInner);

impl StoredEntity {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(bytes)?)
    }
}

/// The hash of the weight which was stored
type ValueIx = u64;

/// The hash of the symbol which was stored
type SymbolIx = u64;

// #[derive(Serialize, Deserialize)]
// enum ValueRef {
//     // Inline(Vec<u8>),
//     Remote(ValueId),
// }
// enum SymbolRef {
//     // Inline(Vec<u8>),
//     Remote(ValueId),
// }

// impl ValueRef {
//     fn ix(&self) -> ValueIx {
//         match self {
//             // ValueRef::Inline(bytes) => {
//             //     let mut hasher = Sha512Trunc256::default();
//             //     hasher.update(bytes);
//             //     let id: ValueId = hasher.finalize().into();
//             //     id
//             // }
//             ValueRef::Remote(id) => id.clone(),
//         }
//     }
//     fn fixed(&self) -> [u8; 32] {
//         match self {
//             ValueRef::Remote(id) => id.to_owned(),
//         }
//     }
// }

// impl ValueRef {
//     fn id(&self) -> ValueId {
//         match self {
//             // ValueRef::Inline(bytes) => {
//             //     let mut hasher = Sha512Trunc256::default();
//             //     hasher.update(bytes);
//             //     let id: ValueId = hasher.finalize().into();
//             //     id
//             // }
//             ValueRef::Remote(id) => id.clone(),
//         }
//     }
//     fn fixed(&self) -> [u8; 32] {
//         match self {
//             ValueRef::Remote(id) => id.to_owned(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub(crate) struct StoredSymbolId(SymbolId);
