mod analogy;
mod atom;
mod symbol;

use analogy::*;
use symbol::*;

fn main() {
    let a1 = Analogy::new(sym("Hot"), sym("Cold"));

    // Temporarily using Analogy::new to construct a pair
    let pair = Analogy::new(sym("Cold"), sym("Hot")).0;

    // println!("{:?}", a1);
    println!("{:?}", pair);

    // This compares the analogy to a SymbolPair
    let a3 = a1.intersect(pair);
    println!("{:?}", a3);
    // Hot' Hot' Cold. Cold.
}
