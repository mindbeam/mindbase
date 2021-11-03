use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait Value: Sized + Serialize + DeserializeOwned {
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

pub trait Symbol: Sized + Serialize + DeserializeOwned {
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

impl Symbol for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self::from_utf8_lossy(bytes).to_string())
    }
}
pub trait Provenance {}

pub trait GraphInterface<Sym, Val>
where
    Sym: Symbol,
    Val: Value,
{
    fn insert(&self, entity: Entity<Sym, Val>) -> Result<EntityId, Error>;
    fn get(&self, entity_id: &EntityId) -> Result<Entity<Sym, Val>, Error>;
    fn get_adjacencies(&self, entity_id: &EntityId) -> Result<Vec<EntityId>, Error>;
    fn get_adjacencies_matching<F>(&self, entity_id: &EntityId, filter: F) -> Result<Vec<EntityId>, Error>
    where
        F: Fn(&Sym, &Val) -> Result<bool, Error>;
}

impl Provenance for () {}

pub mod basics {
    use crate::Error;

    impl super::Value for String {
        // type Symbol = ();
        // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error> {
        //     if self == other {
        //         Ok(1.0)
        //     } else {
        //         Ok(0.0)
        //     }
        // }
    }
}
