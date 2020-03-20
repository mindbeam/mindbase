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

    pub fn concept_filter_allegations_by<'a, F>(&'a self, f: F) -> Result<Concept, Error>
        where F: Fn(&Allegation) -> bool
    {
        let mut members = Vec::new();

        for allegation in self.allegation_iter() {
            let allegation = allegation?;
            if f(&allegation.1) {
                members.push(allegation.0);
            }
        }

        Ok(Concept { members,
                     spread_factor: 0.0 })
    }

    /// For an ordered list of artifacts, we want to try to resolve upon the most precise conceptual definition possible, and
    /// arrive at a "ground symbol" which is meaningful to a given agent. This agent ascribes to a list of "grounding agents",
    /// which the agent trusts implicitly as a source of ground symbols. This list of grounding agents should generally include
    /// the agent itself, plus any "genesis" or "neighbor" agents to which the user chooses to ascribe.
    ///
    /// The genesis/neighbor agents are important, because they represent the starting point of common, culturally originated
    /// defintions in the form of "default" Analogies, which the agent would otherwise have to define for themselves. The
    /// agent in question could theoretically define all of this themselves, but it would be very time consuming, and
    /// crucially, it would impede rather than seed convergence with their neighbors - unless those neighbors first accepted said
    /// agent to be a grounding/neighbor agent. This is of course the goal: that you should ascribe, at least in part, to the set
    /// of definitions which is provided by your neighbor. This is because it reflects ontological alignments which exist in
    /// the real world, at least to some degree.
    ///
    /// This list of artifacts is taken to be a single thread of a taxonomy. Each artifact is initially translated into
    /// the the broadest possible Concept which is inclusive of _all_ potential interpretations of that artifact.
    /// The initial Concept of that taxonomy is not able to be narrowed, but the subsequent concepts in the taxonomy are narrowed
    /// to include only those which are alledged to be in the category of the parent by one of the grounding/neighbor agents.
    ///
    /// This in theory should allow us to resolve upon a single concept which is believed to be meaningful to that agent based on
    /// the artifacts they posess. This is our interface between the physical world, and the perpetually-convergent ontological
    /// continuum we hope to create with mindbase.
    pub fn get_ground_symbol<A>(&self, artifacts: Vec<A>) -> Result<Concept, Error>
        where A: Into<crate::artifact::Artifact>
    {
        let mut search_chain = Vec::with_capacity(artifacts.len());
        for a in artifacts.into_iter() {
            let artifact_id = self.put_artifact(a.into())?;
            search_chain.push(artifact_id);
        }

        use crate::{
            allegation::{
                Body,
                ALLEGATION_ID_SERIALIZED_SIZE,
            },
            util::AsBytes,
        };

        let gs_agents = self.ground_symbol_agents.lock().unwrap();
        let mut last_concept: Option<Concept> = None;
        for search_artifact_id in search_chain {
            // TODO 2 change this to be indexed
            let mut concept = self.concept_filter_allegations_by(|a| {
                                      gs_agents.contains(&a.agent_id)
                                      && match &a.body {
                                          Body::Artifact(artifact_id) => *artifact_id == search_artifact_id,
                                          _ => false,
                                      }
                                  })?;

            if let Some(ref last_concept) = last_concept {
                concept.narrow_by(self, last_concept);
            }

            // None of our ground/neighbor agents have declared this taxonomic/analogic relationship before
            // But we are implicitly doing so now. Lets extend our concept with a new allegation corresponding to a new atom of
            // meaning, and ALSO define that meaning by alledging
            if concept.is_null() {
                // Extend this with a new allegation so we can continue
                // We are doing this because the caller is essentially saying that there is a taxonomic relationship between
                // subsequent allegations
                concept.extend(self.alledge(search_artifact_id)?.id().clone());

                if let Some(parent) = last_concept {
                    self.alledge(Analogy::declarative(concept.clone(), parent))?;
                }
            }

            last_concept = Some(concept);
        }

        Ok(last_concept.unwrap())

        // let mut members = Vec::new();
        // for agent_id in self.ground_symbol_agents.lock().unwrap().iter() {
        //     let mut key: Vec<u8> = Vec::with_capacity(64);

        //     key.extend_from_slice(&agent_id.as_bytes()[..]);
        //     key.extend_from_slice(artifact_id.as_ref());

        //     if let Some(vector) = self.allegation_rev.get(&key[..])? {
        //         members.extend(vector.chunks_exact(ALLEGATION_ID_SERIALIZED_SIZE)
        //                              .map(|c| AllegationId((&c[..]).try_into().unwrap())))
        //     }
        // }

        // if members.len() == 0 {
        //     return Ok(None);
        // } else {
        //     return Ok(Some(Concept { members,
        //                              spread_factor: 0.0 }));
        // }
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

    use inverted_index_util::entity_list::{
        insert_entity_immut,
        ImmutResult,
    };
    use typenum::consts::U16;

    Some(match last_bytes {
             Some(prior) => {
                 match insert_entity_immut::<U16>(prior, op_bytes) {
                     ImmutResult::Changed(newvec) => newvec,
                     ImmutResult::Unchanged => prior.to_vec(),
                 }
             },
             None => op_bytes.to_vec(),
         })
}

#[cfg(test)]
mod tests {
    use crate::*;
    use analogy::Analogy;
    use artifact::{
        text,
        FlatText,
    };

    #[test]
    fn alice() -> Result<(), Error> {
        // A world of analogies...
        //
        // Here's a random example of an analogy: A(your flu) was just like B(my surgical recovery) – in that we were C(stuck in
        // bed for a week). Analogically, both A and B are in the category of C.
        //
        // We get to define the events and the categories. Those categories may themselves be analagous to other categories which
        // were defined by the same agent, or another agent. These analogies allow us to compare and contrast things.
        // What's a category? Everything is a category. Everything in the system starts out as a singleton category, and is then
        // "pluralized" as new allegations pile up, accusing other things of being a member of it.
        //
        // The question is: how do we represent predicates? Things like "was just like", and "in that we were", etc
        //
        // Depending on how you want to look at it, you could say that:
        // * Mindbase has exactly one predicate "is a member of", which is simply implicit in every Analogy
        // or
        // * Mindbase has an infinite number of predicates which you can define – Buttt, they're fused to the object
        //
        // So, we get "is in the category of" for free with each analogy.
        // For a statement like "The pan is hot" we would think of this as:
        // [the pan] (is in the category of) [things that are hot]
        // Connecting words like "things that are" can generally be discarded, provided they are referring to the subject.
        // If the connecting words _do_ in fact change the meaning, then either the subject or the object should be recursively
        // expanded to reflect that meaning.
        //
        // # Why not subject-predicate-object triples?
        // * Because they converge poorly - (speculation)
        // * Because it externalizes the semantics of the predicate to the user
        // * Because the event of jumping into the lake is itself a discrete constituent of the
        // [Alice [jumped into] [the lake]]
        //
        // * What is [jumped-into]ing, and how does it correlate to jumping?
        // * How do we determine which type of jumping it's related to?
        //
        // [Alice [[jumped [into the lake]]]
        //
        //
        // ****** TODO 1 ******
        // Follow up on the notion that a knowledge triple~~dependency tree, whereas a category ~~ a constituency tree
        // It feels like there may be something to this
        //
        // TODO 1 - clarify in the code that:
        //  * An allegation/Concept is a category
        //  * That category be automatically expanded based on Analogies defined against it
        //  Q: how do we make it clear to the user that such Analogies are being traversed?
        //  A: we probably don't - if it's done lazily
        //  Q: how many hops do we do for vicarious analogies? [A] <- B <- C = [A,B,C]
        //  A: I think we have to do this lazily, rather than actually materializing this
        //  Q: When and how is that lazy-evaluation performed?

        // Alice said I like turtles
        //
        // If we represent this in a subject,predicate,object notation we get:
        // (Alice, said, (I, like, turtles))
        //
        // If we use an analogical representation:
        // There exists a specific instance of "like"ing - like1
        // There exists a specific instance of "turtles" - turt1
        // turt1 is in the category of like1
        // There exists a specific instance of "I" - self1
        // that
        // turtles are in a specific instance of "like"-itude
        // I is that instance of likitude is
        // Alice is in the category of (
        //            said is in the category of (
        //                 I is in the category of (
        //                                         )
        //                )
        // )
        // (Alice (said (I (like (turtles)))))
        Ok(())
    }

    #[test]
    // fn apple() -> Result<(), Error> {
    //     let tmpdir = tempfile::tempdir()?;
    //     let tmpdirpath = tmpdir.path();
    //     let mb = MindBase::open(&tmpdirpath)?;

    //     // Lets suppose that Alice makes a statement about apples. Lets record that having happened.
    //     let alice_statement = mb.alledge(text("I love apples"))?;

    //     // Now, lets also use NLP to parse this statement:
    //     //  NP[I]  VP[love apples]
    //     // PRP[I] VBP[love] NP [apples]
    //     //
    //     // Note: these derrived Artifacts are related to the original artifact of alice's statement.
    //     // TODO 2 - How should the system alledge that these are related, and that it wasn't actually alice who broke them down
    //     // this way?
    //     let _np_i = mb.alledge(text("I"))?;
    //     let _vp_love_apples = mb.alledge(text("love apples"))?;
    //     let prp_i = mb.alledge(text("I"))?;

    //     // vbp = Verb non-3rd person singular present form
    //     let vbp_love = mb.alledge(text("love"))?;
    //     // np = Proper Noun
    //     let np_apples = mb.alledge(text("apples"))?;

    //     // the symbol we define for np_apples is in the category of vbp_love
    //     let apple_love = mb.alledge(Analogy::declarative(np_apples.subjective(), vbp_love.subjective()))?;

    //     // The symbol for Alice's self alledged to be in the category of apple_love
    //     let alice_loves_apples = mb.alledge(Analogy::declarative(prp_i.subjective(), apple_love.subjective()));

    //     // ok, great

    //     // Lets make some apples. These all share the same artifact, but they're different allegations.
    //     // Lets imagine that these are part of an initial set of allegations which is provided by some agent
    //     // early in the growth of the system, in order to prime the pump. Other agents may make redundant and/or similar
    //     // allegations, either because they didn't see these, or didn't understand them, or didn't have the time to correlate
    //     // them.
    //     // let apple_computers = mb.alledge(FlatText::new("Apple"))?;
    //     // let apple_the_fruit = mb.alledge(FlatText::new("Apple"))?;
    //     // let apple_of_my_eye = mb.alledge(FlatText::new("Apple"))?;

    //     // // Lets be a liittle more specific. (Using get_ground_symbol here as a shortcut)
    //     // mb.alledge(Analogy::declarative(apple_computers.subjective(), mb.alledge(text("Corporation"))?.subjective()))?;
    //     // mb.alledge(Analogy::declarative(apple_the_fruit.subjective(), mb.alledge(text("Edible Fruit"))?.subjective()))?;
    //     // mb.alledge(Analogy::declarative(apple_of_my_eye.subjective(), mb.alledge(text("Amorousness"))?.subjective()))?;

    //     // // Look up the "ground symbol" for "Apple" without any additional specificity
    //     // let apple: Concept = mb.get_ground_symbol(vec![text("Apple")])?;
    //     // // It's... all of them? Why? Because meaning is contextual/intersectional.
    //     // // We don't have enough information to narrow it down yet and we should not assume what they meant
    //     // assert_eq!(apple.count(), 3);

    //     // let apple_plural = mb.alledge(text("Plural form of Apple"))?;
    //     // mb.alledge(Analogy::declarative(apples.subjective(), things_i_love.subjective()))?;

    //     // // Lets start out simple. Apple. Which apple are you talking about?
    //     // let fruit = mb.get_ground_symbol(vec![text("Apple")])?;

    //     // // Just for fun, Lets get reeal specific with the biological taxonomy. Note that it's conceivable that this exact
    //     // taxonomy // could also be present which might mean something completely different! While the length of our
    //     // specified // taxonomy makes this a bit less likely, remember that there is nothing magical about these
    //     // artifacts. let malus_domestica1 = mb.get_ground_symbol(vec![text("Domain: Eukarya"),
    //     //                                                  text("Kingdom: Plantae"),
    //     //                                                  text("Phylum: Magnoliophyta"),
    //     //                                                  text("Class: Magnoliopsida"),
    //     //                                                  text("Order: Rosales"),
    //     //                                                  text("Family: Rosaceae"),
    //     //                                                  text("Genus: Malus"),
    //     //                                                  text("Species: Malus domestica"),])?;

    //     // let tree = mb.get_ground_symbol(vec![text("Plant"), text("Tree")])?;
    //     // let fruit = mb.get_ground_symbol(vec![text("Fruit")])?;

    //     // //  text("with an elongated stem or trunk"),
    //     // //  text("has branches and leaves"),
    //     // // mb.alledge(Analogy::declare(malus_domestica1.clone(), tree.clone()))?;
    //     // // text("seed-bearing structure"),
    //     // //                                       text("of a flowering plant"),
    //     // //                                       text("formed from the ovary after flowering")

    //     // // text("Apple");
    //     // // text("Fruit of the");;

    //     // let malus_domestica2 = mb.get_ground_symbol(vec![text("Kingdom: Plantae"),
    //     //                                                  text("Clade: Tracheophytes"),
    //     //                                                  text("Clade: Angiosperms"),
    //     //                                                  text("Clade: Eudicots"),
    //     //                                                  text("Clade: Rosids"),
    //     //                                                  text("Order: Rosales"),
    //     //                                                  text("Family: Rosaceae"),
    //     //                                                  text("Genus: Malus"),
    //     //                                                  text("Species: M. domestica"),])?;

    //     // // text("Apple");
    //     // // text("Fruit of the");

    //     // assert_eq!(malus_domestica1, malus_domestica2);
    //     Ok(())
    // }
    #[test]
    fn fridays() -> Result<(), Error> {
        let tmpdir = tempfile::tempdir()?;
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath)?;

        // Next Friday
        let f1 = mb.alledge(text("Friday"))?.subjective();

        // The abstract concept of Friday
        let f2 = mb.alledge(text("Friday"))?.subjective();

        // The person named Friday
        let f3 = mb.alledge(text("Friday"))?.subjective();

        let fut = mb.alledge(text("Days which are in the near future"))?.subjective();
        let dow = mb.alledge(text("Abstract day of the week"))?.subjective();
        let per = mb.alledge(text("Names for a person"))?.subjective();

        mb.alledge(Analogy::declarative(f1, fut))?;
        mb.alledge(Analogy::declarative(f2, dow))?;
        mb.alledge(Analogy::declarative(f3, per))?;

        let friday_person = mb.get_ground_symbol(vec![text("Friday"), text("Names for a person")])?;
        // let names = mb.get_ground_symbols_for_artifact(FlatText::new("Names for a person"))?
        //               .expect("Option");

        // let fridays = fridays.narrow_by(mb, names);
        // println!("{:?}", fridays);
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

        mb.alledge(Analogy::declarative(s1.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::declarative(s2.subjective(), dow.subjective()))?;
        mb.alledge(Analogy::declarative(s3.subjective(), dow.subjective()))?;

        mb.alledge(Analogy::declarative(s1.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::declarative(s2.subjective(), alright.subjective()))?;
        mb.alledge(Analogy::declarative(s3.subjective(), alright.subjective()))?;

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
