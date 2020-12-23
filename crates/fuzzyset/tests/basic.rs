use std::f32::INFINITY;

use fuzzyset::{fuzzyset::FuzzySet, test_util::SimpleMember};

#[test]
fn royalty() {
    // let mut corpus = Vec::new();

    //          Man  Woman  King Queen Apple Orange
    // Gender -1.00   1.00 -0.95  0.97  0.00   0.01
    // Royal   0.01   0.02  0.93  0.95 -0.01   0.00
    // Age     0.03   0.02  0.70  0.69  0.03  -0.02
    // Food    0.09   0.01  0.02  0.01  0.95   0.97

    // infinite-width vector space

    // There exists some entity "man" defined as
    let man = FuzzySet::from_list(&[("gender", -1.0), ("royal", 0.01), ("age", 0.03), ("food", 0.09)]);
    let woman = FuzzySet::from_list(&[("gender", 1.0), ("royal", 0.02), ("age", 0.02), ("food", 0.01)]);
    let king = FuzzySet::from_list(&[("gender", -0.95), ("royal", 0.93), ("age", 0.7), ("food", 0.02)]);
    let queen = FuzzySet::from_list(&[("gender", 0.97), ("royal", 0.95), ("age", 0.69), ("food", 0.01)]);
    let apple = FuzzySet::from_list(&[("gender", 0.0), ("royal", -0.01), ("age", 0.03), ("food", 0.95)]);
    let orange = FuzzySet::from_list(&[("gender", 0.01), ("royal", 0.00), ("age", -0.02), ("food", 0.97)]);

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
