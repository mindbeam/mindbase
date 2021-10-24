use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait Value: Serialize + DeserializeOwned + std::fmt::Debug {
    // type Symbol;
    // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>
    // where
    //     G: GraphInterface<Self>;
}

pub trait Symbol {
    // Compare two symbols to determine a similarity score
    fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>;
    // where
    // G: GraphInterface<W>
    // W: Value<Symbol = Self>;
}
pub trait Provenance {}

pub trait GraphInterface<W>
where
    W: Value + std::fmt::Debug,
{
    fn insert(&self, entity: Entity<W>) -> Result<EntityId, Error>;
    fn get(&self, entity_id: &EntityId) -> Result<Entity<W>, Error>;
    fn get_adjacencies(&self, entity_id: &EntityId) -> Result<Vec<EntityId>, Error>;
    fn get_adjacencies_matching<F>(&self, entity_id: &EntityId, filter: F) -> Result<Vec<EntityId>, Error>
    where
        F: Fn(&W) -> Result<bool, Error>;
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
