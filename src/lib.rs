mod agent;
mod allegation;
mod analogy;
mod artifact;
mod concept;
mod error;
mod genesis;
mod util;

pub use self::{
    agent::{
        Agent,
        AgentId,
    },
    allegation::{
        Allegation,
        AllegationId,
    },
    artifact::ArtifactId,
    error::Error,
};
use artifact::Artifact;

// pub mod allegation_capnp {
//     include!(concat!(env!("OUT_DIR"), "/capnp/allegation_capnp.rs"));
// }

pub struct MindBase {
    /// Sig-Addressable store for Entities (EntityId())
    allegations: sled::Tree,

    /// Content-addressable store for artifacts. ArtifactId(Sha512Trunc256)
    artifacts: sled::Tree,

    /// Credential storage for all agents we manage
    my_agents: sled::Tree,

    ///
    known_agents: sled::Tree,
}

impl MindBase {
    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        let my_agents = db.open_tree("agents")?;
        let artifacts = db.open_tree("artifacts")?;
        let allegations = db.open_tree("allegations")?;

        let known_agents = db.open_tree("known_agents")?;

        let me = MindBase { allegations,
                            my_agents,
                            artifacts,
                            known_agents };

        me.genesis()?;

        me.default_agent()?;

        Ok(me)
    }

    /// Include whatever batteries we want to include
    pub fn genesis(&self) -> Result<(), Error> {
        // TODO 2 - use the genesis Agent, not ours
        // TODO 2 - make this NoOp when an exact artifact exists
        //    Other entity types should NOT deduplicate, only artifacts. This means they have to be hashed, but other entity types
        // should be enumerated
        crate::genesis::language_en::genesis(self)?;

        Ok(())
    }

    pub fn default_agent(&self) -> Result<Agent, Error> {
        match self.my_agents.get(b"latest")? {
            None => self.create_agent(),
            Some(pubkey) => {
                match self.my_agents.get(pubkey)? {
                    None => Err(Error::AgentHandleNotFound),
                    Some(v) => {
                        let agenthandle = bincode::deserialize(&v)?;
                        Ok(agenthandle)
                    },
                }
            },
        }
    }

    #[allow(unused)]
    pub fn put_allegation(&self, allegation: &Allegation) -> Result<(), Error> {
        let encoded: Vec<u8> = bincode::serialize(&allegation).unwrap();

        self.allegations.insert(allegation.id(), encoded)?;
        self.allegations.flush()?;

        Ok(())
    }

    #[allow(unused)]
    pub fn create_agent(&self) -> Result<Agent, Error> {
        let agent = Agent::new();

        let encoded: Vec<u8> = bincode::serialize(&agent).unwrap();
        self.my_agents.insert(agent.pubkey().unwrap().as_bytes(), encoded)?;
        self.my_agents.insert(b"latest", agent.pubkey().unwrap().as_bytes())?;
        self.my_agents.flush()?;

        Ok(agent)
    }

    fn assert_artifact<T>(&self, artifact: T) -> Result<ArtifactId, Error>
        where T: Into<Artifact>
    {
        let artifact: Artifact = artifact.into();
        let (id, bytes) = artifact.get_id_and_bytes();

        use sled::CompareAndSwapError;
        match self.artifacts.compare_and_swap(&id, None::<&[u8]>, Some(bytes))? {
            Ok(_) => {
                // inserted
            },
            Err(CompareAndSwapError { .. }) => {
                // already existed
            },
        }
        self.artifacts.flush()?;

        Ok(id)
    }

    fn entity_iter(&self) -> Iter {
        Iter { iter: self.allegations.iter(), }
    }

    #[allow(unused)]
    pub fn dump_json<T: std::io::Write>(&self, mut writer: T) -> Result<(), Error> {
        for maybe_entity in self.entity_iter() {
            let entity = maybe_entity?; // we may have failed to retrieve/decode one of them

            let entity_string = serde_json::to_string(&entity)?;
            writer.write(entity_string.as_bytes())?;
            writer.write(b"\n");
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn load_json<T: std::io::BufRead>(&self, mut reader: T) -> Result<(), Error> {
        for line in reader.lines() {
            let allegation: Allegation = serde_json::from_str(&line?[..])?;
            self.put_allegation(&allegation)?;
        }

        Ok(())
    }
}

struct Iter {
    iter: sled::Iter,
}

impl Iterator for Iter {
    type Item = Result<Allegation, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // Did we find it?
        match self.iter.next() {
            // End of the road
            None => None,

            // We got one
            Some(retrieval) => {
                match retrieval {
                    Err(e) => Some(Err(e.into())),
                    Ok((_key, value)) => {
                        match bincode::deserialize::<Allegation>(&value[..]) {
                            Err(e) => Some(Err(e.into())),
                            Ok(entity) => Some(Ok(entity)),
                        }
                    },
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use analogy::Analogy;
    use artifact::FlatText;

    #[test]
    fn init() -> Result<(), Error> {
        let tmpdir = tempfile::tempdir()?;
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath)?;

        let agent = mb.create_agent()?;
        let statement = mb.assert_artifact(FlatText::new("I like turtles".to_string()))?
                          .alledge(&agent)?;

        let category = mb.assert_artifact(FlatText::new("Things that I said".to_string()))?
                         .alledge(&agent)?;

        let allegation = Allegation::new(&agent, Analogy::declare(statement, category))?;
        mb.put_allegation(&allegation)?;

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        mb.dump_json(handle).unwrap();
        Ok(())
    }

    #[test]
    fn load() {
        let dump = r#"{"Artifact":{"id":"AXCjJ5rLDlMbRQihFvx8mA","kind":{"FlatText":{"text":"English words"}}}}
        {"Artifact":{"id":"AXCjJ5tx/kk8rsyDspVjXg","kind":{"FlatText":{"text":"I like turtles"}}}}
        {"Artifact":{"id":"AXCjJ5uHNNfmMyoGab4q7g","kind":{"FlatText":{"text":"Things that I said"}}}}
        {"Allegation":{"id":"AXCjJ5ueWgGaXe7lIpctyg","by":"hN3uQFEAPwYBu10I/KvQtQ","analogy":{"id":"AXCjJ5ue6smgJ6Y8dxmlvw","concept":{"members":["AXCjJ5tx/kk8rsyDspVjXg"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXCjJ5uHNNfmMyoGab4q7g"],"spread_factor":0.0}}}}
        {"Agent":{"Keyed":{"pubkey":[132,221,238,64,81,0,63,6,1,187,93,8,252,171,208,181,61,72,95,246,235,30,7,28,218,34,249,119,152,0,39,33]}}}
        {"Agent":{"Keyed":{"pubkey":[147,251,227,23,240,69,73,16,226,8,208,189,132,200,122,25,142,83,53,44,239,65,254,156,156,146,157,91,230,30,60,214]}}}"#;
        let cursor = std::io::Cursor::new(dump);

        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        mb.load_json(cursor).unwrap();
    }
}
