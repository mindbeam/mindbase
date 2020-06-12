#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
mod analogy;
mod fuzzyset;
mod simpleid;
mod symbol;

// use analogy::*;
use analogy::{
    Analogy,
    AnalogyQuery,
};
use fuzzyset::FuzzySet;
use simpleid::*;
use symbol::*;

fn main() {
    experiment()
}

fn experiment() {
    // In this experiment, we are approxmiating the following MBQL
    // $x = Bind("Hot")
    // $y = Ground($x : "Cold")

    let mut x = Symbol::null();
    let mut y = FuzzySet::new();

    // For simplicity, lets say these are all the analogies in the system
    let candidates = [//
                      Analogy::from_left_right("a1", sym!["Hot1", "Hot2", "Heated1"], sym!["Mild1", "Mild2", "Cold3"]),
                      Analogy::from_left_right("a2", sym!["Hot3"], sym!["Cold1", "Cold2"]),
                      Analogy::from_left_right("a3", sym!["Cold3"], sym!["Hot3"])];

    // Imagine we looked up all AtomIds for all Allegations related to Artifacts "Hot" and "Cold"
    let query = AnalogyQuery::from_left_right(sym!["Hot1", "Hot2", "Hot3"], sym!["Cold1", "Cold2", "Cold3"]);
    println!("Query is: {}", query);

    for candidate in &candidates {
        let v = candidate.interrogate(&query).expect("All of the above should match");
        println!("v is {}", v);

        // QUESTION: should the union of the resultant query output sets (for each candidate analogy) bear equal weight in the
        // output set? That seems screwy! Presumably It should be some sort of a weighted union across all candidate
        // analogies, but how do we do this?
        x.set.union(v.left());

        y.union(v);
    }

    println!("symbol x is: {}", x);
    println!("symbol y is: {}", y);
}

// fn experiment2() {
//     // $x = Bind("Hot")
//     // $y = Ground(($x : "Cold") : ("Spicy" : "Mild"))
//     let a1 = Analogy::new(simpleid("a1"), sym!["Hot1"], sym!["Cold1"]);
//     let a2 = Analogy::new(simpleid("a2"), sym!["Cold1"], sym!["Hot1"]);

//     // NOTE - this should have an unassigned Spin, because it's a match pair
//     let search_pair = AtomVec::from_left_right(atomvec!["Hot1"], atomvec!["Cold1"]);
//     // pair.insert(atom("Cold2").left());
//     // pair.insert(atom("Hot2").right());

//     // println!("{:?}", a1);
//     println!("{:?}", search_pair);

//     // This compares the analogy to a SymbolPair
//     let p1 = a1.intersect(&search_pair).unwrap();
//     // THIS is what we actually want to use for the bound symbol

//     println!("p1: {:?}", p1.diag_lr());

//     let p2 = a2.intersect(&search_pair).unwrap();
//     println!("p2: {:?}", p2.diag_lr());
// }
