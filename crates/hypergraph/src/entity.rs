use std::{
    convert::TryInto,
    fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

use crate::{traits::Weight, Error};

/// HyperedgeId is a ULID
#[derive(Serialize, Deserialize, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct EntityId(pub(crate) [u8; 16]);

impl EntityId {
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(EntityId(slice.try_into().map_err(|_| Error::InvalidSlice)?))
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "h:{}", b)
    }
}
impl Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::STANDARD_NO_PAD;
        let b = base64::encode_config(&self.0, STANDARD_NO_PAD);
        write!(f, "hyperedge_id:{}", b)
    }
}

pub struct Entity<W>
where
    W: Weight,
{
    pub weight: W,
    pub(crate) inner: EntityInner,
}
/// TODO1 - implement fuzzy membership
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum EntityInner {
    Vertex,
    Undirected(Vec<EntityId>),
    Directed(Vec<EntityId>, Vec<EntityId>),
}

pub fn directed<'a, W, WI, F, T>(weight: WI, from: F, to: T) -> Entity<W>
where
    W: Weight,
    WI: Into<W>,
    F: Into<Vec<EntityId>>,
    T: Into<Vec<EntityId>>,
{
    Entity {
        weight: weight.into(),
        inner: EntityInner::Directed(from.into(), to.into()),
    }
}
pub fn undirected<'a, WI, W, M>(weight: WI, members: M) -> Entity<W>
where
    WI: Into<W>,
    W: Weight,
    M: Into<Vec<EntityId>>,
{
    Entity {
        weight: weight.into(),
        inner: EntityInner::Undirected(members.into()),
    }
}

pub fn vertex<W, WI>(weight: WI) -> Entity<W>
where
    W: Weight,
    WI: Into<W>,
{
    Entity {
        weight: weight.into(),
        inner: EntityInner::Vertex,
    }
}
