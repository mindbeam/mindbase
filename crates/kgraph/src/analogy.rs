pub mod associative;
pub mod categorical;
pub mod query;

use crate::Entity;

use self::{associative::AssociativeAnalogy, categorical::CategoricalAnalogy};

pub enum Analogy<E>
where
    E: Entity,
{
    Associative(AssociativeAnalogy<E>),
    Categorical(CategoricalAnalogy<E>),
}

#[cfg(test)]
mod test {
    use super::{query::AnalogyQuery, AssociativeAnalogy};
    use crate::{fuzzyset::FuzzySet, prelude::*, testing::SimpleEntity};

    #[test]
    fn lesser_weights_through_imperfect_analogy() {
        // TODO 1 - reconcile this experiment with the core crate

        // Notice this analogy is inverse tom
        let a = AssociativeAnalogy::<SimpleEntity>::new(sym![("X", 1.0), ("F", 1.0)], sym![("A", 1.0), ("B", 1.0), ("Q", 1.0)]);
        println!("{}", a);

        let q = AnalogyQuery::new((
            sym![("A", 1.0), ("B", 1.0), ("C", 1.0), ("D", 1.0)],
            sym![("X", 1.0), ("Y", 1.0), ("Z", 1.0)],
        ));
        println!("{}", q);

        // interrogate the first analogy with the second
        let mut b: FuzzySet<_> = q.interrogate(&a).unwrap();

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
