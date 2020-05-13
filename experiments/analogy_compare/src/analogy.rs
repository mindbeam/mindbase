use super::{
    atom::*,
    symbol::*,
};

pub struct Analogy(pub AtomVec);

impl Analogy {
    pub fn new(mut left: Symbol, mut right: Symbol) -> Self {
        let mut vec = AtomVec::new();

        for atom in left.drain(..) {
            vec.insert(atom.left())
        }
        for atom in right.drain(..) {
            vec.insert(atom.right())
        }

        Analogy(vec)
    }

    pub fn intersect(&self, other: AtomVec) -> Option<AtomVec> {
        let mut a = self.0.iter();
        let mut b = other.iter();

        let mut got_l = false;
        let mut got_r = false;

        let mut out = AtomVec::new();

        // for i in 0..2 {
        //     let my_atom = a.next().unwrap();
        //     let compare_atom = b.next().unwrap();
        //     println!("Compare {:?} vs {:?}", my_atom, compare_atom);

        //     if my_atom.id != compare_atom.id {
        //         continue;
        //     }

        //     match my_atom.charge {
        //         Left => got_l = true,
        //         Right => got_r = true,
        //     };

        //     use Charge::*;
        //     use Spin::*;
        //     let spin = match (&my_atom.charge, &compare_atom.charge) {
        //         (Left, Left) => Up,
        //         (Left, Right) => Down,
        //         (Right, Left) => Down,
        //         (Right, Right) => Up,
        //     };

        //     out.insert();
        // }

        // if got_l && got_r {
        //     Some(out)
        // } else {
        //     None
        // }
        unimplemented!()
    }
}
