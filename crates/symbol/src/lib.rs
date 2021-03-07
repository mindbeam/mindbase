#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
pub mod analogy;
pub mod convenience;
pub mod symbol;
pub mod traits;

pub mod testing;

pub mod prelude {
    pub use crate::sym;
    pub use crate::symbol::Symbol;
}
