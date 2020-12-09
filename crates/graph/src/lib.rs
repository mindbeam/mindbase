use std::marker::PhantomData;

use error::Error;
use mindbase_store::{Store, Tree};

pub mod error;
pub mod traits;

// use traits::NodeInstance;

#[derive(Default, Debug)]
pub struct Graph<S, A, I>
where
    S: Store,
    A: traits::Artifact,
    I: traits::ArtifactInstance<A>,
{
    _store: S,
    artifacts: S::Tree,
    instances: S::Tree,
    _a: PhantomData<A>,
    _i: PhantomData<I>,
}

// TODO 1 - Claim vs Artifact Instance
//
//      EITHER
// * Claims must be artifacts
// or
// * Claims and Artifact instances are different types of things
// or
// * Claims are sometimes artifact instances, but illegal instance values are possible to represent

impl<S, A, I> Graph<S, A, I>
where
    S: Store,
    A: traits::Artifact,
    I: traits::ArtifactInstance<A>,
{
    pub fn new(store: S) -> Result<Self, Error> {
        let artifacts = store.open_tree("graph::artifacts")?;

        fn write_once(
            _key: &[u8],               // the key being merged
            last_bytes: Option<&[u8]>, // the previous value, if one existed
            op_bytes: &[u8],           /* the new bytes being merged in */
        ) -> Option<Vec<u8>> {
            match last_bytes {
                Some(_) => None,
                None => Some(op_bytes.to_vec()),
            }
        }

        artifacts.set_merge_operator(write_once);

        let instances = store.open_tree("graph::instances")?;

        Ok(Graph {
            _store: store,
            artifacts,
            instances,
            _a: PhantomData,
            _i: PhantomData,
        })
    }
    pub fn put_artifact<T: Into<A>>(&mut self, artifact: T) -> Result<I, Error> {
        let artifact: A = artifact.into();

        let (artifact_id, bytes) = artifact.get_id_and_bytes();
        // Only store it if we haven't seen this one before
        let instance = I::instantiate(&artifact_id);

        self.artifacts.merge(artifact_id, bytes)?;

        // either way we want to create an instance

        let (instance_id, bytes) = instance.get_id_and_bytes();
        self.instances.insert(instance_id.as_ref(), bytes)?;

        Ok(instance)
    }
}
