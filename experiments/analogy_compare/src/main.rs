mod analogy;
mod atom;
mod symbol;

use analogy::*;
use atom::*;
use symbol::*;

fn main() {
    let a1 = Analogy::new(sym("Hot"), sym("Cold"));

    let mut pair = AtomVec::new();
    pair.insert(atom("Cold").left());
    pair.insert(atom("Hot").right());

    // println!("{:?}", a1);
    println!("{:?}", pair);

    // This compares the analogy to a SymbolPair
    let a3 = a1.intersect(pair);
    println!("{:?}", a3);
    // Hot' Hot' Cold. Cold.
}
