use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait TValue: Sized + Serialize + DeserializeOwned {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(bytes)?)
    }
    // type Symbol;
    // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>
    // where
    //     G: GraphInterface<Self>;
}

impl<T> TValue for T where T: Sized + Serialize + DeserializeOwned {}

pub trait TSymbol: Sized + Serialize + DeserializeOwned {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(bytes)?)
    }
    // Compare two symbols to determine a similarity score
    // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>;
    // where
    // G: GraphInterface<W>
    // W: Value<Symbol = Self>;
}

impl TSymbol for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self::from_utf8_lossy(bytes).to_string())
    }
}
pub trait TProvenance {}

pub trait GraphInterface<Sym, Val>
where
    Sym: TSymbol,
    Val: TValue,
{
    fn insert(&self, entity: Entity<Sym, Val>) -> Result<EntityId, Error>;
    fn get(&self, entity_id: &EntityId) -> Result<Entity<Sym, Val>, Error>;
    fn get_adjacencies(&self, entity_id: &EntityId) -> Result<Vec<EntityId>, Error>;
    fn get_adjacencies_matching<F>(&self, entity_id: &EntityId, filter: F) -> Result<Vec<EntityId>, Error>
    where
        F: Fn(&Sym, &Val) -> Result<bool, Error>;
}

impl TProvenance for () {}
