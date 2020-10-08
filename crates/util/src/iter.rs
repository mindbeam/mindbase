use std::fmt::Debug;

// pub mod sided_merge;
// pub mod sorted_intersect;
pub mod pairwise_nonrepeating;

pub trait SortedIdentifiable {
    type Ident: Ord + Debug;
    fn sort_ident<'a>(&'a self) -> &'a Self::Ident;
}
