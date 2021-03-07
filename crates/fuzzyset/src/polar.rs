use std::cmp::Ordering;

// TODO 2 - How do we reconcile Associative Analogies and Subject-Predicate-Object statements?
// Arguably they are mirrors of each other. SPO declares the predicate, whereas AA infers it.
// TODO 3 - Experiment – explore the reciprocality of SPO / AA
// TODO 4 - Experiment - explore the inferability of SPO -> AA and AA -> SPO
// TODO 5 - Experiment - explore the relationship between the AA identity and the Predicate identity

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
/// Two of these members have a "negative" polarization, and the other two have a "positive" polarization.
/// Because the intention here is easy invertability, the selection of which is "negative" and which is
/// "positive" is immaterial, so long as that selection is consistent within each individual PolarFuzzySet.
///
/// A PolarFuzzySet could alternately be conceptualized as *two* wrapped FuzzySets - one corresponding to
/// negative, and one corresponding to positive, each with two members in the above example. The reason we do not
/// actually want to represent this with two nonpolar-fuzzysets is because we have to compare and merge
/// _all_ PolarFuzzySet members (regardless of polarity) pairwise when we are comparing to other
/// PolarFuzzySets. We will selectively permute the polarity of all output members based on the strongest
/// affinities between the two sets. This is much more efficient and greatly simpler versus having to compare
/// four sets in this fashion.

/// # Note on PolarFuzzySet membership upon an anti-polar insertion:
/// We believe each sub-member should be unique within the polar set, but some question
/// remains as to how to handle the addition of a conflicting polarity, and how that might
/// come about. For example, given an explicit contradiction like "I like turtles" AND not("I like turtles"),
/// This might be expressed as two set members with opposing polarity.
/// Eg: `polarset.insert([-T, +T])` (pseudocode)
/// So, what should the final PolarFuzzySet contain? Should it be a null set? [],
/// or polarity preserving [-T], or polarity overwriting [+T], or somehow neither - nor + [~T]?
/// Perhaps negative should be represented as -1.0 and positive as 1.0, then averagge the two to get 0.0.
/// Arguably null set and middle/0.0 are funtionally identical, so this
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

    pub fn transmute_n(mut self) -> Self {
        self.polarity = Polarity::Negative;
        self
    }

    pub fn transmute_p(mut self) -> Self {
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
    pub fn scale_np(&mut self, n_scale_factor: f64, p_scale_factor: f64) {
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
    pub fn invert_polarity_and_scale_np(&mut self, n_scale_factor: f64, p_scale_factor: f64) {
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
    /// Return an iterator over over negative-polarized members within the PolarFuzzySet
    pub fn negative<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
        self.0
            .iter()
            .filter(|a| a.member.polarity == Polarity::Negative)
            .map(|a| fs::Item {
                member: a.member.member.clone(),
                degree: a.degree,
            })
    }

    /// Return an iterator over positive-polarized members within the PolarFuzzySet
    pub fn positive<'a>(&'a self) -> impl Iterator<Item = fs::Item<M>> + 'a {
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
            degree: f64,
            count: u32,
        };

        // We need to sum up the degrees, and the count of each matching item
        // The polarity of which is according to the query
        let mut n_bucket: Bucket = Default::default();
        let mut p_bucket: Bucket = Default::default();
        let mut n_inverse_bucket: Bucket = Default::default();
        let mut p_inverse_bucket: Bucket = Default::default();

        // Count the corpus expansions
        let mut ce_n_bucket: Bucket = Default::default();
        let mut ce_p_bucket: Bucket = Default::default();

        // We also need to count the query expansions
        let mut qe_n_bucket: Bucket = Default::default();
        let mut qe_p_bucket: Bucket = Default::default();

        let mut matching_corpus = PolarFuzzySet::new();

        let mut corpus_expansion = Vec::new();

        // This never gets inverted
        let mut query_expansion = Vec::new();

        use itertools::{EitherOrBoth, Itertools};
        for either in iter {
            match either {
                EitherOrBoth::Left(my_item) => {
                    //
                    // println!("Corpus Only {}", my_item);
                    let bucket: &mut Bucket = match &my_item.member.polarity {
                        Polarity::Negative => &mut ce_n_bucket,
                        Polarity::Positive => &mut ce_p_bucket,
                    };
                    bucket.count += 1;
                    bucket.degree += my_item.degree;

                    corpus_expansion.push(my_item.clone());
                }
                EitherOrBoth::Right(query_item) => {
                    // println!("Query Only {}", query_item);
                    // Present in the set, but not present in query
                    let bucket: &mut Bucket = match &query_item.member.polarity {
                        Polarity::Negative => &mut qe_n_bucket,
                        Polarity::Positive => &mut qe_p_bucket,
                    };
                    bucket.count += 1;
                    bucket.degree += query_item.degree;

                    query_expansion.push(query_item.clone());
                }
                EitherOrBoth::Both(my_item, query_item) => {
                    // We've got a hit
                    // println!("Match {}", my_item);
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

        // Members which were in the corpus, and we did not match but will include
        corpus_expansion.into_iter().for_each(|i| matching_corpus.insert(i));

        // Queries may include additional members which do not match the corpus. These are considered expansions of the symbols
        // the nonreductive counts include expansions AND bucketed matches by query polarity

        let ce_buckets = if inverse_count > direct_count {
            (ce_p_bucket, ce_n_bucket)
        } else {
            (ce_n_bucket, ce_p_bucket)
        };

        // Todo selectively invert the ce part of this
        let total_n_count = ce_buckets.0.count + qe_n_bucket.count + n_bucket.count + n_inverse_bucket.count;
        let total_p_count = ce_buckets.1.count + qe_p_bucket.count + p_bucket.count + p_inverse_bucket.count;

        let query_n_count = qe_n_bucket.count + n_bucket.count + n_inverse_bucket.count;
        let query_p_count = qe_p_bucket.count + p_bucket.count + p_inverse_bucket.count;

        let n_scale_factor = if query_p_count == 0 {
            1.0
        } else {
            (qe_p_bucket.degree + p_bucket.degree + p_inverse_bucket.degree) / total_p_count as f64
        };
        let p_scale_factor = if query_n_count == 0 {
            1.0
        } else {
            (qe_n_bucket.degree + n_bucket.degree + n_inverse_bucket.degree) / total_n_count as f64
        };

        //not trying to match one one side at all means the opposing side scale factor is 1.0

        if inverse_count > direct_count {
            matching_corpus.invert_polarity();

            // Do not invert the query expansion, because it's always the correct polarity
            query_expansion.into_iter().for_each(|i| matching_corpus.insert(i));

            matching_corpus.scale_np(p_scale_factor, n_scale_factor);
        } else {
            query_expansion.into_iter().for_each(|i| matching_corpus.insert(i));
            matching_corpus.scale_np(n_scale_factor, p_scale_factor);
        }

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
    use mindbase_hypergraph::Hypergraph;
    use toboggan_kv::adapter::BTreeAdapter;

    use crate::{fuzzyset::FuzzySet, test_util::SimpleMember};

    use super::{PolarFuzzySet, Polarity};

    // The goal is to EXPAND my symbols such that they correlate to diverse ontologies.

    // Note: In these test cases, we are using SimpleMember, which makes it easier to manage enumerated entities.
    // Eg Hot1 and Hot2 are different identities, both with the text payload of "Hot"
    // This maps loosely to the notion of Artifacts "Hot" and entities "Hot1", "Hot2" so as to deconflate labels with concepts.

    #[test]
    fn analogy_with_expansive_corpus() {
        let mut c = PolarFuzzySet::from_dipole(&["hot", "picante"], &["mild", "suave"]);
        let mut q = PolarFuzzySet::from_dipole(&["hot"], &["mild"]);

        // "hot" and "mild" are insufficiently specific in this case. What if the corpus were [hot, calido] : [mild,templado]?
        // Each party starts by defining their own narrow symbols. We want to converge (and expand/clarify) those symbols
        // by "triangulation", not just blind assumption.

        // Corpus symbols are a superset of query symbols.  Result is expanded with *partial*
        // confidence reduction based on the match percentage of the *opposing* pole
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-hot^0.50 -picante^0.50 : +mild^0.50 +suave^0.50]"
        );

        // Polarity is determined by the query, not the corpus
        q.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-mild^0.50 -suave^0.50 : +hot^0.50 +picante^0.50]"
        );

        c.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-mild^0.50 -suave^0.50 : +hot^0.50 +picante^0.50]"
        );
    }
    #[test]
    fn minimal_polar_inference() {
        let mut c = PolarFuzzySet::from_dipole(&["hot", "picante"], &["mild", "suave"]);
        let q = PolarFuzzySet::from_monopole(&["hot", "picante"]);

        let result = c.interrogate_with(&q).unwrap();
        // When the N side fully matches, so does the P
        // Our query has no P to detract from the N so they both strongly match
        assert_eq!(format!("{}", result), "[-hot^1.00 -picante^1.00 : +mild^1.00 +suave^1.00]");

        // Output should be robust against the corpus being inverted
        c.invert_polarity();
        let result = c.interrogate_with(&q).unwrap();
        assert_eq!(format!("{}", result), "[-hot^1.00 -picante^1.00 : +mild^1.00 +suave^1.00]");

        assert_eq!(
            format!("{}", FuzzySet::from_list(result.positive())),
            "{mild^1.00 suave^1.00}"
        );
    }
    #[test]
    fn analogy_with_half_expansive_corpus() {
        // Corpus symbols intersect query symbols, but ONE side is a subset
        let c = PolarFuzzySet::from_dipole(&["hot", "picante"], &["mild", "suave"]);
        let q = PolarFuzzySet::from_dipole(&["hot", "picante"], &["mild"]);

        // Result should be expanded, but with *partial* confidence on the OPPOSING pole
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-hot^0.50 -picante^0.50 : +mild^1.00 +suave^1.00]"
        );
    }
    #[test]
    fn analogy_with_reductive_corpus() {
        let mut c = PolarFuzzySet::from_dipole(&["hot"], &["mild"]);
        let mut q = PolarFuzzySet::from_dipole(&["hot", "calido"], &["mild", "templado"]);

        // THE CORPUS IS LESS SPECIFIC THAN WE'RE ASKING FOR, AND YET WE HAVE LESSER CONFIDENCE?

        // Corpus symbols are a subset of query symbols on both poles.
        // Confidence reduction is applied to *both* poles by their opposite
        // because both poles represent reductive matches
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-calido^1.00 -hot^1.00 : +mild^1.00 +templado^1.00]"
        );

        // Polarity is determined by the query, not the corpus
        q.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-mild^1.00 -templado^1.00 : +calido^1.00 +hot^1.00]"
        );

        c.invert_polarity();
        assert_eq!(
            format!("{}", c.interrogate_with(&q).unwrap()),
            "[-mild^1.00 -templado^1.00 : +calido^1.00 +hot^1.00]"
        );
    }

    #[test]
    fn polysemy() {
        // Mouse (Squeak, click)
        // Door (Walk through that, open the)
    }

    #[test]
    fn recursive_polar_inference() -> Result<(), std::io::Error> {
        let g = Hypergraph::memory();

        // Containment of Sets WITHIN SimpleMembers is simply not tenable due to cloning vs aliasing.
        // We must deal with the grap claim vs artifact issue now in order to proceed with this case,
        // and the fuzzyset basic vector space test

        let subject = PolarFuzzySet::from_dipole(
            &[("n", g.put_vertex_weight(PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]))?)],
            &[(
                "p",
                g.put_vertex_weight(PolarFuzzySet::from_dipole(&["Caliente"], &["Fria"]))?,
            )],
        );

        let query = PolarFuzzySet::from_dipole(
            &[("n", PolarFuzzySet::from_dipole(&["Hot"], &["Cold"]))],
            &[("p", PolarFuzzySet::from_monopole(&["Caliente"]))],
        );

        // TODO 2 - recurse
        let foo = subject.interrogate_with(&query).unwrap();

        Ok(())
    }

    // #[test]
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
        let candidates: [PolarFuzzySet<SimpleMember>; 3] = [
            PolarFuzzySet::from_dipole(&["Hot1", "Hot2", "Heated1"], &["Mild1", "Mild2", "Cold3"]),
            //               NORMAL to query     ^2/3 Match                          ^1/3 match
            PolarFuzzySet::from_dipole(&[("Cold1", 1.0), ("Cold2", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^ 2/3 match                              ^ 1/3 match
            PolarFuzzySet::from_dipole(&[("Cold3", 1.0)], &[("Hot3", 1.0)]),
            //               INVERSE to query    ^1/3  match   ^ 1/3 Match
        ];

        for candidate in &candidates {
            let v = candidate.interrogate_with(&query).expect("All of the above should match");
            println!("Interrogate {} >>> {}", candidate, v);

            // TODO 3 - QUESTION: should the union of the resultant query output sets (for each candidate analogy) bear equal weight in the
            // output set? That seems screwy! Presumably It should be some sort of a weighted union across all candidate
            // analogies, but how do we do this?
            left_result.union(v.negative());

            result.union(v);
        }

        println!("Result is: {}", result);

        println!("Union of left results is: {}", left_result);
        let result_left = FuzzySet::from_list(result.negative());
        println!("Left of union results is: {}", result_left);
        assert_eq!(left_result, result_left);

        // assert_eq!(format!("{}", y), "");
    }

    #[test]
    fn lesser_weights_through_imperfect_analogy() {
        // Feminine Royalty
        let a = PolarFuzzySet::from_dipole(&["Woman", "Girl"], &["Queen", "Princess"]);
        println!("Set A: {}", a);

        // Monarch
        let mut q = PolarFuzzySet::from_dipole(&["Queen", "King"], &["Man", "Woman"]); // order is irrelevant
        println!("Set Q: {}", q);

        // interrogate the first polar set with the second
        let mut b = a.interrogate_with(&q).unwrap();
        println!("Interrogated set: {}", b);

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
