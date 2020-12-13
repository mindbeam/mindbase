use crate::fuzzyset::{FuzzySet, Item};

pub trait Member: Sized + Clone + std::fmt::Display {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    fn display_fmt(&self, item: &Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Member,{:0.1})", item.degree)
    }
    fn display_fmt_set(set: &FuzzySet<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO - consider removing this. We don't need Fuzzysets to format
        // themselves differently based on their member type

        write!(f, "{{")?;
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
        write!(f, "}}")?;
        Ok(())
    }
}
