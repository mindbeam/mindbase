use super::atom::*;

pub struct Symbol {
    pub atoms: Vec<Atom>,
}

impl Symbol {
    pub fn simple(id: &'static str) -> Self {
        Symbol { atoms: vec![Atom { id:     Particle(id.to_string()),
                                    charge: Charge::Left,
                                    spin:   Spin::Up, }], }
    }
}

pub fn sym(id: &'static str) -> Symbol {
    Symbol::simple(id)
}
