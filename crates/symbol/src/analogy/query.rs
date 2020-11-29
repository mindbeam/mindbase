use crate::{
    analogy::associative::AssociativeAnalogyMember, analogy::associative::Side, fuzzyset::FuzzySet, AssociativeAnalogy, Entity,
};

pub struct AnalogyQuery<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    pub set: FuzzySet<AssociativeAnalogyMember<E>>,
}

impl<E> AnalogyQuery<E>
where
    E: Entity,
{
    // Create a new analogy query
    pub fn new<Q>(query: Q) -> Self
    where
        Q: Into<FuzzySet<AssociativeAnalogyMember<E>>>,
    {
        Self { set: query.into() }
    }

    /// identify the subset of this analogy's fuzzy-set which intersect the comparison set
    /// and conditionally invert the sidedness of the resultant set to match the comparison set
    pub fn interrogate(&self, analogy: &AssociativeAnalogy<E>) -> Option<FuzzySet<AssociativeAnalogyMember<E>>> {
        // FuzzySets are always sorted by ID (side is disabled for now), so we can outer join
        let mut iter = analogy
            .set
            .iter()
            .merge_join_by(self.set.iter(), |a, b| a.member.entity.cmp(&b.member.entity));

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

        use itertools::{EitherOrBoth, Itertools};
        for either in iter {
            match either {
                EitherOrBoth::Right(analogy_item) => {
                    // Present in query, but not present in analogy
                    match &analogy_item.member.side {
                        Side::Left => nonmatching_left_count += 1,
                        Side::Right => nonmatching_right_count += 1,
                    }
                }
                EitherOrBoth::Both(analogy_item, query_item) => {
                    // We've got a hit

                    // Scale the degree of the matching item by that of the query
                    let match_degree = analogy_item.degree * query_item.degree;

                    let bucket = match (&analogy_item.member.side, &query_item.member.side) {
                        (Side::Left, Side::Left) => &mut ll_bucket,
                        (Side::Right, Side::Right) => &mut rr_bucket,
                        (Side::Left, Side::Right) => &mut lr_bucket,
                        (Side::Right, Side::Left) => &mut rl_bucket,
                        // _ => unimplemented!("Not clear on how/if categorical analogies mix with sided"),
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
}

impl<E> std::fmt::Display for AnalogyQuery<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.set)
    }
}
