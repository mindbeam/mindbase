pub mod body;
pub mod id;

pub use body::Artifact;
pub use id::ArtifactId;
pub use mindbase_util::Error;
use serde::Serialize;

pub trait NodeType: Serialize {}
