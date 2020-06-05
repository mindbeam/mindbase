use super::simpleid::*;
use crate::{
    analogy::AnalogyMember,
    fuzzyset as fs,
    fuzzyset::FuzzySet,
};

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct SymbolMember {
    pub id: SimpleId,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub set: FuzzySet<SymbolMember>,
}

impl fs::Member for SymbolMember {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl From<fs::Item<AnalogyMember>> for fs::Item<SymbolMember> {
    fn from(analogy_member: fs::Item<AnalogyMember>) -> Self {
        fs::Item { member:  SymbolMember { id: analogy_member.member.id, },
                   pdegree: analogy_member.pdegree,
                   ndegree: analogy_member.ndegree, }
    }
}
impl<T> From<T> for fs::Item<SymbolMember> where T: Into<SimpleId>
{
    fn from(item: T) -> Self {
        fs::Item { member:  SymbolMember { id: item.into() },
                   pdegree: 1.0,
                   ndegree: 0.0, }
    }
}

impl Symbol {
    pub fn null() -> Self {
        Symbol { set: FuzzySet::new() }
    }

    pub fn new<L, T>(list: L) -> Self
        where L: IntoIterator<Item = T>,
              T: Into<fs::Item<SymbolMember>>
    {
        let mut set = FuzzySet::from_list(list);

        Symbol { set }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, fs::Item<SymbolMember>> {
        self.set.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<fs::Item<SymbolMember>> {
        self.set.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, fs::Item<SymbolMember>>
        where T: std::ops::RangeBounds<usize>
    {
        self.set.drain(range)
    }
}

#[macro_export]
#[warn(unused_macros)]
macro_rules! sym {
    ($($x:expr),+ $(,)?) => (
        Symbol::new(&[$($x),+])
    );
}
