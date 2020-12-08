use std::marker::PhantomData;

use error::Error;
use mindbase_store::{Store, Tree};

pub mod error;
pub mod traits;

// use traits::NodeInstance;

#[derive(Default, Debug)]
struct Graph<S, A, I>
where
    S: Store,
    A: traits::Artifact,
    I: traits::ArtifactInstance,
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
    I: traits::ArtifactInstance,
{
    fn new(store: S) -> Result<Self, Error> {
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
    fn put_artifact<T: Into<A>>(&mut self, artifact: T) -> Result<A::ID, Error> {
        let artifact: A = artifact.into();

        let artifact_id = artifact.id();
        // Only store it if we haven't seen this one before

        self.artifacts.merge(artifact_id, artifact);

        // either way we want to create an instance

        // LEFT OFF HERE
        // let instance = A::new(artifact_id);
        // self.instances.insert(instance.id, instance.clone());

        instance
    }
}
