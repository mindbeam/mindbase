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
    // $x = Bind("Hot")
    // $y = Ground($x : "Cold")

    let mut x = AtomVec::new();
    let mut y = AtomVec::new();

    let a1 = Analogy::new(AtomId("a1"), sym("Hot"), sym("Cold"));
    let a2 = Analogy::new(AtomId("a2"), sym("Cold"), sym("Hot"));

    // NOTE - this should have an unassigned Spin, because it's a match pair
    let pair = AtomVec::from_left_right("Hot", "Cold");
    println!("{:?}", pair);

    // This is the analogy we are testing
    match a1.intersect(pair.clone()) {
        None => {
            panic!("should match");
        },
        Some(v) => {
            let foo = v.left();
            x.extend(v.left());
            // y.insert(a1);
        },
    }

    // THIS is what we actually want to use for the bound symbol

    println!("x: {:?}", x.diag());

    // let p2 = a2.intersect(pair.clone()).unwrap();
    // println!("p2: {:?}", p2.diag_lr());
}

fn experiment2() {
    let a1 = Analogy::new(AtomId("a1"), sym("Hot1"), sym("Cold1"));
    let a2 = Analogy::new(AtomId("a2"), sym("Cold1"), sym("Hot1"));

    // NOTE - this should have an unassigned Spin, because it's a match pair
    let pair = AtomVec::from_left_right("Hot1", "Cold1");
    // pair.insert(atom("Cold2").left());
    // pair.insert(atom("Hot2").right());

    // println!("{:?}", a1);
    println!("{:?}", pair);

    // This compares the analogy to a SymbolPair
    let p1 = a1.intersect(pair.clone()).unwrap();
    // THIS is what we actually want to use for the bound symbol

    println!("p1: {:?}", p1.diag_lr());

    let p2 = a2.intersect(pair.clone()).unwrap();
    println!("p2: {:?}", p2.diag_lr());
}
