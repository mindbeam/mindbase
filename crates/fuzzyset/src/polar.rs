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

#[derive(Clone, Debug)]
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
    Negative,
    Positive,
}

impl<M> PolarMember<M>
where
    M: Member,
{
    pub fn invert_polarity(&mut self) {
        match self.polarity {
            Polarity::Negative => {
                self.polarity = Polarity::Positive;
            }
            Polarity::Positive => {
                self.polarity = Polarity::Negative;
            }
        }
    }

    pub fn transmute_left(mut self) -> Self {
        self.polarity = Polarity::Negative;
        self
    }

    pub fn transmute_right(mut self) -> Self {
        self.polarity = Polarity::Positive;
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
    pub fn from_dipole<IterN, IterP, IntoN, IntoP>(negative: IterN, positive: IterP) -> Self
    where
        IterN: IntoIterator<Item = IntoN>,
        IterP: IntoIterator<Item = IntoP>,
        IntoN: Into<fs::Item<M>>,
        IntoP: Into<fs::Item<M>>,
    {
        let mut set = FuzzySet::new();

        for item in negative.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: PolarMember {
                    member: item.member,
                    polarity: Polarity::Negative,
                },
                degree: item.degree,
            });
        }
        for item in positive.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: PolarMember {
                    member: item.member,
                    polarity: Polarity::Positive,
                },
                degree: item.degree,
            });
        }
        PolarFuzzySet(set)
    }
    pub fn from_monopole<IterN, IntoN>(negative: IterN) -> Self
    where
        IterN: IntoIterator<Item = IntoN>,
        IntoN: Into<fs::Item<M>>,
    {
        let mut set = FuzzySet::new();

        for item in negative.into_iter() {
            let item = item.into();
            set.insert(fs::Item {
                member: PolarMember {
                    member: item.member,
                    polarity: Polarity::Negative,
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
    pub fn scale_np(&mut self, n_scale_factor: f32, p_scale_factor: f32) {
        for item in self.0.iter_mut() {
            match item.member.polarity {
                Polarity::Negative => item.degree *= n_scale_factor,
                Polarity::Positive => item.degree *= p_scale_factor,
            }
        }
    }
    pub fn invert_polarity(&mut self) {
        for item in self.0.iter_mut() {
            item.member.invert_polarity();
        }
    }
    pub fn invert_polarity_and_scale_np(&mut self, n_scale_factor: f32, p_scale_factor: f32) {
        for item in self.0.iter_mut() {
            // item.member.invert_polarity();
            match item.member.polarity {
                Polarity::Negative => {
                    item.member.polarity = Polarity::Positive;
                    item.degree *= p_scale_factor;
                }
                Polarity::Positive => {
                    item.member.polarity = Polarity::Negative;
                    item.degree *= n_scale_factor;
                }
            }
        }
    }
    /// Return an iterator over over Left-polarized members within the PolarFuzzySet
    pub fn left<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
        self.0
            .iter()
            .filter(|a| a.member.polarity == Polarity::Negative)
            .map(|a| fs::Item {
                member: a.member.member.clone(),
                degree: a.degree,
            })
    }

    /// Return an iterator over Right-polarized members within the PolarFuzzySet
    pub fn right<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
        self.0
            .iter()
            .filter(|a| a.member.polarity == Polarity::Positive)
            .map(|a| fs::Item {
                member: a.member.member.clone(),
                degree: a.degree,
            })
    }

    /// # interrogate
    ///
    /// Interrogate the subject PolarFuzzySet such that members from the interrogating set which are common to the
    /// subject set may be used to discover and infer new members of the subject set which posess some polar relationship
    /// to the known members of the interrogating set.
    ///
    /// Hot(1) : Cold :: Calliente : ?
    ///
    /// The result members of each polarity are scaled based the percentage match of the members of the opposing polarity.
    /// If a majority of common members between the two sets match with inverse polarity, all members of the subject set
    /// are inverted to conform to the polarity of the interrogating set.
    pub fn interrogate_with(&self, query: &PolarFuzzySet<M>) -> Option<PolarFuzzySet<M>> {
        // self is the interrogator.
        // Polarities of the result set will be conformed to the interrogating set

        let iter = self.0.iter().merge_join_by(query.0.iter(), |a, b| a.member.cmp(&b.member));

        #[derive(Default)]
        struct Bucket {
            degree: f32,
            count: u32,
        };

        // We need to sum up the degrees, and the count of each matching item
        // The polarity of which is according to the query
        let mut n_bucket: Bucket = Default::default();
        let mut p_bucket: Bucket = Default::default();
        let mut n_inverse_bucket: Bucket = Default::default();
        let mut p_inverse_bucket: Bucket = Default::default();

        // Count the corpus expansions
        let mut ce_left_count = 0u32;
        let mut ce_right_count = 0u32;

        // We also need to count the query expansions
        let mut qe_left_count = 0u32;
        let mut qe_right_count = 0u32;

        let mut matching_corpus = PolarFuzzySet::new();

        let mut corpus_expansion = Vec::new();

        // This never gets inverted
        let mut query_expansion = Vec::new();

        use itertools::{EitherOrBoth, Itertools};
        for either in iter {
            match either {
                EitherOrBoth::Left(my_item) => {
                    //
                    println!("MY ITEM ONLY {}", my_item);
                    match &my_item.member.polarity {
                        Polarity::Negative => ce_left_count += 1,
                        Polarity::Positive => ce_right_count += 1,
                    }
                    corpus_expansion.push(my_item.clone());
                }
                EitherOrBoth::Right(query_item) => {
                    println!("QUERY ITEM ONLY {}", query_item);
                    // Present in the set, but not present in query
                    match &query_item.member.polarity {
                        Polarity::Negative => qe_left_count += 1,
                        Polarity::Positive => qe_right_count += 1,
                    }
                    query_expansion.push(query_item.clone());
                }
                EitherOrBoth::Both(my_item, query_item) => {
                    // We've got a hit
                    println!("BOTH {}", my_item);
                    // Scale the degree of the matching item by that of the query
                    let match_degree = query_item.degree * my_item.degree;

                    let bucket = match (&query_item.member.polarity, &my_item.member.polarity) {
                        (Polarity::Negative, Polarity::Negative) => &mut n_bucket,
                        (Polarity::Positive, Polarity::Positive) => &mut p_bucket,
                        (Polarity::Negative, Polarity::Positive) => &mut n_inverse_bucket,
                        (Polarity::Positive, Polarity::Negative) => &mut p_inverse_bucket,
                        // _ => unimplemented!("Not clear on how/if categorical analogies mix with sided"),
                    };

                    bucket.degree += match_degree;
                    bucket.count += 1;

                    let mut output_item = my_item.clone();
                    output_item.degree = match_degree;

                    matching_corpus.insert(output_item);
                }
                _ => {}
            };

            // TODO 2:
            // * How does this compose across multiple levels of Associative analogy? Eg ("Smile" : "Mouth") : ("Wink" : "Eye")
        }

        // Now we have a set of matching items
        // We need to decide if we should invert the members or not.
        // We are not guaranteed to have a clear affinity, or inverse-affinity for the query set
        // It could be mixed - so we have to vote!

        // Count up all the hits
        let direct_count = p_bucket.count + n_bucket.count;
        let inverse_count = p_inverse_bucket.count + n_inverse_bucket.count;

        // If nothing matches, then we're done here
        if direct_count + inverse_count == 0 {
            return None;
        }

        corpus_expansion.into_iter().for_each(|i| matching_corpus.insert(i));

        // Queries may include additional members which do not match the corpus. These are considered expansions of the symbols
        // the nonreductive counts include expansions AND bucketed matches by query polarity
        let nonreductive_n_count = qe_left_count + n_bucket.count + n_inverse_bucket.count;
        let nonreductive_p_count = qe_right_count + p_bucket.count + p_inverse_bucket.count;

        // TODO 1 - handle monopole queries
        let n_scale_factor = (p_bucket.degree + p_inverse_bucket.degree) / nonreductive_p_count as f32;
        let p_scale_factor = (n_bucket.degree + n_inverse_bucket.degree) / nonreductive_n_count as f32;

        if inverse_count > direct_count {
            matching_corpus.invert_polarity();

            // Do not invert the query expansion, because it's always the correct polarity
            query_expansion.into_iter().for_each(|i| matching_corpus.insert(i));

            matching_corpus.scale_np(p_scale_factor, n_scale_factor);
        } else {
            query_expansion.into_iter().for_each(|i| matching_corpus.insert(i));
            matching_corpus.scale_np(n_scale_factor, p_scale_factor);
        }

        // Gotta have at least one member on each side, or we're done
        // if total_right_count == 0 || total_left_count == 0 {
        //     return None;
        // }
        Some(matching_corpus)
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
            Polarity::Negative => "-",
            Polarity::Positive => "+",
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
        for item in self.0.iter().filter(|a| a.member.polarity == Polarity::Negative) {
            if !first {
                write!(f, " {}^{:0.2}", item.member, item.degree)?;
            } else {
                first = false;
                write!(f, "{}^{:0.2}", item.member, item.degree)?;
            }
        }

        write!(f, " :")?;
        for item in self.0.iter().filter(|a| a.member.polarity == Polarity::Positive) {
            write!(f, " {}^{:0.2}", item.member, item.degree)?;
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

    // The goal is to EXPAND my symbols such that they correlate to diverse ontologies.

    // Note: In these test cases, we are using SimpleMember, which makes it easier to manage enumerated entities.
    // Eg Hot1 and Hot2 are different identities, both with the text payload of "Hot"
    // This maps loosely to the notion of Artifacts "Hot" and entities "Hot1", "Hot2" so as to deconflate labels with concepts.

    #[test]
    fn perfect_analogy_with_expansive_corpus() {
        let mut c = PolarFuzzySet::from_dipole(&["Hot", "Calliente"], &["Cold", "Fria"]);
        let mut q = PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]);

        // WEIRD - the corpus is MORE specific than our query, and yet we are fully confident in the outcome??
        // Hot1,Hot2 compared to Hot1 should be LESS confident, I think
        // What are the network dynamics of this under symbol set size constraint?
        // Presumably we want a set of canonical symbols to emerge, which posess with an optimal network of hops between said cano

        // Corpus symbols are a perfect superset of query symbols. Result is expanded with full confidence
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Calliente^1.00 -Hot^1.00 : +Cold^1.00 +Fria^1.00]"
        );

        // Polarity is determined by the query, not the corpus
        q.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Cold^1.00 -Fria^1.00 : +Calliente^1.00 +Hot^1.00]"
        );

        c.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Cold^1.00 -Fria^1.00 : +Calliente^1.00 +Hot^1.00]"
        );
    }
    #[test]
    fn perfect_analogy_with_reductive_corpus() {
        let mut c = PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]);
        let mut q = PolarFuzzySet::from_dipole(&["Hot", "Calliente"], &["Cold", "Fria"]);

        // THE CORPUS IS LESS SPECIFIC THAN WE'RE ASKING FOR, AND YET WE HAVE LESSER CONFIDENCE?

        // Corpus symbols are a subset of query symbols on both poles.
        // Confidence reduction is applied to *both* poles by their opposite
        // because both poles represent reductive matches
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Calliente^0.50 -Hot^0.50 : +Cold^0.50 +Fria^0.50]"
        );

        // Polarity is determined by the query, not the corpus
        q.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Cold^0.50 -Fria^0.50 : +Calliente^0.50 +Hot^0.50]"
        );

        c.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Cold^0.50 -Fria^0.50 : +Calliente^0.50 +Hot^0.50]"
        );
    }

    #[test]
    fn perfect_analogy_with_half_reductive_corpus() {
        // Corpus symbols intersect query symbols, but ONE side is a subset
        let c = PolarFuzzySet::from_dipole(&["Hot", "Calliente"], &["Cold"]);
        let q = PolarFuzzySet::from_dipole(&["Hot", "Calliente"], &["Cold", "Fria"]);

        // Result should be expanded, but with *partial* confidence on the OPPOSING pole
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-Calliente^0.50 -Hot^0.50 : +Cold^1.00 +Fria^1.00]"
        );
    }

    #[test]
    fn minimal_polar_inference() {
        let subject = PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]);
        let query = PolarFuzzySet::from_monopole(&["Hot"]);

        let result = subject.interrogate_with(&query).unwrap();

        // When the left side fully matches, so does the right, even if our query stated nothing for it
        assert_eq!(format!("{}", result), "[-Hot^1.00 :  +Cold^1.00]");

        // println!("result: {}", result);
        // assert_eq!(result.right().next().unwrap().member.text, "Cold");
    }
    #[test]
    fn recursive_polar_inference() {
        let blank: [SimpleMember; 0] = [];
        let subject = PolarFuzzySet::from_dipole(
            &[("left", PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]))],
            &[("right", PolarFuzzySet::from_dipole(&["Caliente"], &["Fria"]))],
        );

        let query = PolarFuzzySet::from_dipole(
            &[("left", PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]))],
            &[("right", PolarFuzzySet::from_dipole(&["Caliente"], &blank))],
        );

        // TODO 1 - recurse
        let foo = subject.interrogate_with(&query).unwrap();

        println!("foo: {}", foo);

        // And make this less bad
        assert_eq!(
            foo.right()
                .next()
                .unwrap()
                .member
                .set
                .unwrap()
                .right()
                .next()
                .unwrap()
                .member
                .text,
            "Fria"
        );
    }

    #[test]
    fn expansive_then_convergent_network() {
        // Model a scenario with several agents
        // each has some symbolic commonality with immediate neighbors, but little or no commonality with others
        // under this scenario, all agents should converge at least partially (full convergence on a canonical symbol may actually be undesirable)

        todo!()
    }
    #[test]
    fn polar_fuzzy_set() {
        let mut left_result = FuzzySet::new();
        let mut result = PolarFuzzySet::new();

        // Imagine we wanted to look up all the Entities for Claims related to Artifacts "Hot" and "Cold"
        let query: PolarFuzzySet<SimpleMember> =
            PolarFuzzySet::from_dipole(&["Hot1", "Hot2", "Hot3"], &["Cold1", "Cold2", "Cold3"]);
        println!("Query is: {}", query);

        // For simplicity, lets say these are all the analogies in the system
        let candidates: [PolarFuzzySet<SimpleMember>; 2] = [
            PolarFuzzySet::from_dipole(&["Hot1", "Hot2", "Heated1"], &["Mild1", "Mild2", "Cold3"]),
            //               NORMAL to query     ^2/3 Match                          ^1/3 match
            PolarFuzzySet::from_dipole(&[("Cold1", 1.0), ("Cold2", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^ 2/3 match                              ^ 1/3 match
            // PolarFuzzySet::from_left_right(&[("Cold3", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^1/3  match   ^ 1/3 Match
        ];

        for candidate in &candidates {
            let v = candidate.interrogate_with(&query).expect("All of the above should match");
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
        // Royalty
        let a = PolarFuzzySet::from_dipole(&["Woman", "Girl"], &["Queen", "Princess"]);
        // let a = PolarFuzzySet::from_left_right(&["X", "F"], &["A", "B", "Q"]);
        println!("Set A: {}", a);

        // Monarch
        let q = PolarFuzzySet::from_dipole(&["Man", "Woman"], &["Queen", "King"]); // order is irrelevant
        println!("Set Q: {}", q);

        // interrogate the first analogy with the second
        let mut b = a.interrogate_with(&q).unwrap();
        println!("Interrogated set: {}", q);

        // Resultant set is scaled based on the common members and their degree
        // and also inverted to match the sidedness of the query analogy
        // left match is .5, right match is .5
        assert_eq!(format!("{}", b), "[-Woman^0.50 : +Queen^0.50]");

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
