use serde::{Deserialize, Serialize};

use crate::{entity::EntityId, traits::Weight};

pub struct Hyperedge<W>
where
    W: Weight,
{
    pub(crate) weight: W,
    pub(crate) inner: HyperedgeInner,
}
/// TODO1 - implement fuzzy membership
#[derive(Serialize, Deserialize)]
pub(crate) enum HyperedgeInner {
    Undirected(Vec<EntityId>),
    Directed(Vec<EntityId>, Vec<EntityId>),
}

pub fn directed<W, F, T>(weight: W, from: F, to: T) -> Hyperedge<W>
where
    W: Weight,
    F: Into<Vec<EntityId>>,
    T: Into<Vec<EntityId>>,
{
    Hyperedge {
        weight,
        inner: HyperedgeInner::Directed(from.into(), to.into()),
    }
}
pub fn undirected<W>(weight: W, members: Vec<EntityId>) -> Hyperedge<W>
where
    W: Weight,
{
    Hyperedge {
        weight,
        inner: HyperedgeInner::Undirected(members),
    }
}
