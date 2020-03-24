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

Another way to say this is that any ontology which is declared by parties other than those who are using it is rife with misalignments. A kind of Ontology-realted form of moral hazard.

Ontologies morph and evolve through cultural processes. This is necessary, because our reality is constantly expanding. Science explains and discovers new phenomena. Cultural memes and tropes shift. New consumer products are introduced. It's not just our _stuff_ that's changing, it's also our ontological system.

So what sort of a narcicistic wingnut would dare plant their flag, declaring that _this_ ontology is correct?

Just imagine a hardcore prescriptivist attempting to bonk you on the head with a first edition, 1884 Oxford English dictionary. The english-speaking world speaks a very different language today versus 1884. Sure, the OED has been updated since then, but it'll never be able to keep up with the Urban Dictionary for expressions like "on fleek", "bromance",  or "the rona".

As is the case with OED and the Urban Dictionary, most people use multiple ontological systems. That is to say that their Meta-ontological system spans these sources.

So how on earth could so many people get ontologies _so_ wrong? How can it be that Bioinformaticists, data scientists, computer scientists, machine learning specialists, military, governments, and industries could have created such a gnarly cornucopia of competing ontologies?

Two reasons:

1. Because most humans and organizations make the huge mistake of believing they are the global oracle of objectivity; or at best that their local scope of objectivity is sufficient for their purposes.
2. Because we don't have a good system of merging multiple, coevolutionary ontologies.

We can't fix #1 directly, but we _can_ make inroads on #2. Mind you, this isn't simply a matter of rooting ontologies in a common "root" ontology like [BFO](https://basic-formal-ontology.org/). Root ontologies are fine, but the main problem isn't that we disagree about the nature of continuants vs occurrents. The problem is that the substrate we use to define ontologies is non-interactive. We don't have a good computerized system that mirrors the collaborative, coevolutionary processes that occur in the real world.

We define new terms all the time. Both those with fleeting meaning, and those which we intend to perpetuate must be connected to our ontological system for both publisher and consumer of each datum. How do we achieve this?

## A world of Analogies

Douglas Hofstadter famously explained that Analogy is the core of Cognition. Simply puy: Analogy is necessary to build connection between any two ideas. There can be no thought, or language, or understanding without it. So what does this have to do with Ontologes? In much the same way that translating between two languages requires analogy, so do ontologies.  

Some translations are easier than others:
`Mi Casa`, `My House`, `Chez Moi` are all fairly cleanly _analagous_

`Hygge` (a Danish word) is kind of like `Coziness` but it's also a `Lifestyle`, and `Insulated from risk`, and `Fun` all rolled into one.

Similarly, `Saudade` and `Tiáo 条` do not have any direct english translation.  

So which language is "correct"? Obviously this is a ridiculous question, but it's exactly the sort of silliness that stumps our researchers who are trying to exchange data. There are two key forces in play here.

1. The fragmentary nature of our storage and collaboration systems is reflected in our Ontologies. (See [Conway's Law](https://en.wikipedia.org/wiki/Conway%27s_law))
2. Even if we had frictionlessly collaborative substrates, our data formats are too fragile. They naively strive for objectivity, but a vastly more potent target would be **convergent intersubjectivity.**  

This is the goal of Mindbase – To serve as a powerful substrate for convergent intersubjectivity. The combination of fragmentary storage and collaboration systems and the naivete of objectivity serves as a critical barrier which we hope to surmount. With it, we may strive to build better personal and professional informatics systems. We may reduce the barriers between open and industrial datasets. We may parse and correlate academic papers. We may even make inroads into explainable AI, and AGI. Of course these are ambitious goals, for any system, but we are at least confident that they cannot be achived with the old techniques.

## Glossary

**Agent** -  A user, or other automated process which is capable of making allegations. Each agent has a cryptographic identity which is used to sign Allegations.  
**Artifact** - Some blob of data which may be meaningful to the outside world, likely containing `External Meaning`, but is otherwise devoid of `Internal meaning`, at least by its existence.
**Allegation** - An event or declaration which is being documented by an agent But it's also more than that – It's an instance of some payload. At present, such payload might be any of the following types:

* ArtifactId - Content-addressable ID of a given `Artifact`.  
* Analogy - Associate one `Concept` with another `Concept` (Eg: Concept A is in the category of Concept B)
* AgentId - PubKey of an Agent
* Unit - Empty payload.  

Though an Allegation may reference one or more `Artifacts`, implying that the allegation is somehow an _instance_ of that `Artifact` it doesn't even need to be an instance of something per se. It could be a `Unit` Allegation, which is just an anonymous enumeration.  
Each Allegation has a Payload.

**Concept** - A structure representing a region of semantic space at the intersection of a given set of Allegations. Such a concept may contain a small number of Allegations by a single, or small number of Agents (in which case it is said to be a `Subjective Concept`) or it may contain a more diverse array of Allegations inclusive of those of other Agents (in which case it is said to be a an `Intersubjective Concept). There is no hard delineation between Subjective and Intersubjective Concepts. It is merely a way of comparing/describing concepts which may be more or less well agreed upon.

**Internal/External Meaning** - Mindbase is all about representing meaning, and striving to do so _internally_, such that meaningful computation can be performed (emergently) inside the Mindbase system. Until such time as Mindbase expands to fill the entire observable universe (har har), there will necessarily be some logical boundary between its internal xpression of meaning, and that of the outside world. We cannot blindly subordinate the one to the other. That would put us right back where we started. We must build a bridge between these systems of internal and external meaning. This requires that such a system be cognizant that a lot of meaning is defined by human culture at large, or other external systems like the semantics of a docx file. Because of this necessity to represent `External Meaning` we have `Artifacts` and `Allegations`

**Symbol** - General term which may refer to an `Alligation` / `Subjective Concept` or `Intersubjective Concept` – each of which is an epistemic device intent on recording some occurrence, or meaning. The notion that a meaning _is_ an occurrence, or a cluster of occurrences (which is essentially memoized into a new occurrence) is a key idea in Minbase.  


## The "Concept Problem"

TODO: Explain Prof Barry Smith's qualms with Concepts, and discuss why we are/aren't subject to them due to the Artifact / Allegation dichotomy.

As in the case of Tree / Apple, an "Apple" is not a "Tree", but rather a "Fruit" which is related to the "Tree". The key to making this work is that "Tree" is not one thing. Sure, there exists exactly one Artifact for the text string "Tree", but there are many many many possible Allegations which refer to that artifact in different capacities.