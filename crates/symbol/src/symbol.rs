use crate::{
    fuzzyset::{self as fs, FuzzySet},
    traits::Entity,
};

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Symbol<E>
where
    E: Entity,
{
    // A symbol is essentially just a Non-polar fuzzyset
    pub set: FuzzySet<SymbolMember<E>>,
}

#[derive(Clone, Debug)]
pub struct SymbolMember<E> {
    pub entity: E,
}

impl<E> Symbol<E>
where
    E: Entity,
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
    E: Entity,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity.cmp(&other.entity)
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{:0.2})", self.entity, item.degree)
    }
}

impl<E> std::fmt::Display for Symbol<E>
where
    E: Entity,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.set)
    }
}

impl<E> IntoIterator for Symbol<E>
where
    E: Entity, // IntoItem: Into<fs::Item<AssociativeAnalogyMember<E>>>,
{
    type Item = fs::Item<SymbolMember<E>>;

    type IntoIter = std::vec::IntoIter<fs::Item<SymbolMember<E>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
