use crate::{
    allegation::AllegationId,
    error::MBError,
    Analogy,
    MindBase,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;

// QUESTION: What is an "uncertainty budget" and how can it help us?
// TODO 2 - create a Context object that contains a lossy lookup of Concepts on a rolling basis

// TODO 2 - rename Concept -> Symbol and AllegationID to... Atom?
/// Pointer to a region within Semantic/Knowledge-Space
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Concept {
    // # how would the agent know which entities they are referring to?
    // # I suppose the UI could remember a list of entities which are being
    // # converged in the rendering. Not really in love with the fact that
    // # the Agent has to pick specific entities as a representative sample
    // # Of the cluster they're actually referring to, but it will suffice
    // # for now I think.
    /// A list of entities which serve as a representative sample of the K-Space cluster
    pub members:       Vec<AllegationId>,
    // TODO 4 - update members to include a "weight" for each allegation id.
    pub spread_factor: f32,
    /* # Here's a slightly different way, but still not great
     * # median_entity: Entity,
     * # radius: Float */
}

impl fmt::Display for Concept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.members.iter().map(|e| format!("{}", e)).collect();
        write!(f, "[{}]", parts.join(","))
    }
}

impl Concept {
    pub fn is_subjective(&self, _mb: &MindBase) -> Result<bool, MBError> {
        unimplemented!()
    }

    pub fn is_intersubjective(&self, _mb: &MindBase) -> Result<bool, MBError> {
        unimplemented!()
    }

    pub fn is_null(&self) -> bool {
        self.members.len() == 0
    }

    pub fn count(&self) -> usize {
        self.members.len()
    }

    pub fn extend(&mut self, allegation_id: AllegationId) {
        self.members.push(allegation_id)
    }

    /// Create a new concept which is analagous to this concept, but consists of only a single allegation
    pub fn surrogate() -> Concept {
        // TODO 2

        // let surrogate = mb.alledge(Unit)?;
        // mb.alledge(Analogy::declarative(surrogate.subjective(), apple_ground_symbol))?;

        unimplemented!()
    }

    // TODO 4 - make this return a match score rather than just bool
    pub fn intersects(&self, other: &Concept) -> bool {
        // TODO 4 - make this a lexicographic comparison rather than a nested loop (requires ordering of .members)

        for member in self.members.iter() {
            if other.members.contains(member) {
                return true;
            }
        }
        false
    }

    pub fn intersection(&self, other: &Concept) -> Vec<AllegationId> {
        // TODO 4 - make this a lexicographic comparison rather than a nested loop (requires ordering of .members)
        let mut out: Vec<AllegationId> = Vec::new();
        for member in self.members.iter() {
            if other.members.contains(member) {
                out.push(member.clone());
            }
        }

        out
    }

    /// Narrow the Symbols in this concept to include only those
    pub fn narrow_by(&mut self, mb: &MindBase, test_memberof: &Concept) -> Result<(), MBError> {
        if self.is_null() {
            // Can't pull over any further
            return Ok(());
        }

        // We're looking for Analogies mentioning the memberof Concept which mention the allegations in our concept

        use crate::allegation::Body;

        let mut members: Vec<AllegationId> = Vec::new();

        // Old
        // Very inefficient. Looping over ALL allegations in the system
        // Searching for Analogies which point (intersectionally) to this concept
        for allegation in mb.allegation_iter() {
            let allegation = allegation?;

            match allegation.1.body {
                Body::Analogy(Analogy { ref left,
                                        ref right,
                                        ref confidence, }) => {
                    // Are you talking about me? (what subset of me are you talking about?)

                    // TODO 2 - Stop allocing a Vec for every intersection test. This is crazy inefficient
                    let overlap = self.intersection(left);
                    if overlap.len() > 0 {
                        // SO YES: self is at least minimally the subject of this Analogy
                        // (I think any overlap should suffice, but the narrowed concept should be the
                        // intersection of these concepts)

                        if right.intersects(test_memberof) {
                            for passing_member in overlap {
                                if !members.contains(&passing_member) {
                                    members.push(passing_member)
                                    // ANALYZE THIS to see if we can achieve it directly with the inverted index
                                }
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        self.members = members;

        Ok(())
    }
}
