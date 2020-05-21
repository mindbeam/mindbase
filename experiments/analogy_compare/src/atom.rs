use crate::Symbol;
use colorful::{
    Color,
    Colorful,
};
use mindbase::util::iter::SortedIdentifiable;
use std::cmp::Ordering;

// IMPORTANT NOTE: In this experiment, we are using string in lieu of unique identifier.
// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AtomId {
    pub id:   &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Spin {
    Up,
    Down,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Side {
    Middle,
    Left,
    Right,
}

pub fn atom(text: &'static str) -> Atom {
    Atom { id:     atomid(text),
           side:   Side::Left,
           spin:   Spin::Up,
           weight: 1.0, }
}

pub fn atomid(id: &'static str) -> AtomId {
    use regex::Regex;
    let re = Regex::new(r"([^\d]+)\d*").unwrap();
    let text = re.captures(&id).unwrap().get(1).unwrap().as_str();

    AtomId { id, text }
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub id:     AtomId,
    pub spin:   Spin,
    pub side:   Side,
    pub weight: f32,
}

impl Atom {
    pub fn new(id: AtomId) -> Self {
        Atom { id,
               side: Side::Middle,
               spin: Spin::Up,
               weight: 0.05 }
    }

    pub fn transmute_left(mut self) -> Self {
        self.side = Side::Left;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.side = Side::Right;
        self
    }

    pub fn mutate_weight(&mut self, weight_factor: f32) {
        println!("mutate_weight {} * {} = {}",
                 self.weight,
                 weight_factor,
                 self.weight * weight_factor);
        self.weight *= weight_factor;
    }

    pub fn invert_side(mut self) -> Self {
        self.side = match self.side {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Middle => Side::Middle,
        };
        self
    }

    pub fn invert_spin(mut self) -> Self {
        self.spin = match self.spin {
            Spin::Up => Spin::Down,
            Spin::Down => Spin::Up,
        };
        self
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.id.cmp(&other.id) {
            Ordering::Equal => {},
            o @ _ => return o,
        }
        match self.side.cmp(&other.side) {
            Ordering::Equal => {},
            o @ _ => return o,
        }
        self.spin.cmp(&other.spin)
    }
}
impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.side == self.side && self.spin == other.spin
    }
}
impl Eq for Atom {}

#[derive(Debug, Clone)]
pub struct AtomVec(Vec<Atom>);

impl AtomVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_from_array(array: Box<[&'static str]>) -> Self {
        let mut me = Self::new();
        for id in array.iter() {
            me.insert(Atom::new(atomid(id)))
        }
        me
    }

    pub fn from_left_right(left: AtomVec, right: AtomVec) -> Self {
        let mut vec = Self::new();
        for atom in left.into_iter() {
            vec.insert(atom.transmute_left());
        }
        for atom in right.into_iter() {
            vec.insert(atom.transmute_right());
        }
        vec
    }

    pub fn extend<'a, T>(&'a mut self, iter: T)
        where T: IntoIterator<Item = &'a Atom>
    {
        for atom in iter {
            self.insert_borrowed(atom)
        }
    }

    pub fn insert(&mut self, atom: Atom) {
        match self.0.binary_search(&atom) {
            Ok(_) => {}, // duplicate
            Err(i) => self.0.insert(i, atom),
        }
    }

    pub fn insert_borrowed(&mut self, atom: &Atom) {
        match self.0.binary_search(atom) {
            Ok(_) => {}, // duplicate
            Err(i) => self.0.insert(i, atom.clone()),
        }
    }

    pub fn left<'a>(&'a self) -> impl Iterator<Item = &Atom> + 'a {
        self.0.iter().filter(|a| a.side == Side::Left)
    }

    pub fn right<'a>(&'a self) -> impl Iterator<Item = &Atom> + 'a {
        self.0.iter().filter(|a| a.side == Side::Right)
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Atom> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Atom> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Atom> {
        self.0.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, Atom>
        where T: std::ops::RangeBounds<usize>
    {
        self.0.drain(range)
    }

    pub fn diag(&self) -> String {
        let mut out: Vec<String> = Vec::new();
        for atom in self.iter() {
            let spin = match atom.spin {
                Spin::Up => "↑",
                Spin::Down => "↓",
            };
            //˰˯

            let side = match atom.side {
                Side::Middle => "ᐧ",
                Side::Left => "˱",
                Side::Right => "˲",
            };

            assert!(atom.weight <= 1.0, "Invalid atom weight");

            let mut weight = format!("{:.2}", atom.weight);
            if atom.weight < 1.0 {
                weight.remove(0);
            } else {
                weight.truncate(0);
            }

            out.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
                                                                        .to_string());
        }

        out.join(",")
    }

    pub fn diag_lr(&self) -> String {
        let mut lefts: Vec<String> = Vec::new();
        let mut rights: Vec<String> = Vec::new();

        for atom in self.iter() {
            let spin = match atom.spin {
                Spin::Up => "↑",
                Spin::Down => "↓",
            };
            let side = match atom.side {
                Side::Middle => "ᐧ",
                Side::Left => "˱",
                Side::Right => "˲",
            };

            assert!(atom.weight <= 1.0, "Invalid atom weight");

            let mut weight = format!("{:.2}", atom.weight);
            if atom.weight < 1.0 {
                weight.remove(0);
            } else {
                weight.truncate(0);
            }

            match atom.side {
                Side::Middle => unimplemented!(),
                Side::Left => {
                    lefts.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
                                                                                  .to_string())
                },
                Side::Right => {
                    rights.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
                                                                                   .to_string())
                },
            }
        }

        format!("{} <-> {}", lefts.join(","), rights.join(",")).to_string()
    }
}

impl SortedIdentifiable for &Atom {
    type Ident = &'static str;

    fn sort_ident<'a>(&'a self) -> &'a Self::Ident {
        &self.id.id
    }
}

#[macro_export]
#[warn(unused_macros)]
macro_rules! atomvec {
    ($($x:expr),+ $(,)?) => (
        AtomVec::new_from_array(Box::new([$($x),+]))
    );
}
