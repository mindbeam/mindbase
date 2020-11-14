use std::cmp::Ordering;

use crate::{
    fuzzyset::{self as fs, FuzzySet},
    symbol::SymbolMember,
};

pub struct AssociativeAnalogy<E = crate::claim::ClaimId>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub(crate) set: FuzzySet<AssociativeAnalogyMember<E>>,
}

#[derive(Debug, Clone)]
pub struct AssociativeAnalogyMember<E> {
    pub entity: E,
    pub side: Side,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl<E> AssociativeAnalogy<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub fn new<List, IntoItem>(left: List, right: List) -> Self
    where
        List: IntoIterator<Item = IntoItem>,
        IntoItem: Into<fs::Item<AssociativeAnalogyMember<E>>>,
    {
        let mut set = FuzzySet::new();

        for item in left.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: AssociativeAnalogyMember {
                    entity: item.member.entity,
                    side: Side::Left,
                },
                degree: item.degree,
            });
        }
        for item in right.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: AssociativeAnalogyMember {
                    entity: item.member.entity,
                    side: Side::Right,
                },
                degree: item.degree,
            });
        }
        AssociativeAnalogy { set }
    }
}

impl<E> AssociativeAnalogyMember<E> {
    pub fn transmute_left(mut self) -> Self {
        self.side = Side::Left;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.side = Side::Right;
        self
    }
}

impl<E> fs::Member for AssociativeAnalogyMember<E>
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

    fn invert(&mut self) -> bool {
        match self.side {
            Side::Left => {
                self.side = Side::Right;
            }
            Side::Right => {
                self.side = Side::Left;
            }
        }
        true
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.side {
            Side::Left => "˱",
            Side::Right => "˲",
        };
        write!(f, "({}{},{:0.2})", self.entity, side, item.degree)
    }
}

impl std::fmt::Display for AssociativeAnalogy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.set.iter().filter(|a| a.member.side == Side::Left) {
            if !first {
                write!(f, " {}", item.member.entity)?;
            } else {
                first = false;
                write!(f, "{}", item.member.entity)?;
            }
        }

        write!(f, " : ")?;

        let mut seen = false;
        for item in self.set.iter().filter(|a| a.member.side == Side::Right) {
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

impl<E> FuzzySet<AssociativeAnalogyMember<E>>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub fn scale_lr(&mut self, left_scale_factor: f32, right_scale_factor: f32) {
        for item in self.iter_mut() {
            match item.member.side {
                Side::Left => item.degree *= left_scale_factor,
                Side::Right => item.degree *= right_scale_factor,
            }
        }
    }
    pub fn left<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember<E>>> + 'a {
        self.iter().filter(|a| a.member.side == Side::Left).map(|a| fs::Item {
            member: SymbolMember {
                entity: a.member.entity.clone(),
            },
            degree: a.degree,
        })
    }

    pub fn right<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember<E>>> + 'a {
        self.iter().filter(|a| a.member.side == Side::Right).map(|a| fs::Item {
            member: SymbolMember {
                entity: a.member.entity.clone(),
            },
            degree: a.degree,
        })
    }
}
