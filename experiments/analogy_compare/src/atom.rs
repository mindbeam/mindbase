use mindbase::util::iter::SortedIdentifiable;

// IMPORTANT NOTE: In this experiment, we are using string in lieu of unique identifier.
// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"
#[derive(Debug, Clone)]
pub struct AtomId {
    pub id:   String,
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
    use regex::Regex;
    let re = Regex::new(r"([^\d]+)\d*").unwrap();
    let id = re.captures(&text).unwrap().get(1).unwrap();

    Atom { id:   AtomId { id, text },
           side: Side::Left,
           spin: Spin::Up, }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Atom {
    pub id:   AtomId,
    pub spin: Spin,
    pub side: Side,
}

impl Atom {
    pub fn new(id: AtomId) -> Self {
        Atom { id,
               side: Side::Middle,
               spin: Spin::Up }
    }

    pub fn transmute_left(mut self) -> Self {
        self.side = Side::Left;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.side = Side::Right;
        self
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

#[derive(Debug, Clone)]
pub struct AtomVec(pub Vec<Atom>);

impl AtomVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_left_right(left: &'static str, right: &'static str) -> Self {
        let mut vec = Self::new();
        vec.insert(atom(left).transmute_left());
        vec.insert(atom(right).transmute_right());
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
            out.push(format!("{}{}{}", atom.id.0, side, spin).to_string());
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

            match atom.side {
                Side::Middle => unimplemented!(),
                Side::Left => lefts.push(format!("{}{}{}", atom.id.0, side, spin).to_string()),
                Side::Right => rights.push(format!("{}{}{}", atom.id.0, side, spin).to_string()),
            }
        }

        format!("{} <-> {}", lefts.join(","), rights.join(",")).to_string()
    }
}

impl SortedIdentifiable for Atom {
    type Ident = String;

    fn sort_ident(&self) -> Self::Ident {
        self.id.id.clone()
    }
}
