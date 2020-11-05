use itertools::{EitherOrBoth, Itertools};

use colorful::{Color, Colorful};

#[derive(Debug, Clone)]
pub struct Item<M>
where
    M: Member,
{
    /// The degree to which this member is applicable to the Fuzzy set
    pub degree: f32,
    /// The Member in question
    pub member: M,
}

pub trait Member: Sized + Clone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    fn invert(&mut self) -> bool {
        // Member did not handle the inversion
        false
    }
    fn display_fmt(&self, item: &Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Member+{:0.1}", item.degree)
    }
    fn display_fmt_set(set: &FuzzySet<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut seen = false;
        for item in set.iter() {
            if seen {
                write!(f, ", ")?;
                item.member.display_fmt(&item, f)?;
            } else {
                seen = true;
                item.member.display_fmt(&item, f)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<M> Item<M>
where
    M: Member,
{
    pub fn invert(&mut self) {
        //did the member handle the inversion?
        if !self.member.invert() {
            // No, therefore we will invert its degree
            self.degree *= -1.0;
        }
    }
}

// Fuzzy set where membership may be negative or positive
#[derive(Clone)]
pub struct FuzzySet<M>(Vec<Item<M>>)
where
    M: Member + Clone;

impl<M> FuzzySet<M>
where
    M: Member + Clone,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_list<A, I>(list: A) -> Self
    where
        A: IntoIterator<Item = I>,
        I: Into<Item<M>>,
    {
        let mut me = Self::new();
        for item in list {
            me.insert(item.into())
        }
        me
    }

    pub fn extend<'a, T>(&'a mut self, iter: T)
    where
        T: IntoIterator<Item = &'a Item<M>>,
    {
        for item in iter {
            self.insert_borrowed(item)
        }
    }

    pub fn insert(&mut self, item: Item<M>) {
        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let existing = &mut self.0.get_mut(i).unwrap();
                existing.degree = existing.degree.max(item.degree);
            }
            Err(i) => self.0.insert(i, item),
        }
    }

    pub fn insert_borrowed(&mut self, item: &Item<M>) {
        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let existing = &mut self.0.get_mut(i).unwrap();
                existing.degree = existing.degree.max(item.degree);
            }
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
    where
        T: std::ops::RangeBounds<usize>,
    {
        self.0.drain(range)
    }

    pub fn union<'a, T>(&'a mut self, other: T)
    where
        T: IntoIterator<Item = Item<M>>,
    {
        for item in other {
            self.insert(item)
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
            item.invert()
        }
    }
}

impl<M> IntoIterator for FuzzySet<M>
where
    M: Member + Clone,
{
    type IntoIter = std::vec::IntoIter<Item<M>>;
    type Item = Item<M>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<M> std::fmt::Display for FuzzySet<M>
where
    M: Member + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Member::display_fmt_set(self, f)
    }
}

impl<M> std::fmt::Debug for FuzzySet<M>
where
    M: Member + std::fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut seen = false;
        for item in self.iter() {
            if seen {
                write!(f, ", ({:?},{:0.1})", item.member, item.degree)?;
            } else {
                seen = true;
                write!(f, "({:?},{:0.1})", item.member, item.degree)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{FuzzySet, Item};

    #[derive(Clone)]
    struct TestMember(usize);
    impl super::Member for TestMember {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl std::fmt::Debug for TestMember {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<usize> for Item<TestMember> {
        fn from(member: usize) -> Self {
            Item {
                member: TestMember(member),
                degree: 1.0,
            }
        }
    }

    #[test]
    fn basic() {
        // All members in this set are fully positive
        let mut fs1 = FuzzySet::from_list(vec![1, 2, 3]);
        assert_eq!(format!("{:?}", fs1), "{1+1.0-0.0, 2+1.0-0.0, 3+1.0-0.0}");

        // Fully negative degree set
        let mut fs2 = FuzzySet::from_list(vec![1, 2, 3]);
        fs2.invert();
        assert_eq!(format!("{:?}", fs2), "{1+0.0-1.0, 2+0.0-1.0, 3+0.0-1.0}");

        // Yes, it's strange, but when we take the union of the two sets, every member should be fully positive and fully negative
        // In practice, this essentially means that the set is null, but the behaviors may be different under subsequent
        // operations versus a null set. Either way we have to differentiate between non-membership and membership which is made
        // irrelevant through contradiction
        fs1.union(fs2);
        assert_eq!(format!("{:?}", fs1), "{1+1.0-1.0, 2+1.0-1.0, 3+1.0-1.0}");
    }
}
