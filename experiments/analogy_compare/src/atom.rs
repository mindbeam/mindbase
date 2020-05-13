// IMPORTANT NOTE: In this experiment, we are using string in lieu of unique identifier.
// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AtomId(pub(crate) String);

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Spin {
    Up,
    Down,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Charge {
    Middle,
    Left,
    Right,
}

pub fn atom(id: &'static str) -> Atom {
    Atom { id:     AtomId(id.to_string()),
           charge: Charge::Left,
           spin:   Spin::Up, }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Atom {
    pub id:     AtomId,
    pub spin:   Spin,
    pub charge: Charge,
}

impl Atom {
    pub fn new(id: AtomId) -> Self {
        Atom { id,
               charge: Charge::Middle,
               spin: Spin::Up }
    }

    pub fn left(mut self) -> Self {
        self.charge = Charge::Left;
        self
    }

    pub fn right(mut self) -> Self {
        self.charge = Charge::Right;
        self
    }

    // pub fn id(&self) -> &AtomId {
    //     &self.id
    // }
}

#[derive(Debug)]
pub struct AtomVec(pub Vec<Atom>);

impl AtomVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, atom: Atom) {
        match self.0.binary_search(&atom) {
            Ok(_) => {}, // duplicate
            Err(i) => self.0.insert(i, atom),
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Atom> {
        self.0.iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, Atom>
        where T: std::ops::RangeBounds<usize>
    {
        self.0.drain(range)
    }
}
