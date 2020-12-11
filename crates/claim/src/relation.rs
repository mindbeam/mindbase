use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Relation<T, E>
where
    T: NodeType,
    E: NodeInstance,
{
    pub relation_type: T,
    pub from: E,
    pub to: E,
    //  pub amendment: RelationAmendment
}

// Scenarios:
// * Polar-Associative relationship (analogy) between two symbols (two fuzzy claim sets)
//    * There exists some (Polar) essence which these things have in common
//      EG: Dog:Cat (Nonpolar interpretation: Both mammals/pets/furry, Polar: Bigger/Nicer : Smaller/Naughtier/)
//                            (These are the same in some fashion)                (These are different in some fashion)
// * Non-polar Associative (Catagorical analogy) between two symbols (two fuzzy claim sets) - Non-Polar Associative
// * Edge between two nodes of a concrete graph (two single claims, but COULD be fuzzy claim set)
//     What kind of affinity to we need here in order to traverse such an edge?
//     Imagine we uploaded a JSON file, but then someone later augmented the representation
//     Same user? Any trusted user? Same Session?

// ARE ALL CLAIMS EDGES? (Sort of/no)
// A claim is either an

// DIFFERENCE IS POLAR (Associative)
// SAMENESS IS NON-POLAR (Catagorical)
