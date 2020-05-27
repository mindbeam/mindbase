use super::simpleid::*;
use crate::fuzzyset::{
    FuzzySet,
    FuzzySetMember,
};

use std::cmp::Ordering;

pub struct SymbolMember {
    pub id:     SimpleId,
    pub degree: f32,
}
pub struct Symbol {
    pub set: FuzzySet<SymbolMember>,
}

impl FuzzySetMember for SymbolMember {
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}

impl<T> From<T> for SymbolMember where T: Into<SimpleId>
{
    fn from(item: T) -> Self {
        SymbolMember { id:     item.into(),
                       degree: 1.0, }
    }
}

impl Symbol {
    pub fn null() -> Self {
        Symbol { set: FuzzySet::new() }
    }

    pub fn new<L, T>(list: L) -> Self
        where L: IntoIterator<Item = T>,
              T: Into<SymbolMember>
    {
        let mut set = FuzzySet::new_from_array(list);

        Symbol { set }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, SymbolMember> {
        self.set.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<SymbolMember> {
        self.set.into_iter()
    }

    pub fn drain<'a, T>(&'a mut self, range: T) -> std::vec::Drain<'a, SymbolMember>
        where T: std::ops::RangeBounds<usize>
    {
        self.set.drain(range)
    }
}

#[macro_export]
#[warn(unused_macros)]
macro_rules! sym {
    ($($x:expr),+ $(,)?) => (
        Symbol::new(Box::new([$($x),+]))
    );
}
