use std::cmp::Ordering;

use crate::{
    fuzzyset::{self as fs, FuzzySet},
    traits::Entity,
};

// QUESTION:
// Is a categorical analogy merely a shortcut for an associative analogy between the subject Symbol and a Narrow-symbol formed of a novel unit claim?
// The identity of that unit claim may be equivalent to the identity of a categorical analogy, except insofar as the associative analogy would bestow an additional identity
// (Assuming an ISA predicate)
// TODO 2 model this both ways to determine equivalence

pub struct CategoricalAnalogy<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    set: FuzzySet<CategoricalAnalogyMember<E>>,
}

#[derive(Debug, Clone)]
pub struct CategoricalAnalogyMember<E> {
    pub entity: E,
}

impl<E> CategoricalAnalogy<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub fn new<IntoId, List, IntoMember>(list: List) -> Self
    where
        List: IntoIterator<Item = IntoMember>,
        IntoMember: Into<fs::Item<CategoricalAnalogyMember<E>>>,
    {
        Self {
            set: FuzzySet::from_list(list),
        }
    }
}

impl<E> fs::Member for CategoricalAnalogyMember<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.entity.cmp(&other.entity) {
            Ordering::Equal => {}
            o @ _ => return o,
        }
        unimplemented!("TODO 2 - better handle same identity with different sidedness")
        // self.side.cmp(&other.side)
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{:0.2})", self.entity, item.degree)
    }

    fn display_fmt_set(set: &FuzzySet<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut first = true;
        for item in set.iter() {
            if !first {
                write!(f, " ")?;
                item.member.display_fmt(&item, f)?;
            } else {
                first = false;
                item.member.display_fmt(&item, f)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl<E> std::fmt::Display for CategoricalAnalogy<E>
where
    E: Entity,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.set.iter() {
            if !first {
                write!(f, " {}", item.member.entity)?;
            } else {
                first = false;
                write!(f, "{}", item.member.entity)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}
