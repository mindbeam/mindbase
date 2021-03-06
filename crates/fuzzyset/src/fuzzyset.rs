use serde::{Deserialize, Serialize};

use crate::traits::Member;

// use itertools::{EitherOrBoth, Itertools};
// use colorful::{Color, Colorful};

const MEMBER_CULL_DEGREE: f64 = 0.001;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item<M>
where
    M: Member,
{
    /// The degree to which this member is applicable to the Fuzzy set
    pub degree: f64,
    /// The Member in question
    pub member: M,
}

impl<M> Item<M>
where
    M: Member,
{
    pub fn invert_degree(&mut self) {
        self.degree *= -1.0;
    }
}

impl<M> std::fmt::Display for Item<M>
where
    M: Member,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}^{:0.2})", self.member, self.degree)
    }
}

// Fuzzy set where membership may be negative or positive
#[derive(Clone, PartialEq)]
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

    /// Insert this item into the set
    /// Note that under certain circumstances this may actually result in the member being _removed_ from
    /// the set if it is negated by this item
    pub fn insert(&mut self, item: Item<M>) {
        if item.degree.abs() < MEMBER_CULL_DEGREE {
            return;
        }

        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let degree;
                {
                    let existing = &mut self.0.get_mut(i).unwrap();
                    // TODO 2 - how do we properly handle union? (Insufficient motivating examples to arrive at clarity)

                    // Lets just average them for now. This gets us:
                    // * idempotence (not sure if this is actually necessary)
                    // * inverse union is null
                    degree = (existing.degree + item.degree) / 2.0;
                    existing.degree = degree;
                }
                if degree < MEMBER_CULL_DEGREE {
                    self.0.remove(i);
                }
            }
            Err(i) => self.0.insert(i, item),
        }
    }

    pub fn insert_borrowed(&mut self, item: &Item<M>) {
        match self.0.binary_search_by(|probe| probe.member.cmp(&item.member)) {
            Ok(i) => {
                let degree;
                {
                    let existing = &mut self.0.get_mut(i).unwrap();
                    degree = (existing.degree + item.degree) / 2.0;
                    existing.degree = degree;
                }
                if degree.abs() < MEMBER_CULL_DEGREE {
                    self.0.remove(i);
                }
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
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        use itertools::{EitherOrBoth, Itertools};
        let iter = self.0.iter().merge_join_by(other.0.iter(), |a, b| a.member.cmp(&b.member));

        let mut sum_of_squares: f64 = 0.0;
        for either in iter {
            // QUESTION - should we attempt to recurse?
            // TODO 1 - Need a difinitive answer to the question of whether an Entity defining a set constitutes CONTAINMENT or something else

            // QUESTION - is euclidean distance even valid if we're missing a data point?
            match either {
                EitherOrBoth::Both(l, r) => {
                    let diff: f64 = l.degree - r.degree;
                    sum_of_squares += diff.powi(2);
                }
                _ =>{}
                // EitherOrBoth::Left(_l) => return None,
                // EitherOrBoth::Right(_r) => return None,
            };
        }

        sum_of_squares.sqrt()
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
        unimplemented!()
    }

    pub fn invert_degree(&mut self) {
        let new = Self::new();
        for item in self.0.iter_mut() {
            item.invert_degree()
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
        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, " ({:?},{:0.2})", item.member, item.degree)?;
            } else {
                first = false;
                write!(f, "({:?},{:0.2})", item.member, item.degree)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::{FuzzySet, Item};

    #[derive(Clone)]
    struct TestMember(usize);
    impl Display for TestMember {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
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
    fn identity() {
        // All members in this set are fully positive
        let mut fs1 = FuzzySet::from_list(vec![1, 2, 3]);
        assert_eq!(format!("{:?}", fs1), "{(1,1.00) (2,1.00) (3,1.00)}");

        // union with itself
        fs1.union(fs1.clone());

        // should be unchanged (??)
        assert_eq!(format!("{:?}", fs1), "{(1,1.00) (2,1.00) (3,1.00)}");
    }
    #[test]
    fn inverse() {
        // All members in this set are fully positive
        let mut fs1 = FuzzySet::from_list(vec![1, 2, 3]);
        assert_eq!(format!("{:?}", fs1), "{(1,1.00) (2,1.00) (3,1.00)}");

        // Fully negative degree set
        let mut fs2 = fs1.clone();
        fs2.invert_degree();
        assert_eq!(format!("{:?}", fs2), "{(1,-1.00) (2,-1.00) (3,-1.00)}");

        // Yes, it's strange, but when we take the union of the two sets, every member should be fully positive and fully negative
        // In practice, this essentially means that the set is null, but the behaviors may be different under subsequent
        // operations versus a null set. Either way we have to differentiate between non-membership and membership which is made
        // irrelevant through contradiction
        // TODO 2 - is the above for real? Don't know enough right now to determine if the set should contain a lasting image of
        // the contradiction, or if it should be expunged

        // TODO 2 - resume support for unions AFTER we have some better use cases to determine ideal behavior
        // fs1.union(fs2);
        // assert_eq!(format!("{:?}", fs1), "{1+1.0-1.0, 2+1.0-1.0, 3+1.0-1.0}");
        // assert_eq!(format!("{:?}", fs1), "{(1,0.0), (2,0.0), (3,0.0)}");
    }
}

impl<M> std::ops::Sub for &FuzzySet<M>
where
    M: Member + std::fmt::Debug + Clone,
{
    type Output = FuzzySet<M>;

    fn sub(self, rhs: &FuzzySet<M>) -> Self::Output {
        use itertools::{EitherOrBoth, Itertools};

        let iter = self.0.iter().merge_join_by(rhs.0.iter(), |a, b| a.member.cmp(&b.member));
        let mut out = FuzzySet::new();
        for either in iter {
            out.insert(match either {
                EitherOrBoth::Both(l, r) => Item {
                    degree: l.degree - r.degree,
                    ..l.clone()
                },
                EitherOrBoth::Left(l) => l.clone(),
                EitherOrBoth::Right(r) => r.clone(),
            })
        }
        out
    }
}
impl<M> std::ops::Add for &FuzzySet<M>
where
    M: Member + std::fmt::Debug + Clone,
{
    type Output = FuzzySet<M>;

    fn add(self, rhs: &FuzzySet<M>) -> Self::Output {
        use itertools::{EitherOrBoth, Itertools};

        let iter = self.0.iter().merge_join_by(rhs.0.iter(), |a, b| a.member.cmp(&b.member));
        let mut out = FuzzySet::new();
        for either in iter {
            out.insert(match either {
                EitherOrBoth::Both(l, r) => Item {
                    degree: l.degree + r.degree,
                    ..l.clone()
                },
                EitherOrBoth::Left(l) => l.clone(),
                EitherOrBoth::Right(r) => r.clone(),
            })
        }
        out
    }
}
