use itertools::{
    EitherOrBoth,
    Itertools,
};

#[derive(Clone)]
pub struct Item<M>
    where M: Member
{
    /// The degree to which this is a member of the Fuzzy set
    pub pdegree: f32,
    /// The degree to which this is an anti-member of the Fuzzy set
    pub ndegree: f32,
    /// The Member in question
    pub member:  M,
}

pub trait Member {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
}

// Fuzzy set where membership may be negative or positive
#[derive(Clone)]
pub struct FuzzySet<M>(Vec<Item<M>>) where M: Member + Clone;

impl<M> FuzzySet<M> where M: Member + Clone
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_from_array<A, I>(array: A) -> Self
        where A: IntoIterator<Item = I>,
              I: Into<Item<M>>
    {
        let mut me = Self::new();
        for item in array {
            me.insert(item.into())
        }
        me
    }

    pub fn extend<'a, T>(&'a mut self, iter: T)
        where T: IntoIterator<Item = &'a Item<M>>
    {
        for item in iter {
            self.insert_borrowed(item)
        }
    }

    pub fn insert(&mut self, item: Item<M>) {
        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let existing = &mut self.0.get_mut(i).unwrap();
                existing.pdegree = existing.pdegree.max(item.pdegree);
                existing.ndegree = existing.ndegree.max(item.ndegree);
            },
            Err(i) => self.0.insert(i, item),
        }
    }

    pub fn insert_borrowed(&mut self, item: &Item<M>) {
        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let existing = &mut self.0.get_mut(i).unwrap();
                existing.pdegree = existing.pdegree.max(item.pdegree);
                existing.ndegree = existing.ndegree.max(item.ndegree);
            },
            Err(i) => self.0.insert(i, item.clone()),
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Item<M>> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Item<M>> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Item<M>> {
        self.0.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, Item<M>>
        where T: std::ops::RangeBounds<usize>
    {
        self.0.drain(range)
    }

    pub fn union(&mut self, other: &Self) {
        for item in other.iter() {
            self.insert_borrowed(item)
        }
    }

    pub fn intersect(&mut self, other: &Self) {
        // Don't keep searching the front of the list over and over
        // TODO
        // let mut cursor = 0;
        // for item in other.iter() {
        //     match self.0[cursor..].binary_search_by(|probe| probe.member.cmp(&item.member)) {
        //         Ok(i) => {
        //             cursor = i;
        //             let d = &mut self.0.get_mut(i).unwrap().degree;
        //             *d = d.max(degree);
        //         },
        //         Err(i) => {
        //             cursor = i;
        //             self.0.insert(i,
        //                           Item { member: (*member).clone(),
        //                                  degree })
        //         },
        //     }
        // }
    }

    pub fn invert(&mut self) {
        let new = Self::new();
        for item in self.0.iter_mut() {
            let pd = item.pdegree;

            // switcheroo
            item.pdegree = item.ndegree;
            item.ndegree = pd;
        }
    }
}

#[cfg(test)]
mod test {
    use super::FuzzySet;

    #[test]
    fn intersect() {
        #[derive(Clone)]
        struct TestMember(usize);

        impl super::Member for TestMember {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        let fs = FuzzySet::new_from_array(&[TestMember(1)]);
        let fs = FuzzySet::new_from_array(&[TestMember(1)]);
    }
}
