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
            vec.insert(atom.left())
        }
        for atom in right.drain(..) {
            vec.insert(atom.right())
        }

        Analogy { id, vec }
    }

    pub fn intersect(&self, mut other: AtomVec) -> Option<AtomVec> {
        let mut a = self.vec.iter();
        let mut b = other.drain(..);

        let mut got_l = false;
        let mut got_r = false;

        let mut out = AtomVec::new();

        for i in 0..2 {
            let my_atom = a.next().unwrap();
            let compare_atom = b.next().unwrap();
            // println!("Compare {:?} vs {:?}", my_atom, compare_atom);

            if my_atom.id != compare_atom.id {
                continue;
            }

            match my_atom.charge {
                Left => got_l = true,
                Right => got_r = true,
                Middle => unimplemented!(),
            };

            use Charge::*;

            // TODO use spin to invert this logic
            let updated_compare_atom = match (&my_atom.charge, &compare_atom.charge) {
                (Left, Left) => compare_atom,
                (Left, Right) => compare_atom.invert_spin(),
                (Right, Left) => compare_atom.invert_spin(),
                (Right, Right) => compare_atom,
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
