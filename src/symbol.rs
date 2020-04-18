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
// TODO 2 - create a Context object that contains a lossy lookup of Symbols on a rolling basis

// TODO 2 - rename Symbol -> Symbol and AllegationID to... Atom?
/// Pointer to a region within Semantic/Knowledge-Space
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Symbol {
    // # how would the agent know which entities they are referring to?
    // # I suppose the UI could remember a list of entities which are being
    // # converged in the rendering. Not really in love with the fact that
    // # the Agent has to pick specific entities as a representative sample
    // # Of the cluster they're actually referring to, but it will suffice
    // # for now I think.
    /// A list of entities which serve as a representative sample of the K-Space cluster
    pub atoms:         Vec<Atom>,
    // TODO 4 - update members to include a "weight" for each allegation id.
    pub spread_factor: f32,
    /* # Here's a slightly different way, but still not great
     * # median_entity: Entity,
     * # radius: Float */
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Atom {
    Up(AllegationId),
    Down(AllegationId),
}

impl Atom {
    pub fn id(&self) -> &AllegationId {
        match self {
            Atom::Up(a) | Atom::Down(a) => a,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Up(a) => write!(f, "↑{}", a),
            Atom::Down(a) => write!(f, "↓{}", a),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.atoms.iter().map(|e| format!("{}", e)).collect();
        write!(f, "[{}]", parts.join(","))
    }
}

impl Symbol {
    pub fn new(atoms: Vec<Atom>) -> Self {
        assert!(atoms.len() > 0);

        Symbol { atoms,
                 spread_factor: 0.0 }
    }

    pub fn new_option(atoms: Vec<Atom>) -> Option<Self> {
        if atoms.len() == 0 {
            None
        } else {
            Some(Symbol { atoms,
                          spread_factor: 0.0 })
        }
    }

    pub fn is_subjective(&self, _mb: &MindBase) -> Result<bool, MBError> {
        unimplemented!()
    }

    pub fn is_intersubjective(&self, _mb: &MindBase) -> Result<bool, MBError> {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        self.atoms.len()
    }

    pub fn extend(&mut self, atom: Atom) {
        self.atoms.push(atom)
    }

    /// Create a new symbol which is analagous to this symbol, but consists of only a single allegation
    pub fn surrogate() -> Symbol {
        // TODO 2

        // let surrogate = mb.alledge(Unit)?;
        // mb.alledge(Analogy::declarative(surrogate.subjective(), apple_ground_symbol))?;

        unimplemented!()
    }

    // TODO 4 - make this return a match score rather than just bool
    pub fn intersects(&self, other: &Symbol) -> bool {
        // TODO 4 - make this a lexicographic comparison rather than a nested loop (requires ordering of .members)

        for member in self.atoms.iter() {
            if other.atoms.contains(member) {
                return true;
            }
        }
        false
    }

    pub fn intersection(&self, other: &Symbol) -> Vec<Atom> {
        // TODO 4 - make this a lexicographic comparison rather than a nested loop (requires ordering of .members)
        let mut out: Vec<Atom> = Vec::new();
        for member in self.atoms.iter() {
            if other.atoms.contains(member) {
                out.push(member.clone());
            }
        }

        out
    }

    /// Narrow the Symbols in this symbol to include only those
    pub fn narrow_by(&mut self, mb: &MindBase, test_memberof: &Symbol) -> Result<(), MBError> {
        // We're looking for Analogies mentioning the memberof Symbol which mention the allegations in our symbol

        use crate::allegation::Body;

        let mut members: Vec<Atom> = Vec::new();

        // Old
        // Very inefficient. Looping over ALL allegations in the system
        // Searching for Analogies which point (intersectionally) to this symbol
        for allegation in mb.allegation_iter() {
            let allegation = allegation?;

            match allegation.1.body {
                Body::Analogy(Analogy { ref left, ref right, .. }) => {
                    // Are you talking about me? (what subset of me are you talking about?)

                    // TODO 2 - Stop allocing a Vec for every intersection test. This is crazy inefficient
                    let overlap = self.intersection(left);
                    if overlap.len() > 0 {
                        // SO YES: self is at least minimally the subject of this Analogy
                        // (I think any overlap should suffice, but the narrowed symbol should be the
                        // intersection of these symbols)

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

        self.atoms = members;

        Ok(())
    }
}
