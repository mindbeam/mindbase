mod agent;
mod allegation;
mod analogy;
mod artifact;
mod concept;
mod error;
mod genesis;
mod policy;
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
use concept::Concept;
use core::marker::PhantomData;
use policy::Policy;
use serde::de::DeserializeOwned;
use sled::IVec;
use std::{
    convert::TryInto,
    sync::{
        Arc,
        Mutex,
    },
};

// pub mod allegation_capnp {
//     include!(concat!(env!("OUT_DIR"), "/capnp/allegation_capnp.rs"));
// }

pub struct MindBase {
    /// Sig-Addressable store for Entities (EntityId())
    allegations: sled::Tree,

    /// Content-addressable store for artifacts. ArtifactId(Sha512Trunc256)
    artifacts: sled::Tree,

    /// Reverse lookup for all allegations
    allegation_rev: sled::Tree,

    /// Credential storage for all agents we manage
    my_agents: sled::Tree,

    /// I forget why I would actually need known agents
    known_agents: sled::Tree,

    ground_symbol_agents: Arc<Mutex<Vec<AgentId>>>,

    // TODO 1 - inverted index by artifact id / allegation id
    // QUESTION: Should these be two different trees? or one?
    default_agent: Agent,
}

impl MindBase {
    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        let my_agents = db.open_tree("agents")?;
        let artifacts = db.open_tree("artifacts")?;
        let allegations = db.open_tree("allegations")?;
        let allegation_rev = db.open_tree("allegation_rev")?;

        allegation_rev.set_merge_operator(merge_allegation_rev);

        let default_agent = _default_agent(&my_agents)?;
        let known_agents = db.open_tree("known_agents")?;

        let ground_symbol_agents = Arc::new(Mutex::new(vec![default_agent.id()]));

        let me = MindBase { allegations,
                            my_agents,
                            artifacts,
                            known_agents,
                            allegation_rev,
                            ground_symbol_agents,
                            default_agent };

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
        _default_agent(&self.my_agents)
    }

    #[allow(unused)]
    pub fn put_allegation(&self, allegation: &Allegation) -> Result<AllegationId, Error> {
        let encoded: Vec<u8> = bincode::serialize(&allegation).unwrap();

        let id = allegation.id().clone();
        self.allegations.insert(id.as_bytes(), encoded)?;
        self.allegations.flush()?;

        // TODO 2 - convert this into an iterator
        if let Some(rev) = allegation.reverse_lookup() {
            use crate::util::AsBytes;

            // TODO 2 - don't just overwrite this. Implement merge logic
            self.allegation_rev.merge(&rev[..], id.as_bytes());
        }

        Ok(id)
    }

    #[allow(unused)]
    pub fn create_agent(&self) -> Result<Agent, Error> {
        _create_agent(&self.my_agents)
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

    // Alledge an Alledgable thing using the default agent
    pub fn alledge<T>(&self, thing: T) -> Result<Allegation, Error>
        where T: crate::allegation::Alledgable
    {
        thing.alledge(self, &self.default_agent)
    }

    // Alledge an Alledgable thing using specified agent
    pub fn alledge2<T>(&self, agent: &Agent, thing: T) -> Result<Allegation, Error>
        where T: crate::allegation::Alledgable
    {
        thing.alledge(self, agent)
    }

    pub fn alledge_artifact<A>(&self, agent: &Agent, artifact: A) -> Result<AllegationId, Error>
        where A: Into<crate::artifact::Artifact>
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

    pub fn get_ground_symbols_for_artifact<A>(&self, artifact: A) -> Result<Option<Concept>, Error>
        where A: Into<crate::artifact::Artifact>
    {
        use crate::{
            allegation::ALLEGATION_ID_SERIALIZED_SIZE,
            util::AsBytes,
        };

        let artifact_id = self.put_artifact(artifact.into())?;

        let mut members = Vec::new();

        for agent_id in self.ground_symbol_agents.lock().unwrap().iter() {
            let mut key: Vec<u8> = Vec::with_capacity(64);

            key.extend_from_slice(&agent_id.as_bytes()[..]);
            key.extend_from_slice(artifact_id.as_ref());

            if let Some(vector) = self.allegation_rev.get(&key[..])? {
                members.extend(vector.chunks_exact(ALLEGATION_ID_SERIALIZED_SIZE)
                                     .map(|c| AllegationId((&c[..]).try_into().unwrap())))
            }
        }

        if members.len() == 0 {
            return Ok(None);
        } else {
            return Ok(Some(Concept { members,
                                     spread_factor: 0.0 }));
        }
    }

    pub fn add_ground_symbol_agent(&self, agent_id: AgentId) -> Result<(), Error> {
        // TODO 2 - Build the policy system and convert this to a policy
        self.ground_symbol_agents.lock().unwrap().push(agent_id);

        Ok(())
    }

    pub fn ground_symbol<A>(&self, artifact: A) -> Result<Concept, Error>
        where A: Into<crate::artifact::Artifact>
    {
        let ground_agents = self.default_agent.ground_agents();

        unimplemented!()
        // self.allegation_rev.get()
        // TODO 1 - fetch allegations which are:
        // * Made by the specified agent
        // * "contain" all of the specified artifacts. (How?)
    }

    pub fn add_policy(&self, policy: Policy) -> Result<(), Error> {
        unimplemented!()
    }
}

fn _default_agent(my_agents: &sled::Tree) -> Result<Agent, Error> {
    match my_agents.get(b"latest")? {
        None => _create_agent(my_agents),
        Some(pubkey) => {
            match my_agents.get(pubkey)? {
                None => Err(Error::AgentHandleNotFound),
                Some(v) => {
                    let agenthandle = bincode::deserialize(&v)?;
                    Ok(agenthandle)
                },
            }
        },
    }
}

fn _create_agent(my_agents: &sled::Tree) -> Result<Agent, Error> {
    let agent = Agent::new();

    let encoded: Vec<u8> = bincode::serialize(&agent).unwrap();
    my_agents.insert(agent.pubkey().as_bytes(), encoded)?;
    my_agents.insert(b"latest", agent.pubkey().as_bytes())?;
    my_agents.flush()?;

    Ok(agent)
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

fn merge_allegation_rev(_key: &[u8],               // the key being merged
                        last_bytes: Option<&[u8]>, // the previous value, if one existed
                        op_bytes: &[u8]            /* the new bytes being merged in */)
                        -> Option<Vec<u8>> {
    // set the new value, return None to delete

    use crate::allegation::ALLEGATION_ID_SERIALIZED_SIZE;
    let op_bytes: [u8; ALLEGATION_ID_SERIALIZED_SIZE] = op_bytes.try_into().unwrap();

    Some(match last_bytes {
             Some(v) => {
                 let mut chunks: Vec<[u8; ALLEGATION_ID_SERIALIZED_SIZE]> = v.chunks_exact(ALLEGATION_ID_SERIALIZED_SIZE)
                                                                             .map(|v| v.try_into().unwrap())
                                                                             .collect();
                 match chunks.binary_search(&op_bytes) {
                     Ok(_) => {},
                     Err(i) => {
                         chunks.insert(i, op_bytes);
                     },
                 }
                 chunks.concat()
             },
             None => op_bytes.to_vec(),
         })
}

#[cfg(test)]
mod tests {
    use crate::*;
    use analogy::Analogy;
    use artifact::FlatText;

    #[test]
    fn fridays() -> Result<(), Error> {
        let tmpdir = tempfile::tempdir()?;
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath)?;

        // Next Friday
        let f1 = mb.alledge(FlatText::new("Friday"))?.subjective();

        // The abstract concept of Friday
        let f2 = mb.alledge(FlatText::new("Friday"))?.subjective();

        // The person named Friday
        let f3 = mb.alledge(FlatText::new("Friday"))?.subjective();

        let fut = mb.alledge(FlatText::new("Days which are in the near future"))?.subjective();
        let dow = mb.alledge(FlatText::new("Abstract day of the week"))?.subjective();
        let per = mb.alledge(FlatText::new("Names for a person"))?.subjective();

        mb.alledge(Analogy::declare(f1, fut))?;
        mb.alledge(Analogy::declare(f2, dow))?;
        mb.alledge(Analogy::declare(f3, per))?;

        let fridays = mb.get_ground_symbols_for_artifact(FlatText::new("Friday"))?.expect("Option");
        let names = mb.get_ground_symbols_for_artifact(FlatText::new("Names for a person"))?
                      .expect("Option");

        let fridays = fridays.narrow_by(mb, names);
        println!("{:?}", fridays);
        Ok(())
    }
    #[test]
    fn saturday_nights_alright() -> Result<(), Error> {
        let tmpdir = tempfile::tempdir()?;
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath)?;

        // Make the ULID play nice for out-of-order testing
        let dur = std::time::Duration::from_millis(50);

        let s1 = mb.alledge(FlatText::new("Saturday"))?;
        println!("1 {}", s1);
        std::thread::sleep(dur);
        let s2 = mb.alledge(FlatText::new("Saturday"))?;
        println!("2 {}", s2);
        std::thread::sleep(dur);
        let s3 = mb.alledge(FlatText::new("Saturday"))?;

        println!("3 {}", s3);
        std::thread::sleep(dur);
        let f1 = mb.alledge(FlatText::new("Night's alright for fighting"))?;

        // TODO 1 - change these to use grounding_symbol:
        let dow = mb.alledge(FlatText::new("Abstract day of the week"))?;
        let alright = mb.alledge(FlatText::new("Days that are alright for figting in the evening"))?;

        mb.alledge(Analogy::declare(s1.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::declare(s2.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::declare(s3.subjective(), dow.subjective()))?;

        mb.alledge(Analogy::declare(s1.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::declare(s2.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::declare(s3.subjective(), alright.subjective()))?;

        let stdout = std::io::stdout();
        let handle = stdout.lock();

        crate::xport::dump_json(&mb, handle).unwrap();
        Ok(())
    }

    #[test]
    fn load() -> Result<(), std::io::Error> {
        let dump = r#"{"Artifact":["MTEmz+3sCpCnSrrvKJglWvWIEOAJ4Ger7cecqz+/p1I",{"FlatText":{"text":"Days that are alright for figting in the evening"}}]}
        {"Artifact":["QAh0LMMPHQMGJhLNdKH1OasSuCTmS9g2xdViW1gmpJ4",{"FlatText":{"text":"Night's alright for fighting"}}]}
        {"Artifact":["Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc",{"FlatText":{"text":"Saturday"}}]}
        {"Artifact":["ZneF/bMFv7Fx4r4eU5NTXJPPBSxrUzWLLZ+jFcm8GAs",{"FlatText":{"text":"English words"}}]}
        {"Artifact":["4Hc9t2ownv7e+hAfzn2f+36xwqKxZWCGJIxKAGQb2KQ",{"FlatText":{"text":"Abstract day of the week"}}]}
        {"Allegation":["AXDCW2fEtID9DIZzkMgQvg",{"id":"AXDCW2fEtID9DIZzkMgQvg","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"Alan4mqUTsRW/hEoqaJRRo7CAwx20go75qny4AytRS1a8nrEl6NAvorbw8XKTS9J+3BSVF5ybsICVP/HhRMhDQ"}]}
        {"Allegation":["AXDCW2dnN7VS4wpoUWGJMw",{"id":"AXDCW2dnN7VS4wpoUWGJMw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"7QGeYfot/6vGZdG33fYEMsbG+0qm2WQpFyWwWyJYaHECuDypksgl55ozPTi6ye8XDCdgxg1/NwiHpjQjNwacAA"}]}
        {"Allegation":["AXDCW2gsaVltJU2vcQFKuQ",{"id":"AXDCW2gsaVltJU2vcQFKuQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc"},"signature":"ChS23ik9E+UdYCQUbQzmiMg9RjVu8pF2vI9VmVD4vOotEdpUMIUM62rjJ4Ne6cJY5Js2BICB/E7OkKWeSeowAA"}]}
        {"Allegation":["AXDCW2iDLmb47/OVfXxIQg",{"id":"AXDCW2iDLmb47/OVfXxIQg","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"QAh0LMMPHQMGJhLNdKH1OasSuCTmS9g2xdViW1gmpJ4"},"signature":"ltKdtKpe8ZVKFrTOJ4u3C5i6e3Gute2whoSqLTBbz5yedUojIylrxQbXVQDt+rAYSvLOYfZdjKzd2at11qjgDQ"}]}
        {"Allegation":["AXDCW2irj9IxJ/jvXLGmmw",{"id":"AXDCW2irj9IxJ/jvXLGmmw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"4Hc9t2ownv7e+hAfzn2f+36xwqKxZWCGJIxKAGQb2KQ"},"signature":"Rf18ZLD9s4U+kYvqiFOCSwTSvvXOo78/6XxcYM61WYcfYPKSLqYF5nA3gxxqcWNfemqx7S3VRfFSpWv4y5cQAw"}]}
        {"Allegation":["AXDCW2jLdjUVrz/igyanqQ",{"id":"AXDCW2jLdjUVrz/igyanqQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Artifact":"MTEmz+3sCpCnSrrvKJglWvWIEOAJ4Ger7cecqz+/p1I"},"signature":"U0Gh4JJReWjQtlttmdeGAwrD2GvkiBxOFfuVZ/rW85Mo7FXXUYplr7mLsGg43/M9xoBmYFt/FjcSf3QCiNRsCA"}]}
        {"Allegation":["AXDCW2jYkL6w/ha0MYDtow",{"id":"AXDCW2jYkL6w/ha0MYDtow","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2dnN7VS4wpoUWGJMw"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2irj9IxJ/jvXLGmmw"],"spread_factor":0.0}}},"signature":"YHW0oa3AixiAVXUBkUEuFBw52qy/2gfuQoJljyCMrQDc8C3C69uTbarIyfcxJS026qMl/vQCT5JOsjaOZhj/Bg"}]}
        {"Allegation":["AXDCW2j39GJJrNZ9fqXZfQ",{"id":"AXDCW2j39GJJrNZ9fqXZfQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2fEtID9DIZzkMgQvg"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2irj9IxJ/jvXLGmmw"],"spread_factor":0.0}}},"signature":"lnwaN4hP+pEN+Jgnd7EbiPhGIxZE18+iyvAtwNHRyj/7KYxrsMO4EjKl0URn/6AC+7GK0LsS5n6+gaISIpIWBg"}]}
        {"Allegation":["AXDCW2kNM6RcpeAWTOrPWQ",{"id":"AXDCW2kNM6RcpeAWTOrPWQ","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2gsaVltJU2vcQFKuQ"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2irj9IxJ/jvXLGmmw"],"spread_factor":0.0}}},"signature":"42iAcnBidsgBjqCxNiu4gOtjQkNv/s2ih1Ebeg/27xJQwSeUnLeIyS9ztV4zBx3N97pUzvTVmjXboaZv0+Y3CA"}]}
        {"Allegation":["AXDCW2kaMP6SgxqaYmFegA",{"id":"AXDCW2kaMP6SgxqaYmFegA","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2dnN7VS4wpoUWGJMw"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2jLdjUVrz/igyanqQ"],"spread_factor":0.0}}},"signature":"7PModUhhvuM8uV5NS8qTcGC+AKvn6KcSdq4hTo52N2ulmwydzml7wzHg33qKttAq2QyErN8iNCl3V5w7wcZ4Aw"}]}
        {"Allegation":["AXDCW2kmqHwYexaNBfcRrw",{"id":"AXDCW2kmqHwYexaNBfcRrw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2fEtID9DIZzkMgQvg"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2jLdjUVrz/igyanqQ"],"spread_factor":0.0}}},"signature":"cfUYwSBtIb/qyw7kMdXRadz7/RfxrTKh3lvjXoxbMvlcTUAdsXQPMLapSrpBJ1rw7RD+F/C2+5mmv8PEAINvDA"}]}
        {"Allegation":["AXDCW2k4A7LuPwBx3l2hBw",{"id":"AXDCW2k4A7LuPwBx3l2hBw","agent_id":{"pubkey":"rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY"},"body":{"Analogy":{"concept":{"members":["AXDCW2gsaVltJU2vcQFKuQ"],"spread_factor":0.0},"confidence":1.0,"memberof":{"members":["AXDCW2jLdjUVrz/igyanqQ"],"spread_factor":0.0}}},"signature":"dd4fqB3J957G/dP/GUl9lP9ZaTYWqQ5zi5U+3oSniUTOd1rtUX9x6nZENxOa8OnW6571nBRpmyXBOPNnGtDdDg"}]}"#;
        let cursor = std::io::Cursor::new(dump);

        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        crate::xport::load_json(&mb, cursor).unwrap();

        let s1 = AllegationId::from_base64("AXDCW2dnN7VS4wpoUWGJMw")?; // this one second
        let s2 = AllegationId::from_base64("AXDCW2fEtID9DIZzkMgQvg")?; // manipulated the dumpfile above for this one to be recorded first
        let s3 = AllegationId::from_base64("AXDCW2gsaVltJU2vcQFKuQ")?; // this one third

        let s = ArtifactId::from_base64("Wtw2TYgjfvmTVazfM0IDhkfwlJFUZ9w0xhNc1H0ilRc")?;

        let genesis_agent_id = AgentId::from_base64("rKEhipCfl9P3K7+6glZVZi1nnQbxVA9vjloNdWsS0bY")?;
        mb.add_ground_symbol_agent(genesis_agent_id)?;

        // let saturdays = mb.get_ground_symbols_for_artifact(&s)?;
        // assert_eq!(saturdays, Some(vec![s1, s2, s3]));

        Ok(())
    }
}
