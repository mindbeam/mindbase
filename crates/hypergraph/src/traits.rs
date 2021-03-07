use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait Weight: Serialize + DeserializeOwned {
    type Symbol;
    // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>
    // where
    //     G: GraphInterface<Self>;
}

pub trait Symbol: std::fmt::Debug {
    // Compare two symbols to determine a similarity score
    fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error>
    where
        G: GraphInterface<W>,
        W: Weight<Symbol = Self>;
}
pub trait Provenance {}

pub trait GraphInterface<W>
where
    W: Weight,
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

    impl super::Weight for String {
        type Symbol = ();
        // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, Error> {
        //     if self == other {
        //         Ok(1.0)
        //     } else {
        //         Ok(0.0)
        //     }
        // }
    }
}
