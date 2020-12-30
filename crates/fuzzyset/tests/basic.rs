use std::f32::INFINITY;

use fuzzyset::{fuzzyset::FuzzySet, test_util::SimpleMember};

#[test]
fn basic_vector_space() {
    // This example illustrates a self-coherant vector space similar to that created by word2vec.
    // In such a scenario, weights are determined by a neural network hidden layer which is trained
    // to predict semantic similarity based on proximity.

    // such self-coherant weights can be problematic however, because they require the iterative refinement to achieve
    // coherancy with other members of the vector space.

    // ***
    //     // I can conceive of two approaches:
    //     // 1. We could select this weight semi-arbitrarily, relying on the law of averages and recombination
    //     //    with other claims in order converge on a stable _set_ of entities for a given concept
    //     // 2. We could potentially eschew weight from the entry process entirely, such that there is no "soft similarity"
    //     // which is expressable – but instead only abstract similarity (or antisimilarity)
    //     // The strategy here is essentially the same however, insofar as we must calculate a similarity
    //     // based on available entity relationships
    //     // Questions:
    //     // Do these relationships bear a weight of 1.0 / -1.0 ? or do they contain some nonspecific notion of
    //     // association/anti-association?
    // ***

    //          Man  Woman  King Queen Apple Orange
    // Female -1.00   1.00 -0.95  0.97  0.00   0.01
    // Royal   0.01   0.02  0.93  0.95 -0.01   0.00
    // Age     0.03   0.02  0.70  0.69  0.03  -0.02
    // Food    0.09   0.01  0.02  0.01  0.95   0.97

    // infinite-width vector space

    // Predefined symbols with predefined weights
    // man: female-1.0, royal+0.01, age+0.03, food+0.09
    let man = FuzzySet::from_list(&[("female", -1.0), ("royal", 0.01), ("age", 0.03), ("food", 0.09)]);
    let woman = FuzzySet::from_list(&[("female", 1.0), ("royal", 0.02), ("age", 0.02), ("food", 0.01)]);
    let king = FuzzySet::from_list(&[("female", -0.95), ("royal", 0.93), ("age", 0.7), ("food", 0.02)]);
    let queen = FuzzySet::from_list(&[("female", 0.97), ("royal", 0.95), ("age", 0.69), ("food", 0.01)]);
    let apple = FuzzySet::from_list(&[("female", 0.0), ("royal", -0.01), ("age", 0.03), ("food", 0.95)]);
    let orange = FuzzySet::from_list(&[("female", 0.01), ("royal", 0.00), ("age", -0.02), ("food", 0.97)]);

    let corpus = vec![&man, &woman, &king, &queen, &apple, &orange];

    let q: FuzzySet<SimpleMember> = &(&king - &man) + &woman;

    // QUESTION: IF we can represent FuzzySets recursively such that each FuzzySet is itself an entity,
    //           Then might it make sense to implement this as `nearest_item` on FuzzySet?
    let nearest: &FuzzySet<SimpleMember> = corpus
        .iter()
        .fold((f64::INFINITY, None), |acc, set| {
            let dist = q.euclidean_distance(set); //.expect("these sets should be safe to compare");

            if dist < acc.0 {
                (dist, Some(*set))
            } else {
                acc
            }
        })
        .1
        .expect("we should have a nearest");

    // println!("queen: {}", queen);
    // println!("k-m+q: {}", q);
    // println!("near:  {}", nearest);
    assert_eq!(nearest, &queen);

    // TODO 2 - construct these *without* directly specifying their degree

    // Lets try to construct something approximating this space without specifying any weights
    // female : woman
    // royal:queen
    // royal:king
    // man:woman::king:queen
    // apple:orange
    // gender : male
    // gender : female

    // a dog isn't very royal
    // corpus.push(PolarFuzzySet::from_dipole(&["dog"], &[("royal", 0.1)]));
    // // a cat isn't very royal either, but somehow posesses more "royalness" than a dog
    // corpus.push(PolarFuzzySet::from_dipole(&["cat"], &[("royal", 0.3)]));
    // // a queen is very royal
    // corpus.push(PolarFuzzySet::from_dipole(&["queen"], &[("royal", 0.99)]));

    // let q = PolarFuzzySet::from_monopole(&["royal"]);

    // let mut result = PolarFuzzySet::new();

    // for c in corpus {
    //     // Not sure if union is right here
    //     result.union(c.interrogate_with(&q).unwrap());
    // }

    // // FAILING TEST CASE
    // assert_eq!(format!("{}", result), "[-royal^1.0 : +cat^0.30 +dog^0.1 +queen^0.99]");
}
