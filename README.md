# Mindbase

A system for convergent intersubjectivity – To store knowledge, and make sense of the world.

[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#work-in-progress)

## Background

In most existing knowledge graph databases, data is stored as triples or quads:
_Subject, Predicate, Object, {Context}_

Given the example: _Alice jumped into the lake._
such a system would represent this as something like:
`Alice`, `jumped into`, `the lake`, `{bag_of_arbitrary_kv_pairs}`

Depending on what you're trying to do with this information, there can be some serious problems with this.

For starters, we have to agree on an ontology for each of the terms. We also have to record all of the other contextual information about this statement in a schemaless format which is generally just a bag of kv pairs. These include the recording user, the time the data was recorded, and scoping information about the statement (which lake? who told you this? etc etc)

## Ontologies – Betcha can't eat just one

Contraversial statement:
**Any Ontology which is fixed is automatically wrong.**

Another way to say this is that any ontology which is declared by parties other than those who are using it is rife with misalignments. A kind of Ontology-related form of moral hazard.

Ontologies morph and evolve through cultural processes. This is necessary, because our reality is constantly expanding. Science explains and discovers new phenomena. Cultural memes and tropes shift. New consumer products are introduced. It's not just our _stuff_ that's changing, it's also our ontological system.

So what sort of a narcissistic wingnut would dare plant their flag, declaring that _this_ ontology is correct?

Just imagine a hardcore prescriptivist attempting to bonk you on the head with a first edition, 1884 Oxford English dictionary. The english-speaking world speaks a very different language today versus 1884. Sure, the OED has been updated since then, but it'll never be able to keep up with the Urban Dictionary for expressions like "on fleek", "bromance",  or "the rona".

As is the case with OED and the Urban Dictionary, most people use multiple ontological systems. That is to say that their Meta-ontological system spans these sources.

So how on earth could so many people get ontologies _so_ wrong? How can it be that Bioinformaticists, data scientists, computer scientists, machine learning specialists, military, governments, and industries could have created such a gnarly cornucopia of competing ontologies?

Two reasons:

1. Because most humans and organizations make the huge mistake of believing they are the global oracle of objectivity; or at best that their local scope of objectivity is sufficient for their purposes.
2. Because we don't have a good system of merging multiple, coevolutionary ontologies.

We can't fix #1 directly, but we _can_ make inroads on #2. Mind you, this isn't simply a matter of rooting ontologies in a common "root" ontology like [BFO](https://basic-formal-ontology.org/). Root ontologies are fine, but the main problem isn't that we disagree about the nature of continuants vs occurrents. The problem is that the substrate we use to define ontologies is non-interactive. We don't have a good computerized system that mirrors the collaborative, coevolutionary processes that occur in the real world.

We define new terms all the time. Both those with fleeting meaning, and those which we intend to perpetuate must be connected to our ontological system for both publisher and consumer of each datum. How do we achieve this?

## A world of Analogies

Douglas Hofstadter famously explained that Analogy is the core of Cognition. Simply put: Analogy is necessary to build connection between any two ideas. There can be no thought, or language, or understanding without it. So what does this have to do with Ontologies? In much the same way that translating between two languages requires analogy, so do ontologies.  

Some translations are easier than others:
`Mi Casa`, `My House`, `Chez Moi` are all fairly cleanly _analagous_

`Hygge` (a Danish word) is kind of like `Coziness` but it's also a `Lifestyle`, and `Insulated from risk`, and `Fun` all rolled into one.

Similarly, `Saudade` and `Tiáo 条` do not have any direct english translation.  

So which language is "correct"? Obviously this is a ridiculous question, but it's exactly the sort of silliness that stumps our researchers who are trying to exchange data. There are two key forces in play here.

1. The fragmentary nature of our storage and collaboration systems is reflected in our Ontologies. (See [Conway's Law](https://en.wikipedia.org/wiki/Conway%27s_law))
2. Even if we had frictionlessly collaborative substrates, our data formats are too fragile. They naively strive for objectivity, but a vastly more potent target would be **convergent intersubjectivity.**  

This is the goal of Mindbase – To serve as a powerful substrate for convergent intersubjectivity. The combination of fragmentary storage and collaboration systems and the naivete of objectivity serves as a critical barrier which we hope to surmount. With it, we may strive to build better personal and professional informatics systems. We may reduce the barriers between open and industrial datasets. We may parse and correlate academic papers. We may even make inroads into explainable AI, and AGI. Of course these are ambitious goals, for any system, but we are at least confident that they cannot be achieved with the old techniques.

## The "Concept Problem"

TODO: Explain Prof Barry Smith's qualms with Concepts, and discuss why we are/aren't subject to them due to the Artifact / Allegation dichotomy.

As in the case of Tree / Apple, an "Apple" is not a "Tree", but rather a "Fruit" which is related to the "Tree". The key to making this work is that "Tree" is not one thing. Sure, there exists exactly one Artifact for the text string "Tree", but there are many many many possible Allegations which refer to that artifact in different capacities.  


## Graph representation

Essentially what we're dealing with here is a meta-graph in which the nodes and edges (Symbols) within the meta-graph are each comprised of a set of nodes and edges (Members) within the lower level graph. Those Symbols are constructed as a set of constitutent allegations (Members) documenting ideas of similarity/relatedness by Agents within the system. This allows the expression of relationships between imprecise logical entities (or rather precise-but-nonconverged logical entities) which describe entities within the real world. It is strictly intentional that this *not* be achieved by deferring to canonical or "objective" representations of symbols within the real world. This is because the real world fundamentally lacks such objectivity, and any such logical representation would thus create a fundamental impedence mismatch with the system which is curable only through out-of-band charismatic initiatives, and significant effort (IE convincing the whole world that your ontology is the "right" one).  
