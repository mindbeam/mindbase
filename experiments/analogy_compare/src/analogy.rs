use super::{
    atom::*,
    symbol::*,
};

pub struct Analogy(Vec<Atom>);

impl Analogy {
    pub fn new(left: Symbol, right: Symbol) -> Self {
        use Charge::*;
        use Spin::*;

        let left = Atom { id:     left,
                          charge: Left,
                          spin:   Up, };

        let right = Entity { id:   right,
                             side: Right,
                             spin: Up, };

        use std::cmp::Ordering::*;

        let vec = match left.id.cmp(right.id) {
            Less => vec![left, right],
            Greater => vec![right, left],
            Equal => panic!("duplicates not permitted"),
        };

        Analogy(vec)
    }

    pub fn intersect(&self, other: Vec<Entity>) -> Option<Vec<Entity>> {
        let mut a = self.0.iter();
        let mut b = other.iter();

        let mut got_l = false;
        let mut got_r = false;

        let mut out = Vec::new();
        for i in 0..2 {
            let my_atom = a.next().unwrap();
            let compare_atom = b.next().unwrap();
            println!("Compare {:?} vs {:?}", my_atom, compare_atom);

            if my_atom.id != compare_atom.id {
                continue;
            }

            match my_atom.side {
                Left => got_l = true,
                Right => got_r = true,
            };

            use Side::*;
            use Spin::*;
            let spin = match (&my_atom.side, &compare_atom.side) {
                (Left, Left) => Up,
                (Left, Right) => Down,
                (Right, Left) => Down,
                (Right, Right) => Up,
            };

            out.push(Entity { id: my_atom.id,
                              spin,
                              side: my_atom.side.clone() });
        }

        if got_l && got_r {
            Some(out)
        } else {
            None
        }
    }
}
