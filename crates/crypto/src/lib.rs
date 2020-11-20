extern crate zeroize;

mod error;
pub mod key_manager;
mod keys;
mod signature;

pub use error::Error;
pub use key_manager::KeyManager;
pub use keys::{AgentId, AgentKey, CustodialAgentKey, PassKey, UserAuthKey};
pub use signature::Signature;
