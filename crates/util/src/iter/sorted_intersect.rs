use super::SortedIdentifiable;
use std::iter::Peekable;

pub struct SortedIntersect<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable
{
    left:  Peekable<L>,
    right: Peekable<R>,
}

impl<L, R> SortedIntersect<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable
{
    // TODO 2 - Consider creating a marker trait for attestation that the iterator is pre-sorted (Ascending)
    pub fn new(left: L, right: R) -> Self {
        SortedIntersect { left:  left.peekable(),
                          right: right.peekable(), }
    }
}

impl<L, R> Iterator for SortedIntersect<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          L::Item: Ord,
          R::Item: SortedIdentifiable
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut left = match self.left.next() {
            None => return None,
            Some(i) => i,
        };

        let mut right = match self.right.next() {
            None => return None,
            Some(i) => i,
        };

        use std::cmp::Ordering::*;
        loop {
            match left.sort_ident().cmp(right.sort_ident()) {
                Less => {
                    left = match self.left.next() {
                        Some(x) => x,
                        None => return None,
                    };
                },
                Greater => {
                    right = match self.right.next() {
                        Some(x) => x,
                        None => return None,
                    };
                },
                Equal => return Some(left),
            }
        }
    }
}
