use super::atom::*;
use crate::analogy::Analogy;

pub struct Symbol {
    pub atoms: AtomVec,
}

impl Symbol {
    pub fn null() -> Self {
        Symbol { atoms: AtomVec::new() }
    }

    pub fn new_from_list(list: &[Analogy]) -> Self {
        let mut atoms = AtomVec::new();
        for item in list.iter() {
            atoms.insert(Atom { id:     item.id.clone(),
                                spin:   Spin::Up,
                                side:   Side::Middle,
                                weight: 1.0, })
        }

        Symbol { atoms }
    }

    pub fn new(ids: Box<[&'static str]>) -> Self {
        let mut atoms = AtomVec::new();

        for id in ids.into_iter() {
            atoms.insert(Atom { id:     atomid(id),
                                side:   Side::Left,
                                spin:   Spin::Up,
                                weight: 1.0, });
        }

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

#[macro_export]
#[warn(unused_macros)]
macro_rules! sym {
    ($($x:expr),+ $(,)?) => (
        Symbol::new(Box::new([$($x),+]))
    );
}
