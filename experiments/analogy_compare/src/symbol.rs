use super::atom::*;

pub struct Symbol {
    pub atoms: AtomVec,
}

impl Symbol {
    pub fn simple(id: &'static str) -> Self {
        let mut atoms = AtomVec::new();

        atoms.insert(Atom { id:     AtomId(id),
                            charge: Charge::Left,
                            spin:   Spin::Up, });

        Symbol { atoms }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Atom> {
        self.atoms.iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, Atom>
        where T: std::ops::RangeBounds<usize>
    {
        self.atoms.drain(range)
    }
}

pub fn sym(id: &'static str) -> Symbol {
    Symbol::simple(id)
}
