use serde::{Deserialize, Serialize};

use crate::fuzzyset::{self as fs, FuzzySet};

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Symbol<E = crate::claim::ClaimId>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub set: FuzzySet<SymbolMember<E>>,
}

#[derive(Clone, Debug)]
pub struct SymbolMember<E> {
    pub entity: E,
}

impl<E> Symbol<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub fn null() -> Self {
        // QUESTION: Should it be possible to represent a null symbol?
        Symbol { set: FuzzySet::new() }
    }

    pub fn new<L, T>(list: L) -> Self
    where
        L: IntoIterator<Item = T>,
        T: Into<fs::Item<SymbolMember<E>>>,
    {
        let mut set = FuzzySet::from_list(list);

        Symbol { set }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, fs::Item<SymbolMember<E>>> {
        self.set.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<fs::Item<SymbolMember<E>>> {
        self.set.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, fs::Item<SymbolMember<E>>>
    where
        T: std::ops::RangeBounds<usize>,
    {
        self.set.drain(range)
    }

    pub fn union(&mut self, other: Self) {
        self.set.union(other.set);
    }
}

impl<E> fs::Member for SymbolMember<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity.cmp(&other.entity)
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{:0.2})", self.entity, item.degree)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.set)
    }
}

pub mod convenience {
    use crate::analogy::associative::AssociativeAnalogyMember;

    use super::*;

    #[macro_export]
    #[warn(unused_macros)]
    macro_rules! sym {
        ($($x:expr),+) => (
            Symbol::new(&[$($x),+])
        );
    }

    // impl<E> From<fs::Item<AssociativeAnalogyMember<E>>> for fs::Item<SymbolMember<E>>
    // where
    //     E: Clone,
    // {
    //     fn from(analogy_member: fs::Item<AssociativeAnalogyMember<E>>) -> Self {
    //         fs::Item {
    //             member: SymbolMember {
    //                 entity: analogy_member.member.entity,
    //             },
    //             degree: analogy_member.degree,
    //         }
    //     }
    // }

    // impl<E, I> From<I> for fs::Item<SymbolMember<E>>
    // where
    //     I: Into<E>,
    // {
    //     fn from(item: I) -> Self {
    //         fs::Item {
    //             member: SymbolMember { entity: item.into() },
    //             degree: 1.0,
    //         }
    //     }
    // }

    impl<E, T> From<&(T, f32)> for fs::Item<SymbolMember<E>>
    where
        T: Into<E>,
        T: Clone,
        E: Clone + std::fmt::Display + std::cmp::Ord,
    {
        fn from(item: &(T, f32)) -> Self {
            fs::Item {
                member: SymbolMember {
                    entity: item.0.clone().into(),
                },
                degree: item.1,
            }
        }
    }
}
