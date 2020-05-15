use super::SortedIdentifiable;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    iter::Peekable,
};

pub struct MatchPairwise<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable
{
    left:  Peekable<L>,
    right: Peekable<R>,
}

impl<'a, L, R> MatchPairwise<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable
{
    // TODO 2 - Consider creating a marker trait for attestation that the iterator is pre-sorted? (Ascending)

    pub fn new(left: L, right: R) -> Self {
        MatchPairwise { left:  left.peekable(),
                        right: right.peekable(), }
    }
}

impl<L, R> Iterator for MatchPairwise<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable,
          R::Item: Clone
{
    type Item = (L::Item, R::Item);

    fn next(&mut self) -> Option<Self::Item> {
        println!("Next");
        loop {
            println!("Loop");
            let (l, r) = match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => (l, r),
                _ => return None,
            };

            println!("Something {:?} <> {:?}", l.sort_ident(), r.sort_ident());
            match l.sort_ident().cmp(&r.sort_ident()) {
                Ordering::Less => {
                    println!("Less");
                    // Skip left
                    self.left.next().unwrap();
                },
                Ordering::Greater => {
                    println!("Greater");
                    // Skip right
                    self.right.next().unwrap();
                },
                Ordering::Equal => {
                    println!("Equal");
                    return Some((l.clone(), r.clone()));
                },
            }
        }
    }
}
