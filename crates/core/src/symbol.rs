use crate::{claim::ClaimId, error::MBError, Analogy, MindBase};
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, fmt};

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
    pub atoms: Vec<Atom>,
    // TODO 4 - update members to include a "weight" for each allegation id.
    pub spread_factor: f32,
    /* # Here's a slightly different way, but still not great
     * # median_entity: Entity,
     * # radius: Float */
}

#[derive(Serialize, Deserialize, PartialEq, Ord, Eq, PartialOrd, Debug, Clone)]
pub enum Spin {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, PartialEq, Ord, Eq, PartialOrd, Debug, Clone)]
pub struct Atom {
    id: ClaimId,
    spin: Spin,
}

// TODO 1 - rename Atom->DirectedAtom, and Allegation->Atom
impl Atom {
    pub fn id(&self) -> &ClaimId {
        &self.id
    }

    pub fn up(id: ClaimId) -> Self {
        Self { id, spin: Spin::Up }
    }

    pub fn down(id: ClaimId) -> Self {
        Self { id, spin: Spin::Down }
    }

    pub fn with_spin(spin: Spin, id: ClaimId) -> Self {
        Self { id, spin }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.spin {
            Spin::Up => write!(f, "{}↑", self.id),
            Spin::Down => write!(f, "{}↓", self.id),
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
    pub fn new(mut unsorted_atoms: Vec<Atom>) -> Self {
        assert!(unsorted_atoms.len() > 0);

        let mut atoms: Vec<Atom> = Vec::new();

        for a in unsorted_atoms.drain(..) {
            match atoms.binary_search(&a) {
                Ok(_) => {} // duplicate
                Err(i) => atoms.insert(i, a),
            }
        }

        Symbol {
            atoms,
            spread_factor: 0.0,
        }
    }

    /// User promises that this vec is lexically sorted by id
    pub fn new_from_sorted_atoms(atoms: Vec<Atom>) -> Self {
        assert!(atoms.len() > 0);

        Symbol {
            atoms,
            spread_factor: 0.0,
        }
    }

    pub fn new_option(atoms: Vec<Atom>) -> Option<Self> {
        if atoms.len() == 0 {
            None
        } else {
            Some(Symbol {
                atoms,
                spread_factor: 0.0,
            })
        }
    }

    pub fn new_from_vec(vec: Vec<u8>) -> Option<Self> {
        // TODO 1 - handle Atom vs DirectedAtom. wrongly assuming that these are Spin::Up
        let atoms: Vec<Atom> = vec
            .chunks_exact(16)
            .map(|c| Atom::up(ClaimId::from_bytes(c.try_into().unwrap())))
            .collect();
        Self::new_option(atoms)
    }

    pub fn as_vec(&self) -> Option<Vec<u8>> {
        if self.atoms.len() == 0 {
            None
        } else {
            // Some(self.atoms.iter().map(|a| a.id()).concat())
            unimplemented!()
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
        match self.atoms.binary_search(&atom) {
            Ok(_) => {} // duplicate
            Err(i) => self.atoms.insert(i, atom),
        }
    }

    /// Create a new symbol which is analagous to this symbol, but consists of only a single allegation
    pub fn surrogate() -> Symbol {
        // TODO 2

        // let surrogate = mb.alledge(Unit)?;
        // mb.alledge(Analogy::declarative(surrogate.subjective(), apple_ground_symbol))?;

        unimplemented!()
    }

    pub fn intersects(&self, other: &Symbol) -> bool {
        let mut a_iter = self.atoms.iter();
        let mut b_iter = other.atoms.iter();

        let mut a = match a_iter.next() {
            Some(v) => v,
            None => {
                return false;
            }
        };

        let mut b = match b_iter.next() {
            Some(v) => v,
            None => {
                return false;
            }
        };

        use std::cmp::Ordering::*;
        loop {
            match a.cmp(b) {
                Less => {
                    a = match a_iter.next() {
                        Some(x) => x,
                        None => return false,
                    };
                }
                Greater => {
                    b = match b_iter.next() {
                        Some(x) => x,
                        None => return false,
                    };
                }
                Equal => return true,
            }
        }
    }

    pub fn intersection(&self, other: &Symbol) -> Vec<Atom> {
        let mut out: Vec<Atom> = Vec::new();

        let mut a_iter = self.atoms.iter();
        let mut b_iter = other.atoms.iter();

        let mut a = match a_iter.next() {
            Some(v) => v,
            None => {
                return out;
            }
        };

        let mut b = match b_iter.next() {
            Some(v) => v,
            None => {
                return out;
            }
        };

        use std::cmp::Ordering::*;
        loop {
            match a.cmp(b) {
                Less => {
                    a = match a_iter.next() {
                        Some(x) => x,
                        None => return out,
                    };
                }
                Greater => {
                    b = match b_iter.next() {
                        Some(x) => x,
                        None => return out,
                    };
                }
                Equal => {
                    out.push(a.clone());
                }
            }
        }

        out
    }

    /// Narrow the Symbols in this symbol to include only those
    pub fn narrow_by(&mut self, mb: &MindBase, test_memberof: &Symbol) -> Result<(), MBError> {
        // We're looking for Analogies mentioning the memberof Symbol which mention the allegations in our symbol

        use crate::claim::Body;

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
                }
                _ => {}
            }
        }

        self.atoms = members;

        Ok(())
    }

    pub fn left_right(&self, mb: &MindBase) -> Result<Option<(Symbol, Symbol)>, MBError> {
        // TODO 3 - optimize this. Presumably via indexing?

        // smush all the analogies together to form a left symbol and a right symbol which is the composition of their (Spin
        // Up/Down aware) branches

        let mut left: Vec<Atom> = Vec::new();
        let mut right: Vec<Atom> = Vec::new();

        for atom in self.atoms.iter() {
            use crate::claim::Body;
            match mb.get_allegation(atom.id())? {
                None => return Err(MBError::TraversalFailed),
                Some(a) => {
                    if let Body::Analogy(analogy) = a.body {
                        match atom.spin {
                            Spin::Up => {
                                left.extend(analogy.left.atoms);
                                right.extend(analogy.right.atoms);
                            }
                            Spin::Down => {
                                left.extend(analogy.right.atoms);
                                right.extend(analogy.left.atoms);
                            }
                        }
                    }
                }
            }
        }

        Ok(Some((Symbol::new(left), Symbol::new(right))))
    }

    pub fn contents_buf(&self, mb: &MindBase, buf: &mut String, depth: usize) -> Result<(), MBError> {
        use std::fmt::Write;
        write!(buf, "[").unwrap();

        let mut seen = false;

        // [ A([]) :  B -> [] ]
        for atom in self.atoms.iter() {
            if seen {
                write!(buf, ",").unwrap();
            }
            seen = true;

            match atom.spin {
                Spin::Up => write!(buf, "{}↑", atom.id).unwrap(),
                Spin::Down => write!(buf, "{}↓", atom.id).unwrap(),
            };

            if depth > 0 {
                use crate::claim::Body;
                match mb.get_allegation(atom.id())? {
                    None => return Err(MBError::TraversalFailed),
                    Some(a) => {
                        write!(buf, "( ").unwrap();
                        if let Body::Analogy(analogy) = a.body {
                            analogy.left.contents_buf(mb, buf, depth - 1)?;
                            write!(buf, " : ").unwrap();
                            analogy.right.contents_buf(mb, buf, depth - 1)?;
                        }
                        write!(buf, " )").unwrap();
                    }
                }
            }
        }

        write!(buf, "]").unwrap();

        Ok(())
    }
}
