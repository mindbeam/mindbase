#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
mod analogy;
mod atom;
mod symbol;

use analogy::*;
use atom::*;
use symbol::*;

fn main() {
    experiment1()
}

fn experiment1() {
    // In this experiment, we are approxmiating the following MBQL
    // $x = Bind("Hot")
    // $y = Ground($x : "Cold")

    let mut x = Symbol::null();
    let mut y = Symbol::null();

    // For simplicity, lets say these are all the analogies in the system
    let candidates = [Analogy::new(AtomId("a1"), sym("Hot1"), sym("Cold1")),
                      Analogy::new(AtomId("a2"), sym("Hot2"), sym("Cold2")),
                      Analogy::new(AtomId("a3"), sym("Cold3"), sym("Hot3"))];

    // NOTE - this should have an unassigned Spin, because it's a match pair
    let search_pair = AtomVec::from_left_right("Hot", "Cold");
    println!("Searching for {}", search_pair.diag_lr());

    for candidate in &candidates {
        let v = candidate.intersect(&search_pair).expect("All of the above should match");
        x.atoms.extend(v.left());

        y.atoms.insert(Atom { id:     candidate.id.clone(),
                              spin:   Spin::Up, /* This is WRONG for a3. It should be Down because the order of the
                                                 * association is reversed.
                                                 * how do we fix this? */
                              side:   Side::Middle,
                              weight: 1.0, });
    }

    println!("symbol x is: [{}]", x.atoms.diag());
    println!("symbol y is: [{}]", y.atoms.diag());
}

fn experiment2() {
    // $x = Bind("Hot")
    // $y = Ground(($x : "Cold") : ("Spicy" : "Mild"))
    let a1 = Analogy::new(AtomId("a1"), sym("Hot1"), sym("Cold1"));
    let a2 = Analogy::new(AtomId("a2"), sym("Cold1"), sym("Hot1"));

    // NOTE - this should have an unassigned Spin, because it's a match pair
    let search_pair = AtomVec::from_left_right("Hot1", "Cold1");
    // pair.insert(atom("Cold2").left());
    // pair.insert(atom("Hot2").right());

    // println!("{:?}", a1);
    println!("{:?}", search_pair);

    // This compares the analogy to a SymbolPair
    let p1 = a1.intersect(&search_pair).unwrap();
    // THIS is what we actually want to use for the bound symbol

    println!("p1: {:?}", p1.diag_lr());

    let p2 = a2.intersect(&search_pair).unwrap();
    println!("p2: {:?}", p2.diag_lr());
}
