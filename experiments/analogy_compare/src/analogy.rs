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
    pub fn intersect(&self, other: &Analogy) -> Option<FuzzySet<AnalogyMember>> {
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

        enum Polarity {
            Regular(fs::Item<AnalogyMember>),
            Goofy(fs::Item<AnalogyMember>),
        };
        let mut tmp: Vec<Polarity> = Vec::new();

        for either in iter {
            match either {
                EitherOrBoth::Both(analogy_item, query_item) => {
                    // So we know we match on ID

                    let regular;
                    let degree;
                    match (&analogy_item.member.side, &query_item.member.side) {
                        (AnalogySide::Left, AnalogySide::Left) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            ll_pdegree += degree;
                            // ll_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            ll_count += 1;
                            regular = true;
                        },
                        (AnalogySide::Right, AnalogySide::Right) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            rr_pdegree += degree;
                            // rr_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            rr_count += 1;
                            regular = true;
                        },
                        (AnalogySide::Left, AnalogySide::Right) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            lr_pdegree += degree;
                            // lr_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            lr_count += 1;
                            regular = false;
                        },
                        (AnalogySide::Right, AnalogySide::Left) => {
                            degree = analogy_item.pdegree * query_item.pdegree;
                            rl_pdegree += degree;
                            // rl_ndegree += analogy_atom.ndegree * query_atom.ndegree;
                            rl_count += 1;
                            regular = false
                        },

                        _ => unimplemented!(),
                    };

                    let mut output_item = analogy_item.clone();
                    output_item.pdegree = degree;
                    output_item.ndegree = 0.0;

                    // assuming we are only inverting the sidedness (conditionally)
                    // then this should be sufficient, as it's based on the analogy_item.
                    // Such inversion would be all-or-none for this analogy
                    if regular {
                        tmp.push(Polarity::Regular(output_item));
                    } else {
                        tmp.push(Polarity::Goofy(output_item));
                    }
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

        if forward_degree > reverse_degree {
            // The output factor of the left symbol is a function of how well the right matched, and vice versa
            let left_factor = rr_pdegree / rr_count as f32;
            let right_factor = ll_pdegree / ll_count as f32;

            let mut f = FuzzySet::new();
            for item in tmp.drain(..) {
                match item {
                    Polarity::Regular(mut i) => {
                        i.scale_lr(left_factor, right_factor);
                        f.insert(i);
                    },
                    Polarity::Goofy(mut i) => {
                        i.invert();
                        i.scale_lr(left_factor, right_factor);
                        f.insert(i);
                    },
                }
            }

            Some(f)
        } else if reverse_degree > 0.0 {
            // Same as above, except we're using the numbers from the inverse comparisons
            let left_factor = rl_pdegree / rl_count as f32;
            let right_factor = lr_pdegree / lr_count as f32;

            let mut f = FuzzySet::new();
            for item in tmp.drain(..) {
                match item {
                    Polarity::Regular(mut i) => {
                        i.invert();
                        i.scale_lr(left_factor, right_factor);
                        f.insert(i);
                    },
                    Polarity::Goofy(mut i) => {
                        i.scale_lr(left_factor, right_factor);
                        f.insert(i);
                    },
                }
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

    pub fn diag(&self) -> String {
        // let mut out: Vec<String> = Vec::new();
        // for atom in self.iter() {
        //     let spin = match atom.spin {
        //         Spin::Up => "↑",
        //         Spin::Down => "↓",
        //     };
        //     //˰˯

        //     let side = match atom.side {
        //         AnalogySide::Middle => "ᐧ",
        //         AnalogySide::Left => "˱",
        //         AnalogySide::Right => "˲",
        //     };

        //     assert!(atom.weight <= 1.0, "Invalid atom weight");

        //     let mut weight = format!("{:.2}", atom.weight);
        //     if atom.weight < 1.0 {
        //         weight.remove(0);
        //     } else {
        //         weight.truncate(0);
        //     }

        //     out.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
        //                                                                 .to_string());
        // }

        // out.join(",")
        unimplemented!()
    }

    pub fn diag_lr(&self) -> String {
        // for atom in self.iter() {
        //     let side = match atom.side {
        //         AnalogySide::Middle => "ᐧ",
        //         AnalogySide::Left => "˱",
        //         AnalogySide::Right => "˲",
        //     };

        //     assert!(atom.weight <= 1.0, "Invalid atom weight");

        //     let mut weight = format!("{:.2}", atom.weight);
        //     if atom.weight < 1.0 {
        //         weight.remove(0);
        //     } else {
        //         weight.truncate(0);
        //     }

        //     match atom.side {
        //         AnalogySide::Middle => unimplemented!(),
        //         AnalogySide::Left => {
        //             lefts.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
        //                                                                           .to_string())
        //         },
        //         AnalogySide::Right => {
        //             rights.push(format!("{}{}{}{}", atom.id.id, side, spin, weight).bg_color(Color::Green)
        //                                                                            .to_string())
        //         },
        //     }
        // }

        // format!("{} <-> {}", lefts.join(","), rights.join(",")).to_string()
        unimplemented!()
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
}

enum AnalogyCompare {
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
