use std::fmt::Debug;

// pub mod sided_merge;
// pub mod sorted_intersect;
pub mod match_pairwise;

pub trait SortedIdentifiable {
    type Ident: Ord + Debug;
    fn sort_ident(&self) -> Self::Ident;
}
