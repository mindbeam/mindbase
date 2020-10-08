use super::SortedIdentifiable;
use std::{borrow::Borrow, cmp::Ordering, iter::Peekable};

pub struct PairwiseNonrepeating<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    R::Item: SortedIdentifiable,
{
    left: Peekable<L>,
    right: Peekable<R>,
}

impl<L, R> PairwiseNonrepeating<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    R::Item: SortedIdentifiable,
{
    pub fn new(left: L, right: R) -> Self {
        PairwiseNonrepeating {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<L, R> Iterator for PairwiseNonrepeating<L, R>
where
    L: Iterator<Item = R::Item>,
    R: Iterator,
    R::Item: SortedIdentifiable,
    R::Item: Clone,
{
    type Item = (L::Item, R::Item);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (l, r) = match (self.left.peek(), self.right.peek()) {
                (Some(l), Some(r)) => (l, r),
                _ => return None,
            };

            match l.sort_ident().cmp(&r.sort_ident()) {
                Ordering::Less => {
                    println!("Less");
                    // Skip left
                    self.left.next().unwrap();
                }
                Ordering::Greater => {
                    // Skip right
                    self.right.next().unwrap();
                }
                Ordering::Equal => {
                    return Some((self.left.next().unwrap(), self.right.next().unwrap()));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::PairwiseNonrepeating;
    use crate::iter::SortedIdentifiable;

    impl SortedIdentifiable for &i32 {
        type Ident = i32;

        fn sort_ident<'a>(&'a self) -> &'a Self::Ident {
            &self
        }
    }
    #[test]
    fn test1() {
        // lock step
        let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let b: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let iter = PairwiseNonrepeating::new(a.iter(), b.iter());

        assert_eq!(
            vec![(1i32, 1i32), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
            iter.map(|(q, r)| (q.to_owned(), r.to_owned())).collect::<Vec<_>>()
        );

        // Ok lets skip a few
        let a: Vec<i32> = vec![1, 2, 3, 5, 6, 7];
        let b: Vec<i32> = vec![1, 3, 4, 5, 6, 7];
        let iter = PairwiseNonrepeating::new(a.iter(), b.iter());

        assert_eq!(
            vec![(1i32, 1i32), (3, 3), (5, 5), (6, 6), (7, 7)],
            iter.map(|(q, r)| (q.to_owned(), r.to_owned())).collect::<Vec<_>>()
        );
    }
}
