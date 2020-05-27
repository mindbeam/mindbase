pub trait FuzzySetMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
}

#[derive(Debug, Clone)]
pub struct FuzzySet<M: FuzzySetMember>(Vec<M>);

impl<M> FuzzySet<M> where M: FuzzySetMember
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_from_array<A, I>(array: A) -> Self
        where A: IntoIterator<Item = I>,
              I: Into<M>
    {
        let mut me = Self::new();
        for item in array {
            me.insert(item.into())
        }
        me
    }

    pub fn extend<'a, T>(&'a mut self, iter: T)
        where T: IntoIterator<Item = &'a M>
    {
        for item in iter {
            self.insert_borrowed(item)
        }
    }

    pub fn insert(&mut self, member: M) {
        match self.0.binary_search_by(|probe| probe.cmp(&member)) {
            Ok(_) => {}, // duplicate
            Err(i) => self.0.insert(i, member),
        }
    }

    pub fn insert_borrowed(&mut self, member: &M) {
        match self.0.binary_search_by(|probe| probe.cmp(member)) {
            Ok(_) => {}, // duplicate
            Err(i) => self.0.insert(i, *member.clone()),
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, M> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, M> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<M> {
        self.0.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, M>
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
                AnalogySide::Middle => "ᐧ",
                AnalogySide::Left => "˱",
                AnalogySide::Right => "˲",
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
                AnalogySide::Middle => "ᐧ",
                AnalogySide::Left => "˱",
                AnalogySide::Right => "˲",
            };

            assert!(atom.weight <= 1.0, "Invalid atom weight");

            let mut weight = format!("{:.2}", atom.weight);
            if atom.weight < 1.0 {
                weight.remove(0);
            } else {
                weight.truncate(0);
            }

            match atom.side {
                AnalogySide::Middle => unimplemented!(),
                AnalogySide::Left => {
                    lefts.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
                                                                                  .to_string())
                },
                AnalogySide::Right => {
                    rights.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
                                                                                   .to_string())
                },
            }
        }

        format!("{} <-> {}", lefts.join(","), rights.join(",")).to_string()
    }
}
