use crate::{
    analogy::associative::{AssociativeAnalogyMember, Side},
    fuzzyset as fs,
    traits::Entity,
};
use crate::{
    fuzzyset::FuzzySet,
    symbol::{Symbol, SymbolMember},
};

#[macro_export]
#[warn(unused_macros)]
macro_rules! sym {
    ($($x:expr),+) => (
        Symbol::new(&[$($x),+])
    );
}

// impl<E> From<fs::Item<AssociativeAnalogyMember<E>>> for fs::Item<SymbolMember<E>>
// where
//     E: Clone,
// {
//     fn from(analogy_member: fs::Item<AssociativeAnalogyMember<E>>) -> Self {
//         fs::Item {
//             member: SymbolMember {
//                 entity: analogy_member.member.entity,
//             },
//             degree: analogy_member.degree,
//         }
//     }
// }

// impl<E, T> Into<fs::Item<SymbolMember<E>>> for T
// where
//     T: Into<E>,
//     E: Entity,
// {
//     fn into(self) -> fs::Item<SymbolMember<E>> {
//         fs::Item {
//             member: SymbolMember { entity: self.into() },
//             degree: 1.0,
//         }
//     }
// }

impl<E> From<(Symbol<E>, Symbol<E>)> for FuzzySet<AssociativeAnalogyMember<E>>
where
    E: Entity,
{
    fn from(tuple: (Symbol<E>, Symbol<E>)) -> Self {
        let mut set = FuzzySet::new();

        for i in tuple.0.into_iter() {
            set.insert(fs::Item {
                degree: i.degree,
                member: AssociativeAnalogyMember {
                    entity: i.member.entity,
                    side: Side::Left,
                },
            });
        }

        for i in tuple.1.into_iter() {
            set.insert(fs::Item {
                degree: i.degree,
                member: AssociativeAnalogyMember {
                    entity: i.member.entity,
                    side: Side::Right,
                },
            });
        }

        set
    }
}

impl<E, T> From<&(T, f32)> for fs::Item<SymbolMember<E>>
where
    T: Into<E>,
    T: Clone,
    E: Entity,
{
    fn from(item: &(T, f32)) -> Self {
        fs::Item {
            member: SymbolMember {
                entity: item.0.clone().into(),
            },
            degree: item.1,
        }
    }
}
