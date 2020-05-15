use std::{
    cmp::Ordering,
    iter::Peekable,
};

pub struct SidedMerge<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator
{
    left:  Peekable<L>,
    right: Peekable<R>,
}

impl<L, R> SidedMerge<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator
{
    // TODO 2 - Consider creating a marker trait for attestation that the iterator is pre-sorted? (Ascending)

    pub fn new(left: L, right: R) -> Self {
        SidedMerge { left:  left.peekable(),
                     right: right.peekable(), }
    }
}

pub struct SidedMergeItem<T> {
    pub item: T,
    side:     ItemSide,
}
enum ItemSide {
    Left,
    Right,
}

impl<T: Clone> SidedMergeItem<&T> {
    pub fn to_owned(self) -> SidedMergeItem<T> {
        SidedMergeItem { item: self.item.clone(),
                         side: self.side, }
    }
}

impl<L, R> Iterator for SidedMerge<L, R>
    where L: Iterator<Item = R::Item>,
          R: Iterator,
          L::Item: Ord
{
    type Item = SidedMergeItem<L::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let which = match (self.left.peek(), self.right.peek()) {
            (Some(l), Some(r)) => Some(l.cmp(r)),
            (Some(_), None) => Some(Ordering::Less),
            (None, Some(_)) => Some(Ordering::Greater),
            (None, None) => None,
        };

        match which {
            Some(Ordering::Less) => {
                Some(SidedMergeItem { item: self.left.next().unwrap(),
                                      side: ItemSide::Left, })
            },
            Some(Ordering::Equal) => {
                Some(SidedMergeItem { item: self.left.next().unwrap(),
                                      side: ItemSide::Left, })
            },
            Some(Ordering::Greater) => {
                Some(SidedMergeItem { item: self.right.next().unwrap(),
                                      side: ItemSide::Right, })
            },
            None => None,
        }
    }
}
