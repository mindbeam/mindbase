use mindbase::*;

#[test]
fn saturday() -> Result<(), std::io::Error> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let _mb = MindBase::open(&tmpdirpath)?;

    // Lets find a grounding symbol for "Saturday"
    // This needs to be a symbol which we regognize from our authorities.
    //   * Our own agent, plus some known set of other agents.

    // Could be this Saturday
    // Could be last Saturday
    // Could be the the abstract *idea* of Saturday
    // Could refer to one specific square on a paper calendar on your wall
    // Could refer to the column on a paper calendar
    // Could be a person's name...
    // let saturday = mb.ground_symbol(FlatText::new("Saturday"));

    Ok(())
}

#[test]
fn dialog_1() -> Result<(), std::io::Error> {
    let tmpdir = tempfile::tempdir()?;
    let tmpdirpath = tmpdir.path();
    let mb = MindBase::open(&tmpdirpath)?;

    let alice = mb.default_agent()?;
    let bob = mb.create_agent()?;

    // Alice and bob were going about their day, when they bumped into each other on the sidewalk.
    // They each have a perspective about what was happening which differ slightly, but they generally agree.
    // They exchange some pleasantries and then go about their day.
    // The goal is to explain, categorize, and correlate each of these things from their own perspectives

    // They haven't yet bumped into each other. What are they doing?
    let a_things_imdoing = mb.get_ground_concept(vec!["Things I'm doing", "Alice"])?;
    let b_things_imdoing = mb.get_ground_concept(vec!["Things I'm doing", "Bob"])?;

    // Alice is going to describe an event, and so we need a unique symbol for that. (each Allegation is a universally unique
    // Symbol) We are alledging/creating a symbol for this event against text artifact, but it could easily be an
    // anonymous "Unit" artifact. Either way, the symbol itself, and its association to this artifact is meaningless of its own
    // accord, except that it's a thing that's discrete from the rest of the universe, at least to start.
    let a_walking = mb.alledge2(&alice, "Walking down the street")?;
    mb.alledge2(&alice, Analogy::declarative(a_walking, a_things_imdoing.clone()))?;

    // Bob describes a different event. Again, it, and the artifact associated with it is meaningless of its own accord.
    let b_on_my_way = mb.alledge2(&bob, "On my way to get a haircut")?;
    mb.alledge2(&alice, Analogy::declarative(a_walking, b_things_imdoing))?;

    //         Alice is defining this (     )  <- (       ) <- (       )
    //   NLP agent is defining this    / | \       /  |  \      /  |  \
    //         Bob is defining this    \/          \___________/
    //
    // Both of these being events, do not require the location of any grounding symbols for their definitions
    // However, we do want to correlate these events to our broader semantic network, and so, we have to categorize/analogize them
    // to something. But what? _That_ is where we need to conjure some grounding symbols. They need not be meaningful to everyone,
    // but they do need to be something which have meaning to Alice and Bob individually.

    // In order to do this, Alice and Bob need to be able to *reproducibly* retrieve the same symbols using external value(s)

    let a_said = mb.get_ground_concept(&alice, vec!["Things that I said", "Walking down the sidewalk"])?;
    let b_said = mb.get_ground_concept(&alice, vec!["My thoughts", "When I was on my way to lunch"])?;

    // mb.alledge2(&alice, FlatText::new("I like turtles"))?;

    // let statement = mb.alledge(FlatText::new("I like turtles"))?;
    // let category = mb.alledge(FlatText::new("Things that I said"))?;
    // let analogy = mb.alledge(Analogy::declare(statement.narrow(), category.narrow()))?;

    // let general = mb.put_artifact(FlatText::new("In general"))?;
    // let things = mb.put_artifact(FlatText::new("Things that I said"))?;

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
