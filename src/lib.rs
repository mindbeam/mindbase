mod error;
mod types;
pub use self::error::Error;
pub use self::types::*;

pub struct MindBase {
    entities: sled::Tree,
}

impl MindBase {
    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        println!("OPEN: {:#?}", pathbuf);
        let db = sled::open(pathbuf.as_path())?;

        let me = Self::new(db)?;
        Ok(me)
    }
    fn new(db: sled::Db) -> Result<Self, Error> {
        let entities = db.open_tree("entities")?;

        Ok(MindBase { entities })
    }

    fn write_entity(&self, entity: &Entity) -> Result<(), Error> {
        let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();

        self.entities.insert(&entity.id.0, encoded)?;
        self.entities.flush()?;

        Ok(())
    }

    #[allow(unused)]
    pub fn make_agent(&self) -> Result<Entity, Error> {
        let entity = Entity {
            id: EntityId::new(),
            kind: EntityKind::Agent(Agent {
                pubkey: *b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            }),
        };

        self.write_entity(&entity)?;

        Ok(entity)
    }

    #[allow(unused)]
    pub fn make_artifact(&self, artifact: Artifact) -> Result<Entity, Error> {
        let entity = Entity {
            id: EntityId::new(),
            kind: EntityKind::Artifact(artifact),
        };

        self.write_entity(&entity)?;

        Ok(entity)
    }

    #[allow(unused)]
    pub fn allege(&self, agent: &Entity, analogy: Analogy) -> Result<Entity, Error> {
        let entity = Entity {
            id: EntityId::new(),
            kind: EntityKind::Allegation(Allegation {
                by: agent.id.clone(),
                analogy,
            }),
        };

        self.write_entity(&entity)?;

        Ok(entity)
    }

    fn entity_iter(&self) -> Iter {
        Iter {
            iter: self.entities.iter(),
        }
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
            Some(retrieval) => match retrieval {
                Err(e) => Some(Err(e.into())),
                Ok((_key, value)) => match bincode::deserialize::<Entity>(&value[..]) {
                    Err(e) => Some(Err(e.into())),
                    Ok(entity) => Some(Ok(entity)),
                },
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

        let agent = mb.make_agent().unwrap();
        let statement = mb
            .make_artifact(Artifact::FlatText(FlatText {
                text: "I like turtles".to_string(),
            }))
            .unwrap();

        let category = mb
            .make_artifact(Artifact::FlatText(FlatText {
                text: "Things that I said".to_string(),
            }))
            .unwrap();

        let _allegation = mb
            .allege(
                &agent,
                Analogy::declare(statement.narrow_concept(), category.narrow_concept()),
            )
            .unwrap();

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        mb.dump_json(handle).unwrap();
    }

    #[test]
    fn load() {
        let dump = r#"{"id":"AXCe1JS3IsTxGxFx4zdjvw","kind":{"Agent":{"pubkey":[97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97,97]}}}
                            {"id":"AXCe1JTSg8mTwyw+4SpGKg","kind":{"Artifact":{"FlatText":{"text":"I like turtles"}}}}
                            {"id":"AXCe1JTnGdQjh3hvh0ryUA","kind":{"Artifact":{"FlatText":{"text":"Things that I said"}}}}
                            {"id":"AXCe1JUABrhgAnAt2qrOog","kind":{"Allegation":{"by":"AXCe1JS3IsTxGxFx4zdjvw","analogy":{"concept":{"members":["AXCe1JTSg8mTwyw+4SpGKg"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXCe1JTnGdQjh3hvh0ryUA"],"spread_factor":0.0}}}}}"#;
        let cursor = std::io::Cursor::new(dump);

        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        mb.load_json(cursor).unwrap();
    }
}
