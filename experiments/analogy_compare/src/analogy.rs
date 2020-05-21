use super::{
    atom::*,
    symbol::*,
};

use mindbase::util::iter::pairwise_nonrepeating::PairwiseNonrepeating;

pub struct Analogy {
    pub id:  AtomId,
    pub vec: AtomVec,
}

impl Analogy {
    pub fn new(id: AtomId, mut left: Symbol, mut right: Symbol) -> Self {
        let mut vec = AtomVec::new();

        for atom in left.drain(..) {
            vec.insert(atom.transmute_left())
        }
        for atom in right.drain(..) {
            vec.insert(atom.transmute_right())
        }

        Analogy { id, vec }
    }

    pub fn intersect(&self, other: &AtomVec) -> Option<AtomVec> {
        let mut got_l = false;
        let mut got_r = false;

        let mut out = AtomVec::new();

        let mut iter = PairwiseNonrepeating::new(self.vec.iter(), other.iter());

        // Execution plan:
        // * We're comparing all Atoms for both symbols within this analogy to a Two-sided AtomVec containing *Candidate* Atoms
        // * At least one Left Atom and one Right Atom must match in order for the relationship to have any weight at all
        // * We're expanding the set of those atoms which are inferred (Spin-adjusted opposite-side Atoms) with a score based on
        // weighted sum of the matching spin-adjusted same-side matches

        // Question: How do we calculate the scores for Spin-adjusted Same-side matches?

        while let Some((my_atom, compare_atom)) = iter.next() {
            println!("Compare {:?} vs {:?}", my_atom, compare_atom);

            match my_atom.side {
                Left => got_l = true,
                Right => got_r = true,
                Middle => unimplemented!(),
            };

            use Side::*;

            // The problem is: An associative analogy involves two sets of Atoms, Left and Right, which are associated sets, but
            // NOT associable pairwise as atoms. This is because each set represents a symbol in its own right.
            // The allegation is that SymbolA(Atomset A) is associatively correlated to SymbolB(Atomset B)

            // Given a perfect match of left-handed Atoms, all right-handed Atoms can be inferred.
            // Given a PARTIAL match of left-handed Atoms, The entirety of the subset the right is inferred, but with a degree of
            // confidence commensurate with the number of left-handed matches Commensurately, the inverse is true as
            // well of right handed matches

            // Questions:
            // * How does this compose across multiple levels of Associative analogy? Eg ("Smile" : "Mouth") : ("Wink" : "Eye")
            // * How do we represent this partial matching. Presumably via some scoring mechanism

            // TODO use spin to invert this logic
            let updated_compare_atom = match (&my_atom.side, &compare_atom.side) {
                (Left, Left) => my_atom.clone(),
                (Left, Right) => my_atom.clone().invert_spin().invert_side(),
                (Right, Left) => my_atom.clone().invert_spin().invert_side(),
                (Right, Right) => my_atom.clone(),
                _ => unimplemented!(),
            };

            out.insert(updated_compare_atom);
        }

        if got_l && got_r {
            Some(out)
        } else {
            None
        }
    }
}
