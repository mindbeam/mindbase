use super::{
    fuzzyset as fs,
    fuzzyset::FuzzySet,
    simpleid::*,
    symbol::*,
};

use itertools::{
    EitherOrBoth,
    Itertools,
};

use std::cmp::Ordering;

pub struct Analogy {
    pub id:  SimpleId,
    pub set: FuzzySet<AnalogyMember>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AnalogySide {
    Catagorical,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct AnalogyMember {
    pub id:   SimpleId,
    pub side: AnalogySide,
}

impl AnalogyMember {
    pub fn new(id: SimpleId) -> Self {
        AnalogyMember { id,
                        side: AnalogySide::Catagorical }
    }

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

#[macro_export]
#[warn(unused_macros)]
macro_rules! atomvec {
    ($($x:expr),+ $(,)?) => (
        AtomVec::new_from_array(Box::new([$($x),+]))
    );
}

impl Analogy {
    pub fn query(&self, other: &AnalogyQuery) -> Option<FuzzySet<AnalogyMember>> {
        // QUESTION - Eventually we will have to trim the output set for performance reasons. Presumably by output weight
        // descending.            How well or poorly does this converge? (TODO 2 - Run an experiment to determine this)

        // outer-join the two sorted lists together based only on ID
        // The list is sorted by ID and side, which means they are in the correct order.
        // TODO 2 - Think about contradictions including the same atom: Eg Atom 123 is on both the left and the right side, or
        // included with opposite Spin on the same side
        let mut iter = self.set
                           .iter()
                           .merge_join_by(other.set.iter(), |a, b| a.member.id.cmp(&b.member.id));

        // Execution plan:
        // * We're comparing all Atoms for both symbols within this analogy to a Two-sided AtomVec containing *Candidate* Atoms
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

        let mut ll_pdegree = 0f32;
        let mut rr_pdegree = 0f32;
        let mut lr_pdegree = 0f32;
        let mut rl_pdegree = 0f32;
        // let mut ll_ndegree = 0f32;
        // let mut rr_ndegree = 0f32;
        // let mut lr_ndegree = 0f32;
        // let mut rl_ndegree = 0f32;
        let mut ll_count = 0u32;
        let mut rr_count = 0u32;
        let mut lr_count = 0u32;
        let mut rl_count = 0u32;

        let mut out: Vec<fs::Item<AnalogyMember>> = Vec::new();

        for either in iter {
            match either {
                EitherOrBoth::Both(analogy_item, query_item) => {
                    // So we know we match on ID

                    let degree;
                    println!("MATCH {:?} <-> {:?}", analogy_item.member.side, query_item.member.side);
                    match (&analogy_item.member.side, &query_item.member.side) {
                        (AnalogySide::Left, AnalogySide::Left) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            ll_pdegree += degree;
                            // ll_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            ll_count += 1;
                        },
                        (AnalogySide::Right, AnalogySide::Right) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            rr_pdegree += degree;
                            // rr_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            rr_count += 1;
                        },
                        (AnalogySide::Left, AnalogySide::Right) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            lr_pdegree += degree;
                            // lr_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            lr_count += 1;
                        },
                        (AnalogySide::Right, AnalogySide::Left) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            rl_pdegree += degree;
                            // rl_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            rl_count += 1;
                        },

                        _ => unimplemented!(),
                    };

                    let mut output_item = analogy_item.clone();
                    output_item.pdegree = degree;
                    output_item.ndegree = 0.0;

                    // assuming we are only inverting the sidedness (conditionally)
                    // then this should be sufficient, as it's based on the analogy_item.
                    // Such inversion would be all-or-none for this analogy
                    // if regular {
                    out.push(output_item);
                    // } else {
                    // tmp.push(Polarity::Inverse(output_item));
                    // }
                },
                _ => {},
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
        let forward_degree = ll_pdegree * rr_pdegree; // + lr_ndegree * rl_ndegree;
        let reverse_degree = lr_pdegree * rl_pdegree; // + ll_ndegree * rr_pdegree;

        println!("FORWARD {:0.1} REVERSE {:0.1}", forward_degree, reverse_degree);

        if forward_degree > reverse_degree {
            println!("Forward");

            // The output factor of the left symbol is a function of how well the right matched, and vice versa
            let left_factor = rr_pdegree / rr_count as f32;
            let right_factor = ll_pdegree / ll_count as f32;

            let mut f = FuzzySet::new();
            for mut item in out.drain(..) {
                item.scale_lr(left_factor, right_factor);
                f.insert(item);
            }

            Some(f)
        } else if reverse_degree > 0.0 {
            println!("Reverse: {:?}", out);
            // Same as above, except we're using the numbers from the inverse comparisons
            let left_factor = rl_pdegree / rl_count as f32;
            let right_factor = lr_pdegree / lr_count as f32;
            println!("Factors: {:0.1} <-> {:0.1}", left_factor, right_factor);
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

    pub fn from_left_right<I>(id: I, left: Symbol, right: Symbol) -> Self
        where I: Into<SimpleId>
    {
        let mut set = FuzzySet::new();

        for sm in left.into_iter() {
            set.insert(fs::Item { member:  AnalogyMember { id:   sm.member.id,
                                                           side: AnalogySide::Left, },
                                  pdegree: sm.pdegree,
                                  ndegree: sm.ndegree, });
        }
        for sm in right.into_iter() {
            set.insert(fs::Item { member:  AnalogyMember { id:   sm.member.id,
                                                           side: AnalogySide::Right, },
                                  pdegree: sm.pdegree,
                                  ndegree: sm.ndegree, });
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
            set.insert(fs::Item { member:  AnalogyMember { id:   sm.member.id,
                                                           side: AnalogySide::Left, },
                                  pdegree: sm.pdegree,
                                  ndegree: sm.ndegree, });
        }
        for sm in right.into_iter() {
            set.insert(fs::Item { member:  AnalogyMember { id:   sm.member.id,
                                                           side: AnalogySide::Right, },
                                  pdegree: sm.pdegree,
                                  ndegree: sm.ndegree, });
        }
        AnalogyQuery { set }
    }
}

impl fs::Member for AnalogyMember {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.id.cmp(&other.id) {
            Ordering::Equal => {},
            o @ _ => return o,
        }
        self.side.cmp(&other.side)
    }

    fn invert(&mut self) {
        self.side = match self.side {
            AnalogySide::Left => AnalogySide::Right,
            AnalogySide::Right => AnalogySide::Left,
            AnalogySide::Catagorical => AnalogySide::Catagorical,
        };
    }

    fn display_fmt(&self, item: &fs::Item<Self>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.side {
            AnalogySide::Catagorical => "ᐧ",
            AnalogySide::Left => "˱",
            AnalogySide::Right => "˲",
        };
        write!(f, "{}{}{:0.1}", self.id.id, side, item.pdegree)
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

        write!(f, "] <-> [")?;

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
        match self.member.side {
            AnalogySide::Catagorical => unimplemented!(),
            AnalogySide::Left => self.pdegree = left_factor,
            AnalogySide::Right => self.pdegree = right_factor,
        }
    }
}

impl FuzzySet<AnalogyMember> {
    pub fn left<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember>> + 'a {
        self.iter().filter(|a| a.member.side == AnalogySide::Left).map(|a| {
                                                                      fs::Item { member:  SymbolMember { id: a.member.id.clone() },
                                                                                 pdegree: a.pdegree,
                                                                                 ndegree: a.ndegree, }
                                                                  })
    }

    pub fn right<'a>(&'a self) -> impl Iterator<Item = fs::Item<SymbolMember>> + 'a {
        self.iter().filter(|a| a.member.side == AnalogySide::Right).map(|a| {
            fs::Item { member:  SymbolMember { id: a.member.id.clone() },
                       pdegree: a.pdegree,
                       ndegree: a.ndegree, }
        })
    }
}
