#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
pub mod analogy;
pub mod convenience;
pub mod fuzzyset;
pub mod symbol;

pub trait Entity: Clone + std::fmt::Display + std::cmp::Ord {}
pub trait Artifact {}

pub mod testing;

pub use crate::analogy::associative::AssociativeAnalogy;
pub use analogy::categorical::CategoricalAnalogy;

pub mod prelude {
    pub use crate::sym;
    pub use crate::symbol::Symbol;
}
