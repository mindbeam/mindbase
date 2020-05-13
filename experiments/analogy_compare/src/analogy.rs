use super::{
    atom::*,
    symbol::*,
};

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
        let mut a = self.vec.iter();
        let mut b = other.iter();

        let mut got_l = false;
        let mut got_r = false;

        let mut out = AtomVec::new();

        for i in 0..2 {
            let my_atom = a.next().unwrap();
            let compare_atom = b.next().unwrap();
            // println!("Compare {:?} vs {:?}", my_atom, compare_atom);

            // For simplicity, use trailing digits to differentiate different allegations of the same "artifact"
            use regex::Regex;
            let re = Regex::new(r"([^\d]+)\d*").unwrap();
            let a = re.captures(my_atom.id.0).unwrap().get(1).unwrap().as_str();
            let b = re.captures(compare_atom.id.0).unwrap().get(1).unwrap().as_str();

            if a != b {
                continue;
            }

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
                (Left, Right) => my_atom.clone().invert_spin(),
                (Right, Left) => my_atom.clone().invert_spin(),
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
