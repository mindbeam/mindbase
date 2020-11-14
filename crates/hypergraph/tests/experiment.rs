use mindbase_hypergraph::{query::AnalogyQuery, AssociativeAnalogy, testing::SimpleEntity};


fn experiment1() {
    // In this experiment, we are approximating the following MBQL
    // $x = Bind("Hot")
    // $y = Ground($x : "Cold")

    let mut x = Symbol::null();
    let mut y = FuzzySet::new();

    // For simplicity, lets say these are all the analogies in the system
    let candidates = [
        //
        AssociativeAnalogy<SimpleEntity>::new(sym!["Hot1", "Hot2", "Heated1"], sym!["Mild1", "Mild2", "Cold3"]),
        AssociativeAnalogy<SimpleEntity>::new(sym!["Hot3"], sym!["Cold1", "Cold2"]),
        AssociativeAnalogy<SimpleEntity>::new(sym!["Cold3"], sym!["Hot3"]),
    ];

    // Imagine we looked up all AtomIds for all Allegations related to Artifacts "Hot" and "Cold"
    let query = AnalogyQuery::new((sym!["Hot1", "Hot2", "Hot3"], sym!["Cold1", "Cold2", "Cold3"]));
    println!("Query is: {}", query);

    for candidate in &candidates {
        let v: FuzzySet<analogy::AnalogyMember> = candidate.interrogate(&query).expect("All of the above should match");
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

// Prompt from #7:
// One thing within the experimental code which is almost certainly wrong is the way unions are being performed across the output
// of each candidate Analogy interrogation. We must explore a more appropriate means of composing these candidate Analogy
// interrogation outputs in a weighted fashion, rather than simply taking the maximum degree of each discrete matching member into
// the final output FuzzySet. This is screwy, because we likely don't want Members from a small subset of candidate Analogies with
// a high degree of matching to compete on equal footing with a corpus of thousands with a low matching degree, as a simple
// maximum-degree of membership union might provide. (current code does this) However, we also don't want to attenuate the signal
// of such a well-matching subset of candidate Analogies as a simple weighted score would suggest either. Presumably there is some
// middle ground which must be found, whereby these considerations are balanced. Not a simple weighted score, and not a
// maximum-degree of FuzzySet membership either.

// For the time being, I will call this the Fuzzyset-union signal-to-noise ratio problem.
fn fuzzy_set_union_signal_to_noise_problem() {
    // Question 1
    // How many fuzzysets do we need to construct for this to be an issue?
    // It seems like two may not be illustrative of the problem in question

    // lets imagine we interrogate three candidate Analogies and we are left with the following Symbols
    // which we are constructing manually here, but would be typically be analogy interrogation outputs created by the query tree.
    let io1 = sym!["Hot1"];
    let io2 = sym![("Hot1", 0.5), ("Muggy1", 0.9)];
    let io3 = sym![("Hot1", 1.0), ("Sticky1", 0.5)];
    println!("{:?}", io3);

    // union the interrogation outputs together
    let mut u = Symbol::null();
    u.union(io1);
    u.union(io2);
    u.union(io3);

    println!("{:?}", u);
    // What do we want to have in the end, and why?

    // Should include the max of each degree?
    // u is [Hot1~1,Muggy1~0.9,Sticky~1]
    // This doesn't seem very good. We want small signals to be boosted, but this might be a bit too much

    // Hot 1 is present in all input Symbols. Should we average them?
    // u is [Hot1~0.6,..]

    // What about Muggy1, and Sticky1 - which are only present in some of the inputs?
    // Should we treat the sets which lack them as degree 0, and include those in the average?
    // u is [Hot1~0.6, Muggy1~0.3, Sticky1~0.4]

    // Or should we average them individually based on their non-null set membership?
    // u is [Hot1~0.6, Muggy1~0.9, Sticky1~0.6]

    // Let's take a step back. What do each of these input symbols represent?
    // Each symbol represents one side of an analogy which a trusted (ground) Agent previously Claimed.
    // Each member of which had its degree determined by some prior query, presumably by that Agent, wherein a partial match of
    // claims was had. This could come about a number of different ways, but the simplest construction of events is:

    // a1hot : Symbol = Agent1.query("Hot");

    // TODO - construct a full chain of events (including genesis Claims) by which Symbol members of a degree <1 are constructed,
    // and then Claimed as new Analogies From there we can determine the most prudent implementation of union, such that we
    // optimize the signal-to-noise ratio
}
