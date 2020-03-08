mod agent;
mod allegation;
mod analogy;
mod artifact;
mod concept;
mod error;
mod genesis;
mod util;
mod xport;

pub use self::{
    agent::{
        Agent,
        AgentId,
    },
    allegation::{
        Allegation,
        AllegationId,
    },
    analogy::Analogy,
    artifact::{
        ArtifactId,
        FlatText,
    },
    error::Error,
};
use artifact::Artifact;
use core::marker::PhantomData;
use serde::de::DeserializeOwned;
use sled::IVec;
use util::TryFromBytes;

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
    pub fn put_allegation(&self, allegation: &Allegation) -> Result<AllegationId, Error> {
        let encoded: Vec<u8> = bincode::serialize(&allegation).unwrap();

        let id = allegation.id().clone();
        self.allegations.insert(id.as_bytes(), encoded)?;
        self.allegations.flush()?;

        Ok(id)
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

    pub fn put_artifact<T>(&self, artifact: T) -> Result<ArtifactId, Error>
        where T: Into<Artifact>
    {
        let artifact: Artifact = artifact.into();
        // TODO 5 - consider whether we want to validate this against an allegation in order to store it,
        // or if we just want to take what we are given. How do we prevent feeding in random junk?
        // It probably doesn't make sense to store the ArifactId in the storage buffer, because it'll be used as the storage key.
        // Does this mean we also shouldn't include it in the network buffer?
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

    pub fn alledge_artifact<T>(&self, agent: &Agent, artifact: T) -> Result<AllegationId, Error>
        where T: Into<crate::artifact::Artifact>
    {
        let artifact_id = self.put_artifact(artifact.into())?;

        let allegation = Allegation::new(agent, crate::allegation::Body::Artifact(artifact_id))?;
        self.put_allegation(&allegation)
    }

    fn artifact_iter(&self) -> Iter<ArtifactId, Artifact> {
        Iter { iter:         self.artifacts.iter(),
               phantomkey:   PhantomData,
               phantomvalue: PhantomData, }
    }

    fn allegation_iter(&self) -> Iter<AllegationId, Allegation> {
        Iter { iter:         self.allegations.iter(),
               phantomkey:   PhantomData,
               phantomvalue: PhantomData, }
    }
}

struct Iter<K, V> {
    iter:         sled::Iter,
    phantomkey:   std::marker::PhantomData<K>,
    phantomvalue: std::marker::PhantomData<V>,
}

impl<K, V> Iterator for Iter<K, V>
    where K: std::convert::TryFrom<IVec>,
          V: DeserializeOwned
{
    type Item = Result<(K, V), crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // Did we find it?
        match self.iter.next() {
            // End of the road
            None => None,

            // We got one
            Some(retrieval) => {
                use std::convert::TryInto;
                match retrieval {
                    Err(e) => Some(Err(e.into())),
                    Ok((key, value)) => {
                        let k: K = match key.try_into() {
                            Ok(k) => k,
                            Err(_) => return Some(Err(Error::TryFromSlice)),
                        };
                        let v: V = match bincode::deserialize::<V>(&value[..]) {
                            Ok(v) => v,
                            Err(e) => return Some(Err(Error::Bincode(e))),
                        };

                        Some(Ok((k, v)))
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
        let statement = mb.alledge_artifact(&agent, FlatText::new("I like turtles".to_string()))?;
        let category = mb.alledge_artifact(&agent, FlatText::new("Things that I said".to_string()))?;

        let allegation = Allegation::new(&agent, Analogy::declare(statement.narrow(), category.narrow()))?;
        mb.put_allegation(&allegation)?;

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        crate::xport::dump_json(&mb, handle).unwrap();
        Ok(())
    }

    #[test]
    fn load() {
        let dump = r#"{"Artifact":["JoknyHnm5yhldjHDMT8gE3IihPQ61OWznrkZqd83h9Q",{"FlatText":{"text":"Things that I said"}}]}
        {"Artifact":["VTZwkrIDfCExVen33D7jNX+a5WRqj+BRtzql3AcWfbA",{"FlatText":{"text":"I like turtles"}}]}
        {"Artifact":["ZneF/bMFv7Fx4r4eU5NTXJPPBSxrUzWLLZ+jFcm8GAs",{"FlatText":{"text":"English words"}}]}
        {"Allegation":["AXC4CwQMGexNcUI16LGEBg",{"id":"AXC4CwQMGexNcUI16LGEBg","agent_id":{"Keyed":{"pubkey":[61,87,75,219,56,163,250,184,22,56,135,245,60,69,241,154,143,251,177,53,180,208,242,83,156,145,59,190,106,230,28,141]}},"body":{"Artifact":"VTZwkrIDfCExVen33D7jNX+a5WRqj+BRtzql3AcWfbA"},"signature":"ipudz5kAWVqiWyaYljLF/CXqYqowdpwMH/b2D4CoqeEo3TZL+6wGENmxcTy2+Bx5ybp+5WWduK1p0f/I57jzCQ"}]}
        {"Allegation":["AXC4CwQr3jbbOjtkmCBAqQ",{"id":"AXC4CwQr3jbbOjtkmCBAqQ","agent_id":{"Keyed":{"pubkey":[61,87,75,219,56,163,250,184,22,56,135,245,60,69,241,154,143,251,177,53,180,208,242,83,156,145,59,190,106,230,28,141]}},"body":{"Artifact":"JoknyHnm5yhldjHDMT8gE3IihPQ61OWznrkZqd83h9Q"},"signature":"XBhndD7rqkzFixkOad8MzKHqJh667PQkoTz524Ei4JNNVsGf0p/Ttj4itPJSHSawJhrD8YaN6LGYdIo0mkivAg"}]}
        {"Allegation":["AXC4CwQ+4nm3NlliFPoM2A",{"id":"AXC4CwQ+4nm3NlliFPoM2A","agent_id":{"Keyed":{"pubkey":[61,87,75,219,56,163,250,184,22,56,135,245,60,69,241,154,143,251,177,53,180,208,242,83,156,145,59,190,106,230,28,141]}},"body":{"Analogy":{"concept":{"members":["AXC4CwQMGexNcUI16LGEBg"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXC4CwQr3jbbOjtkmCBAqQ"],"spread_factor":0.0}}},"signature":"1EJZn/O2SV+0C0mNlQQ1E9SI+zGO4z/2t3Fbs+5Wq2WHeijBl1ZTH4UaVRtRIthKxZ61GHZ1C24xVBWPJMpXDg"}]}"#;
        let cursor = std::io::Cursor::new(dump);

        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        crate::xport::load_json(&mb, cursor).unwrap();
    }
}
