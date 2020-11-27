#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
pub mod analogy;
pub mod artifact;
pub mod claim;
pub mod fuzzyset;
pub mod symbol;

#[cfg(test)]
pub mod testing;

pub use analogy::associative::AssociativeAnalogy;
pub use analogy::categorical::CategoricalAnalogy;
