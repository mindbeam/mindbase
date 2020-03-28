use crate::{
    allegation::AllegationId,
    error::Error,
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
    pub fn is_subjective(&self, _mb: &MindBase) -> Result<bool, Error> {
        unimplemented!()
    }

    pub fn is_intersubjective(&self, _mb: &MindBase) -> Result<bool, Error> {
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
    pub fn matches(&self, other: &Concept) -> bool {
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
        let mut out = Vec::new();
        for member in self.members.iter() {
            if other.members.contains(member) {
                out.push(member.clone());
            }
        }

        out
    }

    /// Narrow the Symbols in this concept to include only those
    pub fn narrow_by(&mut self, mb: &MindBase, test_memberof: &Concept) -> Result<(), Error> {
        if self.is_null() {
            // Can't pull over any further
            return Ok(());
        }

        // We're looking for Analogies mentioning the memberof Concept which mention the allegations in our concept

        use crate::allegation::Body;

        let mut members: Vec<AllegationId> = Vec::new();

        // New - uses the inverted index of Analogy.subject.members Analogy.allegation_id
        // maintained by put_allegation() builds an inverted
        for member in self.members.iter() {
            if let Some(list_of_referring_analogy_ids) = mb.analogy_rev.get(member)? {
                // This is the ID of the actual Analogy. NOT the Analogy memberOf allegation_ids
                for analogy_id in list_of_referring_analogy_ids.chunks(16) {
                    // mb.get()
                    // LEFT OFF HERE
                    // Seems kinda dumb to have to look up each analogy
                    // Can we put the memberof symbols directly into the inverted index?
                    // need to analyze the part labeled ANALYZE THIS
                }
            }
        }

        // Old
        // Very inefficient. Looping over ALL allegations in the system
        // Searching for Analogies which point (intersectionally) to this concept
        for allegation in mb.allegation_iter() {
            let allegation = allegation?;

            match allegation.1.body {
                Body::Analogy(Analogy { ref subject,
                                        // ref confidence,
                                        ref memberof,
                                        .. }) => {
                    // Are you talking about me? (what subset of me are you talking about?)

                    // TODO 2 - Stop allocing a Vec for every intersection test. This is crazy inefficient
                    let overlap = self.intersection(subject);
                    if overlap.len() > 0 {
                        // SO YES: self is at least minimally the subject of this Analogy
                        // (I think any overlap should suffice, but the narrowed concept should be the
                        // intersection of these concepts)

                        if memberof.matches(test_memberof) {
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
