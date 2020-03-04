mod agent;
mod entity;
mod error;
mod genesis;
pub use self::{
    agent::*,
    entity::*,
    error::Error,
};

pub struct MindBase {
    entities: sled::Tree,
    agents:   sled::Tree,
}

impl MindBase {
    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        let entities = db.open_tree("entities")?;
        let agents = db.open_tree("agents")?;

        let me = MindBase { entities, agents };

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
        match self.agents.get(b"latest")? {
            None => self.create_agent(),
            Some(pubkey) => {
                match self.agents.get(pubkey)? {
                    None => Err(Error::AgentHandleNotFound),
                    Some(v) => {
                        let agenthandle = bincode::deserialize(&v)?;
                        Ok(agenthandle)
                    },
                }
            },
        }
    }

    fn write_entity(&self, entity: &Entity) -> Result<(), Error> {
        let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();

        self.entities.insert(&entity.id(), encoded)?;
        self.entities.flush()?;

        Ok(())
    }

    #[allow(unused)]
    pub fn create_agent(&self) -> Result<Agent, Error> {
        let agenthandle = Agent::new();

        let entity = agenthandle.entity();

        let encoded: Vec<u8> = bincode::serialize(&agenthandle).unwrap();
        self.agents.insert(agenthandle.pubkey().unwrap().as_bytes(), encoded)?;
        self.agents.insert(b"latest", agenthandle.pubkey().unwrap().as_bytes())?;
        self.agents.flush()?;

        self.write_entity(&entity)?;

        Ok(agenthandle)
    }

    #[allow(unused)]
    pub fn make_artifact(&self, kind: ArtifactKind) -> Result<Entity, Error> {
        let entity = Entity::Artifact(Artifact { id: EntityId::new(),
                                                 kind });

        self.write_entity(&entity)?;

        Ok(entity)
    }

    #[allow(unused)]
    pub fn alledge(&self, agenthandle: &Agent, analogy: Analogy) -> Result<Entity, Error> {
        let entity = Entity::Allegation(Allegation { id: EntityId::new(),
                                                     by: agenthandle.entity().id(),
                                                     analogy });

        self.write_entity(&entity)?;

        Ok(entity)
    }

    fn entity_iter(&self) -> Iter {
        Iter { iter: self.entities.iter(), }
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
            let entity: Entity = serde_json::from_str(&line?[..])?;
            self.write_entity(&entity);
        }
        Ok(())
    }
}

struct Iter {
    iter: sled::Iter,
}

impl Iterator for Iter {
    type Item = Result<Entity, crate::Error>;

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
                        match bincode::deserialize::<Entity>(&value[..]) {
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

    #[test]
    fn init() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let agent = mb.create_agent().unwrap();
        let statement = mb.make_artifact(ArtifactKind::FlatText(FlatText { text: "I like turtles".to_string(), }))
                          .unwrap();

        let category = mb.make_artifact(ArtifactKind::FlatText(FlatText { text: "Things that I said".to_string(), }))
                         .unwrap();

        let _allegation = mb.alledge(&agent,
                                     Analogy::declare(statement.narrow_concept(), category.narrow_concept()))
                            .unwrap();

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        mb.dump_json(handle).unwrap();
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
