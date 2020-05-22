use super::{
    atom::*,
    symbol::*,
};

use itertools::{
    EitherOrBoth,
    Itertools,
};

pub struct Analogy {
    pub id:  AtomId,
    pub vec: AtomVec,
}

impl Analogy {
    pub fn new(id: AtomId, mut left: Symbol, mut right: Symbol) -> Self {
        let mut vec = AtomVec::new();

        for atom in left.drain(..) {
            vec.insert(atom.transmute_left())
        }
        for atom in right.drain(..) {
            vec.insert(atom.transmute_right())
        }

        Analogy { id, vec }
    }

    pub fn intersect(&self, other: &AtomVec) -> Option<AtomVec> {
        let mut out = AtomVec::new();

        // QUESTION - Eventually we will have to trim the output set for performance reasons. Presumably by output weight
        // descending.            How well or poorly does this converge? (TODO 2 - Run an experiment to determine this)
        // TODO 2 - Think about contradictions including the same atom: Eg Atom 123 is on both the left and the right side, or
        // included with opposite Spin on the same side
        let mut iter = self.vec.iter().merge_join_by(other.iter(), |a, b| a.id.cmp(&b.id));

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

        let mut ll_weight = 0f32;
        let mut rr_weight = 0f32;
        let mut lr_weight = 0f32;
        let mut rl_weight = 0f32;
        let mut ll_count = 0u32;
        let mut rr_count = 0u32;
        let mut lr_count = 0u32;
        let mut rl_count = 0u32;

        for either in iter {
            match either {
                EitherOrBoth::Both(analogy_atom, query_atom) => {
                    // So we know we match on ID
                    // In theory, Query atom spin is always UP (which means that Atom is a poor abstraction for the query)

                    match (&analogy_atom.side, &query_atom.side) {
                        (AnalogySide::Left, AnalogySide::Left) => {
                            ll_weight += analogy_atom.weight * query_atom.weight;
                            ll_count += 1;
                        },
                        (AnalogySide::Right, AnalogySide::Right) => {
                            rr_weight += analogy_atom.weight * query_atom.weight;
                            rr_count += 1;
                        },
                        (AnalogySide::Left, AnalogySide::Right) => {
                            lr_weight += analogy_atom.weight * query_atom.weight;
                            lr_count += 1;
                        },
                        (AnalogySide::Right, AnalogySide::Left) => {
                            rl_weight += analogy_atom.weight * query_atom.weight;
                            rl_count += 1;
                        },

                        _ => unimplemented!(),
                    };

                    let mut output_atom = query_atom.clone();
                    output_atom.mutate_weight(analogy_atom.weight);

                    // TODO 1 - Even thought we know the weight, we don't yet know IF this atom is to be included
                    // in the output set. The way the output weight is calculated may change as well
                    out.insert(output_atom);
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

        let forward_weight = ll_weight * rr_weight;
        let reverse_weight = lr_weight * rl_weight;

        if forward_weight > reverse_weight {
            // The output factor of the left symbol is a function of how well the right matched, and vice versa
            let left_factor = rr_weight / rr_count as f32;
            let right_factor = ll_weight / ll_count as f32;

            for atom in out.iter_mut() {
                // TODO 1 the query atom weight SHOULD be multiplied by the Analogy atom weight, but we would need two places to
                // store that before we know what the winning polarity is >_> Output side should match that of the
                match atom.side {
                    AnalogySide::Middle => unimplemented!(),
                    AnalogySide::Left => atom.mutate_weight(left_factor),
                    AnalogySide::Right => atom.mutate_weight(right_factor),
                }
            }
            Some(out)
        } else if reverse_weight > 0.0 {
            // Same as above, except we're using the numbers from the inverse comparisons
            let left_factor = rl_weight / rl_count as f32;
            let right_factor = lr_weight / lr_count as f32;

            for atom in out.iter_mut() {
                // Output side should match that of the
                match atom.side {
                    AnalogySide::Middle => unimplemented!(),
                    AnalogySide::Left => atom.mutate_weight(left_factor),
                    AnalogySide::Right => atom.mutate_weight(right_factor),
                }
            }
            Some(out)
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
}
