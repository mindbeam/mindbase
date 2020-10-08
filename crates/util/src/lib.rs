pub mod error;
pub mod iter;

pub use crate::error::Error;

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait TryFromBytes
where
    Self: Sized,
{
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>;
}

pub mod array64;

pub mod serde_helper;
