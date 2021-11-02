pub mod adapter;
pub mod entity;
pub mod error;
pub mod hypergraph;
mod index;
pub mod traits;

pub use entity::{Entity, EntityId};
pub use error::Error;
pub use hypergraph::Hypergraph;
