# Glossary

## Agent

A user, or other automated process which is capable of making allegations. Each agent has a cryptographic identity which is used to sign Allegations.  

## Artifact

Some blob of data which may be meaningful to the outside world, likely containing `External Meaning`, but is otherwise devoid of `Internal meaning`, at least by its existence.

## AllegationID

Renamed to [ClaimID](./GLOSSARY.md#ClaimId)  

## Allegation

Renamed to [Claim](./GLOSSARY.md#Claim)

## ClaimID

The unique identity of some specific [Claim](./GLOSSARY.md#Claim)

## Claim

An event or declaration which is being documented by an agent But it's also more than that – It's an instance of some payload. At present, such payload might be any of the following types:

* ArtifactId - Content-addressable ID of a given `Artifact`.  
* Analogy - Associate one `Symbol` with another `Symbol` (Eg: There exists some Symbol C that represents the connection of Symbol A and Symbol B)
* AgentId - PubKey of an Agent
* Unit - Empty payload.  

Though an Allegation may reference one or more `Artifacts`, implying that the allegation is somehow an _instance_ of that `Artifact` it doesn't even need to be an instance of something per se. It could be a `Unit` Allegation, which is just an anonymous enumeration.  
Each Allegation has a Payload.

## Concordance

Noun. A declarative or final state in which elements or subgroups of a given cohort have become "sufficiently" aligned by whatever standard.

We may wish to use the dictionary-synonymous term "Converged" (Noun) to refer to this final state. However, given that it may be readily conflated with the continuous, aggregate process of [Convergence](./GLOSSARY.md#convergence) which is a crucial differentiation for our purposes here.

## Convergence

##### As in

Convergence, Converging, Converge.

##### Definition

1. Verb – Referring to a continuous process in which an open-ended subset of a given cohort become increasingly aligned in some fashion, while not necessarily concluding in their alignment.
2. Noun – Referring to the process of Converging itself, and crucially: NOT a declarative or final state of "sufficiency". See [Concordance](./GLOSSARY.md#concordance)

**Example**
One example of convergence is a river going over a waterfall, insofar as all the river's water is headed to the same place. Yet it never concludes, and at least for as long as the river flows, it never reaches a final state of alignment, where all the water has arrived in the same place.  

We can also imagine other abstract scenarios in which every point in space is the origin of some river, and every other point of space is within its river-delta, such that the whole of space is continuously converging, but never fully concordant.

For purposes of clarity, we recommend against using the term "Converged" (Noun) to describe a process which has concluded to sufficiency by some standard, and instead use: Concorded, [Concordance](./GLOSSARY.md#concordance), Concluded, Completed, or Aligned.

**Note**
Convergence may in fact occur in discrete chunks, by way of many incremental or momentary Concordances. However, in this nomenclature at least, we must take great care to differentiate such finalities, however momentary, from the overarchingly-continuous dynamic processes of a system with continuous inputs.

## FuzzySet

TODO

## Symbolic Grounding

TODO

## MBQL

Mindbase Query Language

```mbql
    $x = Bind("Hot")
    $y = Ground($x : "Cold")
```

## Meaning

Meaning of any kind is necessarily an emergent property, or [gestalt](https://en.wikipedia.org/wiki/Gestalt_psychology) of a larger system. Because of which, you can never quite put your finger on "meaning" – only definitions. If you look in the Oxford English Dictionary, you will see declarative definitions curated by the editors which approximate meaning in the broader world. They are necessarily imperfect, not only because they are described by some authority who may be wrong, but also because the world is an ever-changing place. There exists exquisite nuance which is being manufactured constantly, and which is at any point-in-time many orders of magnitude more complex than a single book could ever hope to document.  

The Mindbase project intends to contemplate, and perhaps in some limited fashion to bridge, two different types of "Meaning":

#### External Meaning

Meaning which emerges from cultural and social processes within the world. This could refer to pure analog/meatspace type processes, and could even include other computer-based systems which are _not MindBase._

You employ _External Meaning_ every time you speak, listen, read, think, hear music, dance, or reach for the handle of that coffee cup. You learn the _meaning_ of road signs, body language, and that smokey smell coming from the kitchen. You do this via the staggeringly complex system of symbol exchange and manipulation which we refer to as "Culture" or "Civilization". Culture, and the participants thereof, perform this process via the confluence of a huge number of tiny stimuli. The first _meaningful_ words you spoke as a baby were likely in response to an agglomeration of such stimuli previously received. Your first words [Symbolized](./GLOSSARY.md#Symbol) those previous stimuli, plus perhaps some modicum of intention (whatever that means) on your part to use them.

#### Internal Meaning

Meaning which emerges from within Mindbase itself as the sum of its constituent data.  

Of course there exists a yawning gulf in terms of scale, and a profound impedance mismatch between the exquisite processes of [External Meaning](./GLOSSARY.md#external-meaning), and the puny machinations of any kind of semantic database. For example, there are infinite senses and implications of the word "is", because symbols like "is", and the sound that a person makes when pronouncing or hearing it are each _individual occurrences_, each with potentially different contexts, connotations, implications, and interpretations.  

And so, prescriptivists should rightly tremble in fear from the implications of trying to describe and document each discrete sense of each word and symbol which might be used.

With a problem so daunting, how on earth or the heavens can we ever hope to put even a tiny dent in capturing this universe of meaning?

The goal of Mindbase is to representing meaning in a way that is BOTH programmatically useful, and able to interface with the nuance of the world _internally_, such that meaningful computation can be performed inside the Mindbase system. Until such time as Mindbase expands to fill the entire observable universe (har har), there will necessarily be some logical boundary between its internal expression of meaning, and that of the outside world. We cannot blindly subordinate the one to the other. That would put us right back where we started. We must build a bridge between these systems of internal and external meaning. This requires that such a system be cognizant that a lot of meaning is defined by human culture at large, or other external systems like the semantics of a `docx` file. Because of this necessity to represent `External Meaning` we have `Artifacts` and `Allegations`

## Ontology

#### Common Ontology

Any ontological system which is either non-formal, or trivially formal.
Such systems are predominantly managed via interpersonal processes such as oral tradition and other social meme propagation dynamics.  
These systems define the symbols and concepts which you, as a functioning member of the human collective, use every day to communicate with the world at large.

For the purposes of our definition here, the English language itself could be a considered an informal, or trivially formal ontology, and would thus be considered "Common". Although we have elaborate dictionaries and thesauruses, they can't possibly stand on their own. They are necessarily written in such a way that requires substantial contextual information about society which we as children must first learn. [External Meaning](./GLOSSARY.md#External-Meaning) is generally based on such Common Ontologies, thus requiring an agent containing a significant fraction of their massive breadth in order to competently interpret or generate.

#### Formal Ontology

For our purposes here, we consider an Ontological System to be "Formal" when its creation is the result of a deliberative process which seeks to minimize or eliminate self-inconsistency. All such systems are necessarily imperfect, because meaning itself ([External](./GLOSSARY.md#External-Meaning) or otherwise) emerges from interpretation of information. Such interpretation necessarily requires the bridging of domains, in which the symbols of one ontological system must be correlated, or [Grounded](./GLOSSARY.md#Symbolic-Grounding) to those of another. Any system which lacks this capacity to correlate elements within two epistemic domains (however trivial they might be) is necessarily devoid of meaning.

The inexorable misalignment between these domains (the Formal Ontology in question, and the foreign domain which it intends to describe) necessarily results in a sort of impedance mismatch. No matter how exquisite a given Formal Ontology might be, the opportunity for it to be subtly misaligned with some novel or reinterpreted facet of the other domain will always exist.

A Formal ontology could be said to be perpetually "chasing" said foreign domain, rather like a person chasing a rainbow – Always _just- out of reach.

## Symbol

A Symbol is a set of [Atoms](./GLOSSARY.md#Atom) – The agglomeration of which represents some abstract concept. Each of these Atoms represents some [Allegation](./GLOSSARY.md#Allegation) which documents the instance of something according to a given agent. Individually, each of these Atoms doesn't [mean](./GLOSSARY.md#internal-meaning) much. Collectively, they serve as tent-poles, expanding or limiting the degrees of freedom within some abstract semantic space.  

## Truth

#### Intersubjective Truth

The notion of _Intersubjective Truth_ describes symbols, definitions, or meanings which are agreed upon by some cohort as being factual or incontrovertible. A given set of intersubjective truths may be divergent within subgroups, such that proximate cohorts disagree, or [Convergent](./GLOSSARY.md#Convergence), such that proximate cohorts agree. Note that we are not talking about the entirety of the system, which may be beyond the horizon of comparability.

Intersubjective truth is essentially the only type of truth which we know, and can access. See [philosophical discussion](./PHILOSOPHY.md) for more detail.

As opposed to [Declarative Truth](./GLOSSARY.md#declarative-truth)

#### Declarative Truth

The assertion by some authority that a symbol, definition, or meaning is the official or canonical one.
As opposed to [Intersubjective Truth](./GLOSSARY.md#intersubjective-truth)  
