use bincode::serialize;
use rusty_ulid::generate_ulid_bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Write, marker::PhantomData, sync::atomic::AtomicU64};

use crate::{index, Entity, EntityId, Error};

fn serialize<T: Serialize>(thing: &T) -> Result<Vec<u8>, Error> {
    Ok(bincode::serialize(thing)?)
}
fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    Ok(bincode::deserialize(bytes)?)
}

pub struct SledAdapter<Prop, Val, Prov = ()> {
    /// Keyed on UUID for now, but this is ripe for optimization
    entity_storage: sled::Tree,

    /// Smaller values may be stored directly in the hyper entity property
    /// But larger weights should be stored separately
    /// Keyed by hash of value
    value_storage: sled::Tree,
    last_value_id: AtomicU64,

    /// Symbols could potentially be large. We need to locally enumerate them for compactness
    symbol_storage: sled::Tree,
    last_symbol_id: AtomicU64,

    /// Index the raw weight value directly to the hyperedge
    idx_propertyvalue_to_entity: sled::Tree,
    idx_entity_to_hyperedge: sled::Tree,

    db: sled::Db,

    // Prevent mixing and matching
    #[doc(hidden)]
    _prop: PhantomData<Prop>,
    #[doc(hidden)]
    _val: PhantomData<Val>,
    #[doc(hidden)]
    _prov: PhantomData<Prov>,
}

impl<Sym, Val> SledAdapter<Sym, Val>
where
    Sym: Serialize + DeserializeOwned,
    Val: Serialize + DeserializeOwned,
{
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        let symbol_storage = db.open_tree("hypergraph::symbol_storage")?;
        symbol_storage.set_merge_operator(op_write_once);
        let last_symbol_id = AtomicU64::new(symbol_storage.last().map_or(0, |v| v.map_or(0, |(k, _)| read_be_u64(&k))));

        let value_storage = db.open_tree("hypergraph::value_storage")?;
        value_storage.set_merge_operator(op_write_once);
        let last_value_id = AtomicU64::new(value_storage.last().map_or(0, |v| v.map_or(0, |(k, _)| read_be_u64(&k))));

        let entity_storage = db.open_tree("hypergraph::entity_storage")?;
        let last_entity_id = AtomicU64::new(entity_storage.last().map_or(0, |v| v.map_or(0, |(k, _)| read_be_u64(&k))));

        let idx_entity_to_hyperedge = db.open_tree("hypergraph::hyperedge_by_entity_id")?;
        idx_entity_to_hyperedge.set_merge_operator(index::merge_byte_list::<typenum::U16>);

        let idx_propertyvalue_to_entity = db.open_tree("hypergraph::entity_by_weight_index")?;
        idx_propertyvalue_to_entity.set_merge_operator(index::merge_byte_list::<typenum::U16>);

        Ok(SledAdapter {
            _prop: PhantomData,
            _val: PhantomData,
            _prov: PhantomData,
            db,
            symbol_storage,
            last_symbol_id,
            value_storage,
            last_value_id,
            entity_storage,
            idx_entity_to_hyperedge,
            idx_propertyvalue_to_entity,
        })
    }

    pub fn insert(&self, entity: Entity<Sym, Val>) -> Result<EntityId, Error> {
        let entity_id: [u8; 16] = generate_ulid_bytes();

        //     (&self.symbol_storage, &self.value_storage, &self.entity_storage, )
        // .transaction(|(unprocessed, processed)| {
        //TODO: sled transactions

        let storedprops = Vec::new();
        for property in entity.properties {
            let (symbol_ref, symbol_id): (SymbolRef, SymbolId) = self.put_symbol(property.key)?;
            let (value_ref, value_id): (SymbolRef, ValueId) = self.put_value(property.value)?;

            self.idx_propertyvalue_to_entity.merge(value_id, entity_id)?;
        }

        match &entity.inner {
            EntityInner::Vertex => {}
            EntityInner::Edge(member_ids) => {
                unimplemented!()
                // for m in member_ids.iter() {
                //     self.idx_entity_to_hyperedge.merge(m.0, &entity_id)?;
                // }
            }
            EntityInner::DirectedEdge(from_member_ids, to_member_ids) => {
                unimplemented!()
                // for m in from_member_ids.iter() {
                //     self.idx_entity_to_hyperedge.merge(m.0, &entity_id)?;
                // }
                // for m in to_member_ids.iter() {
                //     self.idx_entity_to_hyperedge.merge(m.0, &entity_id)?;
                // }
            }
        }

        self.entity_storage
            .insert(entity_id, bincode::serialize(&StoredEntity(storedprops, entity.inner))?)?;

        // What if we did this once per property?
        // symbol id + value -> [entity_id]

        Ok(EntityId(entity_id).into())
    }

    // pub fn get_weight(&self, entity_id: &EntityId) -> Result<Val, Error> {
    // match self.entity_storage.get(entity_id.0)? {
    //     Some(entity_bytes) => {
    //         let sv: StoredEntity = deserialize(&entity_bytes)?;
    //         Ok(self.get_weight_by_ref(sv.0)?)
    //     }
    //     None => Err(Error::NotFound),
    // }
    // }

    fn put_symbol<T: Into<Sym>>(&mut self, into_sym: T) -> Result<(SymbolRef, SymbolId), Error> {
        let symbol: Sym = into_sym.into();

        let bytes = serialize(&symbol)?;

        // let mut hasher = Sha512Trunc256::default();
        // hasher.update(&bytes);
        // let id: ValueId = hasher.finalize().into();
        self.last_symbol_id += 1;
        let event_id = self.last_event_id;

        let wr = if bytes.len() < 32 {
            SymbolRef::Inline(bytes.clone())
        } else {
            SymbolRef::Remote(StoredValueId(id))
        };

        // Only store it if we haven't seen this one before
        self.value_storage.merge(id.clone(), bytes)?;

        Ok((wr, id))
    }
    fn put_value<T: Into<Val>>(&self, into_value: T) -> Result<(SymbolRef, ValueId), Error> {
        let weight: Val = into_value.into();

        let bytes = serialize(&weight)?;
        let mut hasher = Sha512Trunc256::default();
        hasher.update(&bytes);
        let id: ValueId = hasher.finalize().into();

        let wr = if bytes.len() < 32 {
            SymbolRef::Inline(bytes.clone())
        } else {
            SymbolRef::Remote(StoredValueId(id))
        };

        // Only store it if we haven't seen this one before
        self.value_storage.merge(id.clone(), bytes)?;

        Ok((wr, id))
    }
    fn get_weight_by_ref(&self, wr: SymbolRef) -> Result<Val, Error> {
        match wr {
            SymbolRef::Inline(ref bytes) => Ok(deserialize(bytes)?),
            SymbolRef::Remote(id) => match self.value_storage.get(id.0)? {
                Some(ref bytes) => Ok(deserialize(bytes)?),
                None => return Err(Error::NotFound),
            },
        }
    }

    pub fn dump_entities<O: Write>(&self, mut writer: O) -> Result<(), Error>
    where
        Val: std::fmt::Display,
    {
        unimplemented!()
        // for entity_rec in self.entity_storage.iter() {
        //     let (id_bytes, bytes) = entity_rec?;
        //     let entity_id = EntityId::from_slice(&id_bytes[0..16])?;
        //     let entity: StoredEntity = deserialize(&bytes)?;
        //     write!(
        //         writer,
        //         "{} = {} Weight: {}\n",
        //         entity_id,
        //         entity.1,
        //         self.get_weight_by_ref(entity.0)?,
        //     )?;
        // }
        // Ok(())
    }
}

fn op_write_once(
    _key: &[u8],               // the key being merged
    last_bytes: Option<&[u8]>, // the previous value, if one existed
    op_bytes: &[u8],           /* the new bytes being merged in */
) -> Option<Vec<u8>> {
    match last_bytes {
        Some(_) => None,
        None => Some(op_bytes.to_vec()),
    }
}

fn read_be_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u64>());
    // *input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
}

fn read_be_u32(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u32>());
    // *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}
