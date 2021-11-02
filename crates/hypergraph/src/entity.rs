use std::{
    convert::TryInto,
    fmt::{Debug, Display},
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{traits::Value, Error};

/// HyperedgeId is a ULID
#[derive(Serialize, Deserialize, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct EntityId(pub(crate) [u8; 16]);

impl EntityId {
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(EntityId(slice.try_into().map_err(|_| Error::InvalidSlice)?))
    }
    pub fn short(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0[12..], STANDARD_NO_PAD)
    }
    pub fn full(&self) -> String {
        use base64::STANDARD_NO_PAD;
        base64::encode_config(&self.0, STANDARD_NO_PAD)
    }
    pub fn write_short<W: std::io::Write>(&self, w: W) {
        use base64::STANDARD_NO_PAD;
        use std::io::Write;
        let mut enc = base64::write::EncoderWriter::new(w, STANDARD_NO_PAD);
        enc.write_all(&self.0[12..]).unwrap();
        enc.finish().unwrap();
    }
    pub fn write_full<W: std::io::Write>(&self, w: W) {
        use base64::STANDARD_NO_PAD;
        use std::io::Write;
        let mut enc = base64::write::EncoderWriter::new(w, STANDARD_NO_PAD);
        enc.write_all(&self.0).unwrap();
        enc.finish().unwrap();
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short())
    }
}
impl Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full())
    }
}

#[derive(Debug)]
pub struct Property<Sym, Val> {
    pub key: Sym,
    pub value: Val,
}

#[derive(Debug)]
pub struct Entity<Key, Val> {
    pub properties: Vec<Property<Key, Val>>,
    pub(crate) inner: EntityInner,
}

/// TODO1 - implement fuzzy membership
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum EntityInner {
    Vertex,
    Edge(Vec<EntityId>),
    DirectedEdge(Vec<EntityId>, Vec<EntityId>),
}

impl Display for EntityInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityInner::Vertex => write!(f, "Vertex"),
            EntityInner::Edge(e) => write!(f, "Edge({})", e.iter().map(|eid| eid.short()).join(",")),
            EntityInner::DirectedEdge(fe, te) => write!(
                f,
                "DirectedEdge({} -> {})",
                fe.iter().map(|eid| eid.short()).join(","),
                te.iter().map(|eid| eid.short()).join(",")
            ),
        }
        // let mut comma = false;
        // match self {
        //     EntityInner::Vertex => write!(f, "Vertex"),
        //     EntityInner::Edge(v) => {
        //         write!(f, "Edge(");
        //         v.iter().map(|e| {
        //             if comma {
        //                 write!(f, ", ");
        //             }
        //             comma = true;
        //             e.short(f)
        //         });
        //         write!(f, ")")
        //     }
        //     EntityInner::DirectedEdge(_, _) => todo!(),
        // }
    }
}

pub fn directed<'a, Key, Val, PI, F, T>(properties: PI, from: F, to: T) -> Entity<Key, Val>
where
    PI: Into<Vec<Property<Key, Val>>>,
    F: Into<Vec<EntityId>>,
    T: Into<Vec<EntityId>>,
{
    Entity {
        properties: properties.into(),
        inner: EntityInner::DirectedEdge(from.into(), to.into()),
    }
}
pub fn undirected<'a, Key, Val, PI, M>(properties: PI, members: M) -> Entity<Key, Val>
where
    PI: Into<Vec<Property<Key, Val>>>,
    M: Into<Vec<EntityId>>,
{
    Entity {
        properties: properties.into(),
        inner: EntityInner::Edge(members.into()),
    }
}

pub fn vertex<Key, Val, PI>(properties: PI) -> Entity<Key, Val>
where
    PI: Into<Vec<Property<Key, Val>>>,
{
    Entity {
        properties: properties.into(),
        inner: EntityInner::Vertex,
    }
}
