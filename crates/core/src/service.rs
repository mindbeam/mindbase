use mindbase_artifact::Artifact;
use mindbase_mbql::Query;
use mindbase_store::{MemoryStore, Store, Tree};

use crate::Error;

pub mod index;

struct Service<S: Store> {
    store: S,

    /// Sig-Addressable store for Entities (EntityId())
    allegations: S::Tree,

    graph: Graph<S, Artifact>,

    /// Reverse lookup for all allegations
    // analogy_rev: S::Tree,

    /// Reverse lookup for all allegations
    atoms_by_artifact_agent: S::Tree,

    /// I forget why I would actually need known agents
    _known_agents: S::Tree,
}

impl<S> Service<S>
where
    S: Store,
{
    pub fn new(store: S) -> Result<Self, Error> {
        let graph = Graph::new(store.clone());

        // TODO 1 - should this be moved into Graph?
        let allegations = store.open_tree("core::allegations")?;
        let atoms_by_artifact_agent = store.open_tree("core::allegation_rev")?;
        // let analogy_rev = db.open_tree("allegation_rev")?;

        // Both of these are &k[..] / Vec<sorted u8;16 chunks>
        atoms_by_artifact_agent.set_merge_operator(index::merge_16byte_list);
        // analogy_rev.set_merge_operator(merge_16byte_list);

        // let default_agent = _default_agent(&my_agents)?;
        let _known_agents = store.open_tree("core::known_agents")?;

        Ok(Service {
            store,
            graph,
            allegations,
            atoms_by_artifact_agent,
            _known_agents,
        })
    }
    pub fn query_str(&self, mbql_str: &str) -> Result<Query, Error> {
        Query::from_str(self, mbql_str)
    }

    pub fn query<T: std::io::BufRead>(&self, reader: T) -> Result<Query, Error> {
        Query::new(self, reader)
    }
}

#[test]
fn basic() -> Result<(), Error> {
    let mb = Service::new(MemoryStore::new())?;
    let query = mb.query_str(r#"$isaid = Ground("Things that I said" : "In this test")"#)?;

    query.apply()?;
    let isaid = query.get_symbol_for_var("isaid")?.unwrap();

    Ok(())
}
