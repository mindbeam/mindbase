use std::cmp::Ordering;

// TODO 1 - How do we reconcile Associative Analogies and Subject-Predicate-Object statements?
// Arguably they are mirrors of each other. SPO declares the predicate, whereas AA infers it.
// TODO 2 - Experiment – explore the reciprocality of SPO / AA
// TODO 3 - Experiment - explore the inferability of SPO -> AA and AA -> SPO
// TODO 3 - Experiment - explore the relationship between the AA identity and the Predicate identity

// (Parent  : Child)   :  (Commander  : Subordinate)
// (Parent [X] Child) <-> (Commander [X] Subordinate)
// where X is all of the predicates which associate the terms ( according to whom? )
// Eg: [in charge of], [older than], [more resolute than], []

use crate::{
    fuzzyset::{self as fs, FuzzySet},
    traits::Member,
};

/// # PolarFuzzySet<M>
/// A wrapper around a FuzzySet<M> which wraps each member to include a specific polarity representing
/// the directionality of association/implication. For example, consider the following trivial PolarFuzzySet
/// containing four members: [Small, Tiny : Big, Large]
/// Two of these members have a "left" polarization, and the other two have a "right" polarization.
/// Because the intention here is easy invertability, the selection of which is "left" and which is
/// "right" is immaterial, so long as that selection is consistent within each individual PolarFuzzySet.
///
/// A PolarFuzzySet could alternately be conceptualized as *two* wrapped FuzzySets - one corresponding to
/// Left, and one corresponding to Right, each with two members in the above example. The reason we do not
/// actually want to represent this with two nonpolar-fuzzysets is because we have to compare and merge
/// _all_ PolarFuzzySet members (regardless of polarity) pairwise when we are comparing to other
/// PolarFuzzySets. We will selectively permute the polarity of all output members based on the strongest
/// affinities between the two sets. This is much more efficient and greatly simpler versus having to compare
/// four sets in this fashion.

/// # Note on PolarFuzzySet membership upon an anti-polar insertion:
/// We believe each sub-member should be unique within the polar set, but some question
/// remains as to how to handle the addition of a conflicting polarity, and how that might
/// come about. For example, given an explicit contraction like "I like turtles" AND not("I like turtles"),
/// This might be expressed as two set members with opposing polarity.
/// Eg: `polarset.insert([T-Left, T-Right])` (pseudocode)
/// So, what should the final PolarFuzzySet contain? Should it be a null set? [],
/// or polarity preserving [T-Left], or polarity overwriting [T-Right], or somehow [T-Center]?
/// Perhaps Left should be represented as -1.0 and Right as 1.0, then averagge the two to get 0.0.
/// Arguably null set and Center/0.0 are funtionally identical, so this
///
/// NOTE(!) this is decidedly different from a statement such as "I like Bob, but I don't like Bob"
/// Elements of such a statement describing a love-hate relationship may _feel_ polar from a human perspective,
/// but it's not logically polar because these sub statements have different senses which capture the attributes
/// that are liked and the attributes that are not liked.
pub struct PolarFuzzySet<M>(FuzzySet<PolarMember<M>>)
where
    M: Member;

#[derive(Debug, Clone)]
pub struct PolarMember<M: Member> {
    pub member: M,
    pub polarity: Polarity,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Polarity {
    Left,
    Right,
}

impl<M> PolarMember<M>
where
    M: Member,
{
    pub fn invert_polarity(&mut self) -> bool {
        match self.polarity {
            Polarity::Left => {
                self.polarity = Polarity::Right;
            }
            Polarity::Right => {
                self.polarity = Polarity::Left;
            }
        }
        true
    }

    pub fn transmute_left(mut self) -> Self {
        self.polarity = Polarity::Left;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.polarity = Polarity::Right;
        self
    }
}

impl<M> PolarFuzzySet<M>
where
    M: Member,
{
    pub fn new() -> Self {
        Self(FuzzySet::new())
    }
    pub fn from_left_right<Left, Right, IntoItemLeft, IntoItemRight>(left: Left, right: Right) -> Self
    where
        Left: IntoIterator<Item = IntoItemLeft>,
        Right: IntoIterator<Item = IntoItemRight>,
        IntoItemLeft: Into<fs::Item<M>>,
        IntoItemRight: Into<fs::Item<M>>,
    {
        let mut set = FuzzySet::new();

        for item in left.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: PolarMember {
                    member: item.member,
                    polarity: Polarity::Left,
                },
                degree: item.degree,
            });
        }
        for item in right.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: PolarMember {
                    member: item.member,
                    polarity: Polarity::Right,
                },
                degree: item.degree,
            });
        }
        PolarFuzzySet(set)
    }
    pub fn insert(&mut self, item: fs::Item<PolarMember<M>>) {
        self.0.insert(item)
    }
    pub fn union<'a, T>(&'a mut self, other: T)
    where
        T: IntoIterator<Item = fs::Item<PolarMember<M>>>,
    {
        for item in other {
            self.insert(item)
        }
    }
    pub fn scale_lr(&mut self, left_scale_factor: f32, right_scale_factor: f32) {
        for item in self.0.iter_mut() {
            match item.member.polarity {
                Polarity::Left => item.degree *= left_scale_factor,
                Polarity::Right => item.degree *= right_scale_factor,
            }
        }
    }
    pub fn invert_polarity(&mut self) {
        for item in self.0.iter_mut() {
            item.member.invert_polarity();
        }
    }
    /// Return an iterator over over Left-polarized members within the PolarFuzzySet
    pub fn left<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
        self.0
            .iter()
            .filter(|a| a.member.polarity == Polarity::Left)
            .map(|a| fs::Item {
                member: a.member.member.clone(),
                degree: a.degree,
            })
    }

    /// Return an iterator over Right-polarized members within the PolarFuzzySet
    pub fn right<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
        self.0
            .iter()
            .filter(|a| a.member.polarity == Polarity::Right)
            .map(|a| fs::Item {
                member: a.member.member.clone(),
                degree: a.degree,
            })
    }

    /// TODO 1 - rewrite this description
    /// identify the subset of this analogy's fuzzy-set which intersect the comparison set
    /// and conditionally invert the sidedness of the resultant set to match the comparison set
    pub fn interrogate(&self, other: &PolarFuzzySet<M>) -> Option<PolarFuzzySet<M>> {
        let mut iter = other.0.iter().merge_join_by(self.0.iter(), |a, b| a.member.cmp(&b.member));

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

        let mut out = PolarFuzzySet::new();

        use itertools::{EitherOrBoth, Itertools};
        for either in iter {
            match either {
                EitherOrBoth::Right(other_item) => {
                    // Present in query, but not present in analogy
                    match &other_item.member.polarity {
                        Polarity::Left => nonmatching_left_count += 1,
                        Polarity::Right => nonmatching_right_count += 1,
                    }
                }
                EitherOrBoth::Both(other_item, my_item) => {
                    // We've got a hit

                    // Scale the degree of the matching item by that of the query
                    let match_degree = other_item.degree * my_item.degree;

                    let bucket = match (&other_item.member.polarity, &my_item.member.polarity) {
                        (Polarity::Left, Polarity::Left) => &mut ll_bucket,
                        (Polarity::Right, Polarity::Right) => &mut rr_bucket,
                        (Polarity::Left, Polarity::Right) => &mut lr_bucket,
                        (Polarity::Right, Polarity::Left) => &mut rl_bucket,
                        // _ => unimplemented!("Not clear on how/if categorical analogies mix with sided"),
                    };

                    bucket.degree += match_degree;
                    bucket.count += 1;

                    let mut output_item = other_item.clone();
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
            out.invert_polarity()
        }

        Some(out)
    }
}

impl<M> Member for PolarMember<M>
where
    M: Member,
{
    /// Compare this member to another
    fn cmp(&self, other: &Self) -> Ordering {
        // Ignore polarity. Set membership is determined by the inner member identity alone
        // not the polarity
        self.member.cmp(&other.member)
    }
}

impl<M> std::fmt::Display for PolarMember<M>
where
    M: Member,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let polarity = match self.polarity {
            Polarity::Left => "+",
            Polarity::Right => "-",
        };
        write!(f, "{}{}", polarity, self.member)
    }
}

impl<M> std::fmt::Display for PolarFuzzySet<M>
where
    M: Member,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.0.iter().filter(|a| a.member.polarity == Polarity::Left) {
            if !first {
                write!(f, " {}^{:0.2}", item.member, item.degree)?;
            } else {
                first = false;
                write!(f, "{}^{:0.2}", item.member, item.degree)?;
            }
        }

        write!(f, " : ")?;

        let mut seen = false;
        for item in self.0.iter().filter(|a| a.member.polarity == Polarity::Right) {
            if !first {
                write!(f, " {}^{:0.2}", item.member, item.degree)?;
            } else {
                first = false;
                write!(f, "{}^{:0.2}", item.member, item.degree)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl<M> IntoIterator for PolarFuzzySet<M>
where
    M: Member + Clone,
{
    type IntoIter = std::vec::IntoIter<fs::Item<PolarMember<M>>>;
    type Item = fs::Item<PolarMember<M>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{fuzzyset::FuzzySet, test_util::SimpleMember};

    use super::{PolarFuzzySet, Polarity};

    #[test]
    fn polar_inference() {
        let subjects: [PolarFuzzySet<SimpleMember>; 4] = [
            PolarFuzzySet::from_left_right(&["Hot"], &["Cold"]),      // hot vs cold
            PolarFuzzySet::from_left_right(&["Caliente"], &["Fria"]), // hot vs cold
            PolarFuzzySet::from_left_right(&["Hot"], &["Caliente"]),  // hot things
            PolarFuzzySet::from_left_right(&["Cold"], &["Fria"]),     // cold things
        ];

        // Hot : Cold :: Calliente : ?
        let a = PolarFuzzySet::from_left_right(&["Hot"], &["Cold"]);
        let blank: [SimpleMember; 0] = [];
        let b = PolarFuzzySet::from_left_right(&["Calliente"], &blank);

        let mut alpha = PolarFuzzySet::new();
        let mut beta = PolarFuzzySet::new();
        for subject in &subjects {
            if let Some(result) = a.interrogate(subject) {
                println!("Alpha {} >>> {}", subject, result);
                alpha.union(result)
            }
        }
        for subject in &subjects {
            if let Some(result) = b.interrogate(subject) {
                println!("Beta {} >>> {}", subject, result);
                beta.union(result);
            }
        }

        let foo = alpha.interrogate(&beta).unwrap();

        println!("foo: {}", foo);
    }
    #[test]
    fn polar_fuzzy_set() {
        let mut left_result = FuzzySet::new();
        let mut result = PolarFuzzySet::new();

        // Imagine we wanted to look up all the Entities for Claims related to Artifacts "Hot" and "Cold"
        let query: PolarFuzzySet<SimpleMember> =
            PolarFuzzySet::from_left_right(&["Hot1", "Hot2", "Hot3"], &["Cold1", "Cold2", "Cold3"]);
        println!("Query is: {}", query);

        // For simplicity, lets say these are all the analogies in the system
        let candidates: [PolarFuzzySet<SimpleMember>; 2] = [
            PolarFuzzySet::from_left_right(&["Hot1", "Hot2", "Heated1"], &["Mild1", "Mild2", "Cold3"]),
            //               NORMAL to query     ^2/3 Match                          ^1/3 match
            PolarFuzzySet::from_left_right(&[("Cold1", 1.0), ("Cold2", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^ 2/3 match                              ^ 1/3 match
            // PolarFuzzySet::from_left_right(&[("Cold3", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^1/3  match   ^ 1/3 Match
        ];

        for candidate in &candidates {
            let v = query.interrogate(&candidate).expect("All of the above should match");
            println!("Interrogate {} >>> {}", candidate, v);

            // TODO 3 - QUESTION: should the union of the resultant query output sets (for each candidate analogy) bear equal weight in the
            // output set? That seems screwy! Presumably It should be some sort of a weighted union across all candidate
            // analogies, but how do we do this?
            left_result.union(v.left());

            result.union(v);
        }

        println!("Result is: {}", result);

        println!("Union of left results is: {}", left_result);
        let result_left = FuzzySet::from_list(result.left());
        println!("Left of union results is: {}", result_left);
        assert_eq!(left_result, result_left);

        // assert_eq!(format!("{}", y), "");
    }

    #[test]
    fn lesser_weights_through_imperfect_analogy() {
        // Notice this analogy is inverse tom
        let a = PolarFuzzySet::from_left_right(&["X", "F"], &["A", "B", "Q"]);
        println!("Set A: {}", a);

        let q = PolarFuzzySet::from_left_right(&["A", "B", "C", "D"], &["X", "Y", "Z"]);
        println!("Set Q: {}", q);

        // interrogate the first analogy with the second
        let mut b = a.interrogate(&q).unwrap();
        println!("Interrogated set: {}", q);

        // Resultant set is scaled based on the common members and their degree
        // and also inverted to match the sidedness of the query analogy
        assert_eq!(format!("{}", b), "[+X^0.67 :  -A^0.50 -B^0.50]");

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

        // TODO 1 - Validate this test case
    }
}
