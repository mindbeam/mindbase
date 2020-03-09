use mindbase::*;

#[test]
fn dialog_1() -> Result<(), std::io::Error> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    let alice = mb.default_agent()?;
    let bob = mb.create_agent()?;

    let a_said = mb.ground_symbol(&alice,
                                  vec![FlatText::new("Things that I said"),
                                       FlatText::new("Walking down the sidewalk")])?;
    let b_said = mb.ground_symbol(&alice,
                                  vec![FlatText::new("My thoughts"), FlatText::new("When I was on my way to lunch")])?;

    mb.alledge2(&alice, FlatText::new("I like turtles"))?;

    let statement = mb.alledge(FlatText::new("I like turtles"))?;
    let category = mb.alledge(FlatText::new("Things that I said"))?;
    let analogy = mb.alledge(Analogy::declare(statement.narrow(), category.narrow()))?;

    let general = mb.put_artifact(FlatText::new("In general"))?;
    let things = mb.put_artifact(FlatText::new("Things that I said"))?;

    // I want to conjure/scrounge/locate/triangulate/intersect a Concept based on:
    // My AgentId + ArtifactId
    // And what else?
    // There needs to be something that this is rooted.

    // What situations might have precipitated that would lead me to conjuring a non-narrow concept?

    // A: Hey, do you want to take a [trip1] with me? -- Narrow concept conjured from new allegation I just made
    // B: Sure, I'll take a [trip2,trip1] with you    -- Create a new allegation to represent interpreted meaning (same artifact)
    // A: What's a good day for our [trip1,trip2]?    -- "trip" Concept broadens to 2 allegations
    // B: How about tuesday? A: Great, I'll get the psilocybin
    // B: Whoa, I thought you meant a [trip2] not a [trip1] -- {negative analogy between [trip1] and [trip2]}

    // Things that I said
    // Where "I" is my agent ( Agent is an allegation too? )
    // (bit of a bootstrapping dilemma here)
    Ok(())
}
