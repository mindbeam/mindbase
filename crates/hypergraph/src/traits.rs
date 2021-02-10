use serde::{de::DeserializeOwned, Serialize};

use crate::{Entity, EntityId, Error};

pub trait Weight: Serialize + DeserializeOwned {}
pub trait Provenance {}

pub trait GraphInterface<W>
where
    W: Weight,
{
    fn insert(&self, entity: Entity<W>) -> Result<EntityId, Error>;
    fn get(&self, entity_id: &EntityId) -> Result<Entity<W>, Error>;
}

impl Provenance for () {}

pub mod basics {
    impl super::Weight for String {}
}
