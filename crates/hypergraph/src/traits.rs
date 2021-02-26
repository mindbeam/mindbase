use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait Weight: Serialize + DeserializeOwned {
    type Symbol;
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
    fn get_edges_containing(&self, entity_id: &EntityId) -> Result<Option<Vec<EntityId>>, Error>;
}

impl Provenance for () {}

pub mod basics {
    impl super::Weight for String {
        type Symbol = ();
    }
}
