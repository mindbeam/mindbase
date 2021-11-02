use crate::{Entity, EntityId, Error};

pub mod sled;

pub trait StorageAdapter<Sym, Val, Prov> {
    fn insert(&self, entity: Entity<Sym, Val>) -> Result<EntityId, Error>;
    // fn put_symbol<T: Into<Sym>>(&mut self, into_sym: T) -> Result<(SymbolRef, SymbolId), Error>;
}
