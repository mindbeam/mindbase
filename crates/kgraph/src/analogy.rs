pub mod associative;
pub mod categorical;
pub mod query;

use self::{associative::AssociativeAnalogy, categorical::CategoricalAnalogy};

pub enum Analogy<E>
where
    E: Clone + std::fmt::Display + std::cmp::Ord,
{
    Associative(AssociativeAnalogy<E>),
    Categorical(CategoricalAnalogy<E>),
}

// impl std::fmt::Display for Analogy {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}={}", self.set)
//     }
// }

#[cfg(test)]
mod test {

    use crate::fuzzyset::FuzzySet;

    use super::{query::AnalogyQuery, AssociativeAnalogy};

    #[test]
    fn lesser_weights_through_imperfect_analogy() {
        // TODO 1 - reconcile this experiment with the core crate

        // Notice this analogy is inverse to
        let a = AssociativeAnalogy::new(sym!["X", "F"], sym!["A", "B", "Q"]);
        println!("{}", a);

        let q = AnalogyQuery::new((sym!["A", "B", "C", "D"], sym!["X", "Y", "Z"]));
        println!("{}", q);

        // interrogate the first analogy with the second
        let mut b: FuzzySet = q.interrogate(&a).unwrap();

        // Resultant set is scaled based on the common members and their degree
        // and also inverted to match the sidedness of the query analogy
        assert_eq!(format!("{}", b), "[(X,0.67) : (A,0.50) (B,0.50)]");

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
