use mindbase::prelude::*;

#[test]
fn alice() -> Result<(), MBError> {
    // The question is: how do we represent predicates? Things like "was just like", and "in that we were", etc
    //
    // Depending on how you want to look at it, you could say that:
    // * Mindbase has exactly one predicate "is a member of", which is simply implicit in every Analogy
    // or
    // * Mindbase has an infinite number of predicates which you can define – Buttt, they're fused to the object
    //
    // So, we get "is in the category of" for free with each analogy.
    // For a statement like "The pan is hot" we would think of this as:
    // [the pan] (is in the category of) [things that are hot]
    // Connecting words like "things that are" can generally be discarded, provided they are referring to the subject.
    // If the connecting words _do_ in fact change the meaning, then either the subject or the object should be recursively
    // expanded to reflect that meaning.
    //
    // # Why not subject-predicate-object triples?
    // * Because they converge poorly - (speculation)
    // * Because it externalizes the semantics of the predicate to the user
    // * Because the event of jumping into the lake is itself a discrete constituent of the
    // [Alice [jumped into] [the lake]]
    //
    // * What is [jumped-into]ing, and how does it correlate to jumping?
    // * How do we determine which type of jumping it's related to?
    //
    // [Alice [[jumped [into the lake]]]
    //
    // [the lake]
    // [jumped]
    // [Artifact(Alice)] catOf [Alice]
    // [Artifact(JPGofAlice)] catOf []
    // [llkv9iowdehasdfasdfusdf, sdflksdfluiweedfsdf] catOf Agent(aliceuuid)
    // [Alice] catOf [that girl at the club on tuesday]
    // [Alice] catOf [Joe's friend]
    // [Alice] catOf [Things that are definable with this picture]
    //
    //  [Artifact("Alice")][789] --\          (Subjective) Symbol {789,123}
    //                              |
    //                          Unit[123] ---> [Joe's Friend]
    //                              |
    //      [Artifact(AliceJPG)] ->/
    //
    //
    //    Artifact("Alice")[456]   (Subjective) Symbol {456}
    //
    //
    //   <->
    //
    //  D:Allege (456 catof 123)  -> (Intersubjective) Symbol{123,456,789} I talk about this ALice which is a little bigger
    //       than your alice symbol
    //  R:Allege (123 catof 456)  -> (Intersubjective) Symbol{456,123}   You talk about this Alice, which is very closely
    //       aligned with mine
    //
    // TODO 2 - symbol surrogates
    //
    //    **Critically* My previous statements about Alice{123,789} can be compared with your statements about Alice {456}
    //    and vice versa
    //
    //  Question: When Daniel gets a ground symbol (Symbol) about Alice *after* Daniel and Rob have exchanged alices, is that
    //  expansive of both, or is it the responsibility of the projection to fill this in.
    //  IS ROOT SYMBOL CONJURING INCLUSIVE OF OTHER SEMI-TRUSTED AGENTS? (I think no. Only of ground agents)
    //  if a disjunction is later found that invalidates some dimension(s) of the symbol that was used for OTHER allegations
    //
    //
    //
    //                   Unit[some event] -> [into the lake]
    //
    //  [into the lake]  -> [the lake]
    //        \----> [cases where jumping into something occurred]
    //        \----> [things that were done into something]
    //        \----> [things a person did]
    //        \----> [Things alice did]
    //                       \----> [things that are about Alice] <- Artifact("Alice")
    //                       \---->
    //        \----> [things that happened last tuesday]
    //
    //
    // [Jumping] catOf [things Alice did] [in the lake]
    // [in the lake] catOf [things relating to lakes]
    // [things alice did] catOf []
    //
    // ****** TODO 2 ******
    // Follow up on the notion that a knowledge triple~~dependency tree, whereas a category ~~ a constituency tree
    // It feels like there may be something to this
    //
    // TODO 2 - clarify in the code that:
    //  * An allegation/Symbol is a category
    //  * That category be automatically expanded based on Analogies defined against it
    //  Q: how do we make it clear to the user that such Analogies are being traversed?
    //  A: we probably don't - if it's done lazily
    //  Q: how many hops do we do for vicarious analogies? [A] <- B <- C = [A,B,C]
    //  A: I think we have to do this lazily, rather than actually materializing this
    //  Q: When and how is that lazy-evaluation performed?

    // Alice said I like turtles
    //
    // If we represent this in a subject,predicate,object notation we get:
    // (Alice, said, (I, like, turtles))
    //
    // If we use an analogical representation:
    // There exists a specific instance of "like"ing - like1
    // There exists a specific instance of "turtles" - turt1
    // turt1 is in the category of like1
    // There exists a specific instance of "I" - self1
    // that
    // turtles are in a specific instance of "like"-itude
    // I is that instance of likitude is
    // Alice is in the category of (
    //            said is in the category of (
    //                 I is in the category of (
    //                                         )
    //                )
    // )
    // (Alice (said (I (like (turtles)))))
    Ok(())
}

#[test]
fn apple() -> Result<(), MBError> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    let apple_computers = mb.alledge(Text::new("Apple"))?;
    let apple_the_fruit = mb.alledge(Text::new("Apple"))?;
    let apple_of_my_eye = mb.alledge(Text::new("Apple"))?;

    // Look up the "ground symbol" for "Apple" without any additional specificity
    let query = mb.query_str(r#"$a = Ground("Apple")"#)?;
    query.apply()?;

    let apple_ground_symbol = query.get_symbol_var("a")?.unwrap();

    // It's... all of them. Why? Because meaning is contextual/intersectional.
    // We don't have enough information to narrow it down yet and we should not assume what they meant
    assert_eq!(apple_ground_symbol.count(), 3);

    let _statement = mb.alledge(Text::new("I love Apple"))?;

    // TODO 2 - surrogate Symbols
    // let apple_for_the_purposes_of_this_conversation = apple.surrogate();

    //     // // Lets be a liittle more specific. (Using get_ground_symbol here as a shortcut)
    mb.alledge(Analogy::declarative(apple_computers.subjective(), mb.alledge("Corporation")?.subjective()))?;
    mb.alledge(Analogy::declarative(apple_the_fruit.subjective(), mb.alledge("Edible Fruit")?.subjective()))?;
    mb.alledge(Analogy::declarative(apple_of_my_eye.subjective(), mb.alledge("Amorousness")?.subjective()))?;

    let query = mb.query_str(r#"$a = Ground("Corporation" : "Apple")"#)?;
    query.apply()?;
    let apple = query.get_symbol_var("a")?.unwrap();
    assert_eq!(apple.count(), 1);

    Ok(())
}

#[test]
fn apple_ii() -> Result<(), MBError> {
    //     // Lets suppose that Alice makes a statement about apples. Lets record that having happened.
    //     let alice_statement = mb.alledge(text("I love apples"))?;

    //     // Now, lets also use NLP to parse this statement:
    //     //  NP[I]  VP[love apples]
    //     // PRP[I] VBP[love] NP [apples]
    //     //
    //     // Note: these derrived Artifacts are related to the original artifact of alice's statement.
    //     // TODO 2 - How should the system alledge that these are related, and that it wasn't actually alice who broke them
    // down     // this way?
    //     let _np_i = mb.alledge(text("I"))?;
    //     let _vp_love_apples = mb.alledge(text("love apples"))?;
    //     let prp_i = mb.alledge(text("I"))?;

    //     // vbp = Verb non-3rd person singular present form
    //     let vbp_love = mb.alledge(text("love"))?;
    //     // np = Proper Noun
    //     let np_apples = mb.alledge(text("apples"))?;

    //     // the symbol we define for np_apples is in the category of vbp_love
    //     let apple_love = mb.alledge(Analogy::declarative(np_apples.subjective(), vbp_love.subjective()))?;

    //     // The symbol for Alice's self alledged to be in the category of apple_love
    //     let alice_loves_apples = mb.alledge(Analogy::declarative(prp_i.subjective(), apple_love.subjective()));

    //     // ok, great

    //     // Lets make some apples. These all share the same artifact, but they're different allegations.
    //     // Lets imagine that these are part of an initial set of allegations which is provided by some agent
    //     // early in the growth of the system, in order to prime the pump. Other agents may make redundant and/or similar
    //     // allegations, either because they didn't see these, or didn't understand them, or didn't have the time to
    // correlate     // them.
    //
    //     // let apple_plural = mb.alledge(text("Plural form of Apple"))?;
    //     // mb.alledge(Analogy::declarative(apples.subjective(), things_i_love.subjective()))?;

    //     // // Lets start out simple. Apple. Which apple are you talking about?
    //     // let fruit = mb.get_ground_symbol(vec![text("Apple")])?;

    //     // // Just for fun, Lets get reeal specific with the biological taxonomy. Note that it's conceivable that this
    // exact     // taxonomy // could also be present which might mean something completely different! While the
    // length of our     // specified // taxonomy makes this a bit less likely, remember that there is nothing magical
    // about these     // artifacts.
    // let malus_domestica1 = mb.get_ground_symbol(vec![text("Domain: Eukarya"),
    //                                                  text("Kingdom: Plantae"),
    //                                                  text("Phylum: Magnoliophyta"),
    //                                                  text("Class: Magnoliopsida"),
    //                                                  text("Order: Rosales"),
    //                                                  text("Family: Rosaceae"),
    //                                                  text("Genus: Malus"),
    //                                                  text("Species: Malus domestica"),])?;

    //     // let tree = mb.get_ground_symbol(vec![text("Plant"), text("Tree")])?;
    //     // let fruit = mb.get_ground_symbol(vec![text("Fruit")])?;

    //     // //  text("with an elongated stem or trunk"),
    //     // //  text("has branches and leaves"),
    //     // // mb.alledge(Analogy::declare(malus_domestica1.clone(), tree.clone()))?;
    //     // // text("seed-bearing structure"),
    //     // //                                       text("of a flowering plant"),
    //     // //                                       text("formed from the ovary after flowering")

    //     // // text("Apple");
    //     // // text("Fruit of the");;

    // let malus_domestica2 = mb.get_ground_symbol(vec![text("Domain: Eukarya"),
    //                                                  text("Kingdom: Plantae"),
    //                                                  text("Phylum: Magnoliophyta"),
    //                                                  text("Class: Magnoliopsida"),
    //                                                  text("Order: Rosales"),
    //                                                  text("Family: Rosaceae"),
    //                                                  text("Genus: Malus"),
    //                                                  text("Species: Malus domestica"),])?;

    //     // // text("Apple");
    //     // // text("Fruit of the");

    // assert_eq!(malus_domestica1, malus_domestica2);
    Ok(())
}
#[test]
fn fridays() -> Result<(), MBError> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    // Next Friday
    let f1 = mb.alledge("Friday")?.subjective();

    // The abstract symbol of Friday
    let f2 = mb.alledge("Friday")?.subjective();

    // The person named Friday
    let f3 = mb.alledge("Friday")?.subjective();

    let fut = mb.alledge("Days which are in the near future")?.subjective();
    let dow = mb.alledge("Abstract day of the week")?.subjective();
    let per = mb.alledge("Names for a person")?.subjective();

    mb.alledge(Analogy::declarative(f1, fut))?;
    mb.alledge(Analogy::declarative(f2, dow))?;
    mb.alledge(Analogy::declarative(f3, per))?;

    unimplemented!();
    // let _friday_person = mb.get_ground_symbol(vec!["Friday", "Names for a person"])?;
    // let names = mb.get_ground_symbols_for_artifact(FlatText::new("Names for a person"))?
    //               .expect("Option");

    // let fridays = fridays.narrow_by(mb, names);
    // println!("{:?}", fridays);
    Ok(())
}
