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
    left:  L,
    right: R,
    buff:  Vec<MatchPairwiseBuffItem<R::Item>>,
}
enum MatchPairwiseBuffItem<T> {
    Left(T),
    Right(T),
}

impl<'a, L, R> MatchPairwise<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          R::Item: SortedIdentifiable
{
    pub fn new(left: L, right: R) -> Self {
        MatchPairwise { left,
                        right,
                        buff: Vec::new() }
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
            if self.buff.len() == 0 {
                match self.left.next() {
                    None => return None,
                    Some(l) => self.buff.push(MatchPairwiseBuffItem::Left(l.clone())),
                }
            } else {
                match self.right.next() {
                    None => {},
                    Some(_) => {},
                }
            }

            // println!("Loop");
            // let (l, r) = match (self.left.peek(), self.right.peek()) {
            //     (Some(l), Some(r)) => (l, r),
            //     _ => return None,
            // };

            // println!("Something {:?} <> {:?}", l.sort_ident(), r.sort_ident());
            // match l.sort_ident().cmp(&r.sort_ident()) {
            //     Ordering::Less => {
            //         println!("Less");
            //         // Skip left
            //         self.left.next().unwrap();
            //     },
            //     Ordering::Greater => {
            //         println!("Greater");
            //         // Skip right
            //         self.right.next().unwrap();
            //     },
            //     Ordering::Equal => {
            //         println!("Equal");
            //         return Some((self.left.next().unwrap(), self.right.next().unwrap()));
            //     },
            // }
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::MatchPairwise;
    use crate::util::iter::SortedIdentifiable;

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
        let iter = MatchPairwise::new(a.iter(), b.iter());

        assert_eq!(vec![(1i32, 1i32), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
                   iter.map(|(q, r)| (q.to_owned(), r.to_owned())).collect::<Vec<_>>());

        // one side has an extra "2"
        let a: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 6, 7];
        let b: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 6, 7];
        let iter = MatchPairwise::new(a.iter(), b.iter());

        assert_eq!(vec![(1i32, 1i32), (2, 2), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
                   iter.map(|(q, r)| (q.to_owned(), r.to_owned())).collect::<Vec<_>>());
    }
}
