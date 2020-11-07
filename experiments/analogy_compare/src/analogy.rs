use super::{fuzzyset as fs, fuzzyset::FuzzySet, simpleid::*, symbol::*};

use itertools::{EitherOrBoth, Itertools};

use std::cmp::Ordering;

pub struct Analogy {
    pub id: SimpleId,
    pub set: FuzzySet<AnalogyMember>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AnalogySide {
    Categorical,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct CategoricalAnalogyMember {
    pub id: SimpleId,
}

impl Into<AnalogyMember> for CategoricalAnalogyMember {
    fn into(self) -> AnalogyMember {
        AnalogyMember {
            id: self.id,
            side: AnalogySide::Categorical,
        }
    }
}

impl Into<fs::Item<AnalogyMember>> for &'static &'static str {
    fn into(self) -> fs::Item<AnalogyMember> {
        fs::Item {
            degree: 1.0,
            member: AnalogyMember {
                id: self.into(),
                side: AnalogySide::Categorical,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalogyMember {
    pub id: SimpleId,
    pub side: AnalogySide,
}

impl AnalogyMember {
    // pub fn new(id: SimpleId) -> Self {
    //     AnalogyMember { id,
    //                     side: AnalogySide::Categorical }
    // }

    // pub fn match_side(&self, other: &Self) -> Option<AnalogySide> {
    //     if self.spin == other.spin {
    //         match (&self.side, &other.side) {
    //             (AnalogySide::Left, AnalogySide::Left) => Some(AnalogySide::Left),
    //             (AnalogySide::Right, AnalogySide::Right) => Some(AnalogySide::Right),
    //             _ => None,
    //         }
    //     } else {
    //         // inverse
    //         match (&self.side, &other.side) {
    //             (AnalogySide::Left, AnalogySide::Right) => Some(AnalogySide::Left),
    //             (AnalogySide::Right, AnalogySide::Left) => Some(AnalogySide::Right),
    //             _ => None,
    //         }
    //     }
    // }

    pub fn transmute_left(mut self) -> Self {
        self.side = AnalogySide::Left;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.side = AnalogySide::Right;
        self
    }
}

impl<'a> Into<&'a FuzzySet<AnalogyMember>> for &'a Analogy {
    fn into(self) -> &'a FuzzySet<AnalogyMember> {
        &self.set
    }
}
impl<'a> Into<&'a FuzzySet<AnalogyMember>> for &'a AnalogyQuery {
    fn into(self) -> &'a FuzzySet<AnalogyMember> {
        &self.set
    }
}

impl Analogy {
    /// identify the subset of this analogy's fuzzy-set which intersect the comparison set
    /// and conditionally invert the sidedness of the resultant set to match the comparison set
    pub fn interrogate<'a, T>(&'a self, query_set: T) -> Option<FuzzySet<AnalogyMember>>
    where
        T: Into<&'a FuzzySet<AnalogyMember>>,
    {
        // FuzzySets are always sorted by ID (side is disabled for now), so we can outer join
        let mut iter = self
            .set
            .iter()
            .merge_join_by(query_set.into().iter(), |a, b| a.member.id.cmp(&b.member.id));

        // Execution plan:
        // * We're comparing all Members for both symbols within this analogy to a Two-sided FuzzySet containing *Candidate* Members
        // * At least one Left Atom and one Right Atom must match in order for the relationship to have any weight at all
        // * We're expanding the set of those atoms which are inferred (Spin-adjusted opposite-side Atoms) with a score based on
        // weighted sum of the matching spin-adjusted same-side matches

        #[derive(Default)]
        struct Bucket {
            degree: f32,
            count: u32,
        };

        // We need to sum up the degrees, and the count of each matching item
        // The first letter is the analogy item side, and the second is the query item side
        let mut ll_bucket: Bucket = Default::default();
        let mut rr_bucket: Bucket = Default::default();
        let mut lr_bucket: Bucket = Default::default();
        let mut rl_bucket: Bucket = Default::default();

        // We also need to count the non-matching count
        let mut nonmatching_left_count = 0u32;
        let mut nonmatching_right_count = 0u32;

        let mut out = FuzzySet::new();

        for either in iter {
            match either {
                EitherOrBoth::Right(analogy_item) => {
                    // Present in query, but not present in analogy
                    match &analogy_item.member.side {
                        AnalogySide::Left => nonmatching_left_count += 1,
                        AnalogySide::Right => nonmatching_right_count += 1,
                        _ => unimplemented!("Not clear on how/if categorical analogies mix with sided"),
                    }
                }
                EitherOrBoth::Both(analogy_item, query_item) => {
                    // We've got a hit

                    // Scale the degree of the matching item by that of the query
                    let match_degree = analogy_item.degree * query_item.degree;

                    let bucket = match (&analogy_item.member.side, &query_item.member.side) {
                        (AnalogySide::Left, AnalogySide::Left) => &mut ll_bucket,
                        (AnalogySide::Right, AnalogySide::Right) => &mut rr_bucket,
                        (AnalogySide::Left, AnalogySide::Right) => &mut lr_bucket,
                        (AnalogySide::Right, AnalogySide::Left) => &mut rl_bucket,
                        _ => unimplemented!("Not clear on how/if categorical analogies mix with sided"),
                    };

                    bucket.degree += match_degree;
                    bucket.count += 1;

                    let mut output_item = analogy_item.clone();
                    output_item.degree = match_degree;

                    out.insert(output_item);
                }
                _ => {}
            };

            // TODO 2:
            // * How does this compose across multiple levels of Associative analogy? Eg ("Smile" : "Mouth") : ("Wink" : "Eye")
            // * How do we represent this partial matching. Presumably via some scoring mechanism
        }

        // Now we have a set of matching items
        // We need to decide if we should invert the members or not.
        // We are not guaranteed to have a clear affinity, or inverse-affinity for the query set
        // It could be mixed - so we have to vote!

        // If ALL of the matches from one side is zero, then the other side is irrelevant
        // let direct_degree = ll_bucket.degree * rr_bucket.degree;
        // let inverse_degree = lr_bucket.degree * rl_bucket.degree;

        // Count up all the hits by
        let direct_count = rr_bucket.count + ll_bucket.count;
        let inverse_count = rl_bucket.count + lr_bucket.count;

        let total_right_count = nonmatching_right_count + rr_bucket.count + rl_bucket.count;
        let total_left_count = nonmatching_left_count + ll_bucket.count + lr_bucket.count;

        // If nothing matches, then we're done here
        if direct_count == 0 && inverse_count == 0 {
            return None;
        }

        // Gotta have at least one member on each side, or we're done
        if total_right_count == 0 || total_left_count == 0 {
            return None;
        }

        // Scale *both* sides based on the opposing match_degree
        // Remember first letter of the bucket is the input analogy item side
        let left_scale_factor = (rr_bucket.degree + rl_bucket.degree) / total_right_count as f32;
        let right_scale_factor = (ll_bucket.degree + lr_bucket.degree) / total_left_count as f32;

        out.scale_lr(left_scale_factor, right_scale_factor);

        // Our output set has a stronger affinity than anti-affinity, so we _do not_ invert it
        if inverse_count > direct_count {
            out.invert()
        }

        Some(out)
    }

    pub fn categorical<I, L, T>(id: I, list: L) -> Self
    where
        L: IntoIterator<Item = T>,
        T: Into<fs::Item<AnalogyMember>>,
        I: Into<SimpleId>,
    {
        let mut set = FuzzySet::from_list(list);

        Analogy { id: id.into(), set }
    }

    pub fn associative<I>(id: I, left: Symbol, right: Symbol) -> Self
    where
        I: Into<SimpleId>,
    {
        let mut set = FuzzySet::new();

        for sm in left.into_iter() {
            set.insert(fs::Item {
                member: AnalogyMember {
                    id: sm.member.id,
                    side: AnalogySide::Left,
                },
                degree: sm.degree,
            });
        }
        for sm in right.into_iter() {
            set.insert(fs::Item {
                member: AnalogyMember {
                    id: sm.member.id,
                    side: AnalogySide::Right,
                },
                degree: sm.degree,
            });
        }
        Analogy { id: id.into(), set }
    }
}

pub struct AnalogyQuery {
    pub set: FuzzySet<AnalogyMember>,
}

impl AnalogyQuery {
    pub fn from_left_right(left: Symbol, right: Symbol) -> Self {
        let mut set = FuzzySet::new();

        for sm in left.into_iter() {
            set.insert(fs::Item {
                member: AnalogyMember {
                    id: sm.member.id,
                    side: AnalogySide::Left,
                },
                degree: sm.degree,
            });
        }
        for sm in right.into_iter() {
            set.insert(fs::Item {
                member: AnalogyMember {
                    id: sm.member.id,
                    side: AnalogySide::Right,
                },
                degree: sm.degree,
            });
        }
        AnalogyQuery { set }
    }
}

impl fs::Member for AnalogyMember {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.id.cmp(&other.id) {
            Ordering::Equal => {}
            o @ _ => return o,
        }
        unimplemented!("TODO 2 - better handle same identity with different sidedness")
        // self.side.cmp(&other.side)
    }

    fn invert(&mut self) -> bool {
        match self.side {
            AnalogySide::Left => {
                self.side = AnalogySide::Right;
                true
            }
            AnalogySide::Right => {
                self.side = AnalogySide::Left;
                true
            }
            AnalogySide::Categorical => false,
        }
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.side {
            AnalogySide::Categorical => "ᐧ",
            AnalogySide::Left => "˱",
            AnalogySide::Right => "˲",
        };
        write!(f, "({}{},{:0.2})", self.id.id, side, item.degree)
    }

    fn display_fmt_set(set: &FuzzySet<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in set.left() {
            if !first {
                write!(f, " ")?;
                item.member.display_fmt(&item, f)?;
            } else {
                first = false;
                item.member.display_fmt(&item, f)?;
            }
        }

        write!(f, " : ")?;

        let mut seen = false;
        for item in set.right() {
            if seen {
                write!(f, " ")?;
                item.member.display_fmt(&item, f)?;
            } else {
                seen = true;
                item.member.display_fmt(&item, f)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

pub enum AnalogyCompare {
    Different,
    Same,
    Inverse,
}

impl fs::Item<AnalogyMember> {
    pub fn acmp(&self, other: &Self) -> AnalogyCompare {
        if self.member.id == other.member.id {
            if self.member.side == other.member.side {
                AnalogyCompare::Same
            } else {
                AnalogyCompare::Inverse
            }
        } else {
            AnalogyCompare::Different
        }
    }
}

impl FuzzySet<AnalogyMember> {
    pub fn scale_lr(&mut self, left_scale_factor: f32, right_scale_factor: f32) {
        for item in self.iter_mut() {
            match item.member.side {
                AnalogySide::Categorical => unimplemented!(),
                AnalogySide::Left => item.degree *= left_scale_factor,
                AnalogySide::Right => item.degree *= right_scale_factor,
            }
        }
    }
    pub fn left<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember>> + 'a {
        self.iter().filter(|a| a.member.side == AnalogySide::Left).map(|a| fs::Item {
            member: SymbolMember { id: a.member.id.clone() },
            degree: a.degree,
        })
    }

    pub fn right<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember>> + 'a {
        self.iter().filter(|a| a.member.side == AnalogySide::Right).map(|a| fs::Item {
            member: SymbolMember { id: a.member.id.clone() },
            degree: a.degree,
        })
    }
}

impl std::fmt::Display for Analogy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.id.id, self.set)
    }
}
impl std::fmt::Display for AnalogyQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.set)
    }
}

#[cfg(test)]
mod test {
    use crate::{sym, FuzzySet, Symbol};

    use super::{Analogy, AnalogyMember};

    #[test]
    fn lesser_weights_through_imperfect_analogy() {
        // TODO 1 - reconcile this experiment with the core crate

        let a1 = Analogy::associative("a1", sym!["A", "B", "C", "D"], sym!["X", "Y", "Z"]);
        // Notice this analogy is inverse to
        let a2 = Analogy::associative("a2", sym!["X", "F"], sym!["A", "B", "Q"]);
        println!("{}", a1);
        println!("{}", a2);

        // interrogate the first analogy with the second
        let mut b: FuzzySet<AnalogyMember> = a1.interrogate(&a2).unwrap();

        // Resultant set is scaled based on the common members and their degree
        // and also inverted to match the sidedness of the query analogy
        assert_eq!(format!("{}", b), "[(X,0.67) : (A,0.50) (B,0.50)]");

        // TODO 2 - Continue authoring this test case meaningfully
        // // So, We've interrogated a1 with a2 and gotten some "naturally" members with < 1 weights.
        // // How do we clean up this scenario to be more realistic?
        // // "interrogation" only makes sense in the context of a query – Not just blindly rubbing two analogies together
        // // How do we formulate a query using a corpus of prior analogies?

        // There exists some catagory which is descibable with all of the following terms, modulo any potential mistakes
        // let c1 = Analogy::categorical("c1", &["doughnut", "bun", "pastry", "cruller", "sweet roll"]);

        // let a3 = Analogy::associative("a2", sym!["Q", "R"], sym!["F", "G"]);
        // // let c = b.interrogate(&a3).unwrap();
        // // This does not work, because interrogation (rightly) does not return an analogy. Someone would have to claim that analogy on
        // // the basis of some prior query

        // let Analogy::from_left_right("a2", sym!["Q", "R"], sym!["F", "G"]);

        // println!("{}", c);
    }
}
