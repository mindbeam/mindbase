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
    /// and conditionally invert the sidedness of the resultant subset to match the comparison set.
    ///
    ///
    pub fn interrogate<'a, T>(&'a self, compare: T) -> Option<FuzzySet<AnalogyMember>>
    where
        T: Into<&'a FuzzySet<AnalogyMember>>,
    {
        // QUESTION - Eventually we will have to trim the output set for performance reasons. Presumably by output weight
        // descending.            How well or poorly does this converge? (TODO 2 - Run an experiment to determine this)

        // outer-join the two sorted lists together based only on ID
        // The list is sorted by ID and side, which means they are in the correct order.

        // FuzzySets are always sorted by ID+Side, so we can use

        // TODO 2 - Think about contradictions including the same atom: Eg Atom 123 is on both the left and the right side, or
        // included with opposite Spin on the same side
        let mut iter = self
            .set
            .iter()
            .merge_join_by(compare.into().iter(), |a, b| a.member.id.cmp(&b.member.id));

        // Execution plan:
        // * We're comparing all Members for both symbols within this analogy to a Two-sided FuzzySet containing *Candidate* Members
        // * At least one Left Atom and one Right Atom must match in order for the relationship to have any weight at all
        // * We're expanding the set of those atoms which are inferred (Spin-adjusted opposite-side Atoms) with a score based on
        // weighted sum of the matching spin-adjusted same-side matches

        // Question: How do we calculate the scores for Spin-adjusted Same-side matches?

        // NOTE: I don't think we can swap sidedness of individual Atoms
        //       I'm pretty sure we have to swap the sidedness of whole thing, or nothing         (extreme temps <> middle temps)
        //       There could be tension between This analogy [hot,warm]<>[cold,cool] and this query pair [hot,cold]<>[warm,cool]
        //       So they will have to vote.
        //       This example should produce a pretty low weight for all of the above atoms
        //       This means we will have to score BOTH AL<>QL + AR<>QR __AND__ AL<>QR + AR<>QL
        //       then whichever one is stronger wins, and we either flip, or don't flip ALL Atoms in the resultant set

        #[derive(Default)]
        struct Avg {
            degree: f32,
            count: u32,
        };
        let mut left_left: Avg = Default::default();
        let mut right_right: Avg = Default::default();
        let mut left_right: Avg = Default::default();
        let mut right_left: Avg = Default::default();

        let mut left_count = 0u32;
        let mut right_count = 0u32;

        // let mut normal_count = 0u32;
        // let mut inverse_count = 0u32;

        let mut out: Vec<fs::Item<AnalogyMember>> = Vec::new();

        for either in iter {
            match either {
                // EitherOrBoth::Left(analogy_item) => {
                // // QUESTION: Do we care if the analogy has more members than the query?
                //     // Present in analogy, not present in query
                //     match &analogy_item.member.side {
                //         AnalogySide::Left => {
                //             left_count += 1;
                //         },
                //         AnalogySide::Right => {
                //             right_count += 1;
                //         },
                //         _ => unimplemented!(),
                //     }
                // },
                EitherOrBoth::Right(analogy_item) => {
                    // Present in query, not present in analogy
                    match &analogy_item.member.side {
                        AnalogySide::Left => {
                            left_count += 1;
                        }
                        AnalogySide::Right => {
                            right_count += 1;
                        }
                        _ => unimplemented!(),
                    }
                }
                EitherOrBoth::Both(analogy_item, query_item) => {
                    // So we know we match on ID

                    let degree = analogy_item.degree * query_item.degree;
                    // println!("MATCH {:?} <-> {:?}", analogy_item.member.side, query_item.member.side);
                    match (&analogy_item.member.side, &query_item.member.side) {
                        (AnalogySide::Left, AnalogySide::Left) => {
                            left_left.degree += degree;
                            left_left.count += 1;
                        }
                        (AnalogySide::Right, AnalogySide::Right) => {
                            right_right.degree += degree;
                            right_right.count += 1;
                        }
                        (AnalogySide::Left, AnalogySide::Right) => {
                            left_right.degree += degree;
                            left_right.count += 1;
                        }
                        (AnalogySide::Right, AnalogySide::Left) => {
                            right_left.degree += degree;
                            right_left.count += 1;
                        }

                        _ => unimplemented!(),
                    };

                    let mut output_item = analogy_item.clone();
                    output_item.degree = degree;

                    // assuming we are only inverting the sidedness (conditionally)
                    // then this should be sufficient, as it's based on the analogy_item.
                    // Such inversion would be all-or-none for this analogy
                    // if regular {
                    out.push(output_item);
                    // } else {
                    // tmp.push(Polarity::Inverse(output_item));
                    // }
                }
                _ => {}
            };

            // The problem is: An associative analogy involves two sets of Atoms, Left and Right, which are associated sets, but
            // NOT associable pairwise as atoms. This is because each set represents a symbol in its own right.
            // The allegation is that SymbolA(Atomset A) is associatively correlated to SymbolB(Atomset B)

            // Given a perfect match of left-handed Atoms, all right-handed Atoms can be inferred.
            // Given a PARTIAL match of left-handed Atoms, The entirety of the subset the right is inferred, but with a degree of
            // confidence commensurate with the number of left-handed matches Commensurately, the inverse is true as
            // well of right handed matches

            // Questions:
            // * How does this compose across multiple levels of Associative analogy? Eg ("Smile" : "Mouth") : ("Wink" : "Eye")
            // * How do we represent this partial matching. Presumably via some scoring mechanism
        }

        // If any of these are zero, it will only detract from the average
        // If ALL of the set from the same side is zero, then the other side won't matter, because we're multiplying by zero
        let forward_degree = left_left.degree * right_right.degree;
        let reverse_degree = left_right.degree * right_left.degree;

        // println!("Left {}: (LL {:0.1} + LR: {:0.1}), Right: {}: (RR: {:0.1} + RL: {:0.1})",
        //  left_count, ll_pdegree, lr_pdegree, right_count, rr_pdegree, rl_pdegree);

        right_count += right_right.count + right_left.count;
        left_count += left_left.count + left_right.count;

        if right_count == 0 || left_count == 0 {
            return None;
        }

        let left_factor = (right_right.degree + right_left.degree) / right_count as f32;
        let right_factor = (left_left.degree + left_right.degree) / left_count as f32;
        // println!("FACTOR LEFT {:0.1} RIGHT {:0.1}", left_factor, right_factor);

        let normal_count = right_left.count + left_right.count;
        let inverse_count = right_right.count + left_left.count;
        if normal_count > inverse_count {
            // println!("Forward");

            // The output factor of the left symbol is a function of how well the right matched, and vice versa

            let mut f = FuzzySet::new();
            for mut item in out.drain(..) {
                item.scale_lr(left_factor, right_factor);
                f.insert(item);
            }

            Some(f)
        } else if inverse_count > 0 {
            // println!("Reverse: {:?}", out);
            // Same as above, except we're using the numbers from the inverse comparisons
            let mut f = FuzzySet::new();
            for mut item in out.drain(..) {
                item.invert();
                item.scale_lr(left_factor, right_factor);
                f.insert(item);
            }

            Some(f)
        } else {
            None
        }

        // if left_weight > 0.0 || right_weight > 0.0 {
        //     // each factor is the average of the opposite-side weights
        //     let left_factor = right_weight / right_count as f32;
        //     let right_factor = left_weight / left_count as f32;

        //     for atom in out.iter_mut() {
        //         // Output side should match that of the
        //         match atom.side {
        //             AnalogySide::Middle => unimplemented!(),
        //             AnalogySide::Left => atom.mutate_weight(left_factor),
        //             AnalogySide::Right => atom.mutate_weight(right_factor),
        //         }
        //     }
        //     Some(out)
        // } else {
        //     None
        // }
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
        self.side.cmp(&other.side)
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
        let mut seen = false;
        for item in set.left() {
            if seen {
                write!(f, ", ")?;
                item.member.display_fmt(&item, f)?;
            } else {
                seen = true;
                item.member.display_fmt(&item, f)?;
            }
        }

        write!(f, " <-> ")?;

        let mut seen = false;
        for item in set.right() {
            if seen {
                write!(f, ", ")?;
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

    pub fn scale_lr(&mut self, left_factor: f32, right_factor: f32) {
        // match self.member.side {
        //     AnalogySide::Catagorical => unimplemented!(),
        //     AnalogySide::Left => self.pdegree = left_factor,
        //     AnalogySide::Right => self.pdegree = right_factor,
        // }
        unimplemented!()
    }
}

impl FuzzySet<AnalogyMember> {
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
        write!(f, "{}[{}]", self.id.id, self.set)
    }
}
impl std::fmt::Display for AnalogyQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.set)
    }
}
