# Glossary

## Agent

A user, or other automated process which is capable of making allegations. Each agent has a cryptographic identity which is used to sign Allegations.  

## Artifact

Some blob of data which may be meaningful to the outside world, likely containing `External Meaning`, but is otherwise devoid of `Internal meaning`, at least by its existence.

## AllegationID

Renamed to [ClaimID](./GLOSSARY.md#ClaimId) 

## Allegation

Renamed to [Claim](./GLOSSARY.md#Claim)

## Ontology, Common

Any ontological system which is either non-formal, or trivially formal.
Such systems are predominantly managed via interpersonal processes such as oral tradition and other social meme propagation dynamics.  
These systems define the symbols and concepts which you, as a functioning member of the human collective, use every day to communicate with the world at large.

For the purposes of our definition here, the English language itself could be a considered an informal, or trivially formal ontology, and would thus be considered "Common". Although we have elaborate dictionaries and thesauruses, they can't possibly stand on their own. They are necessarily written in such a way that requires substantial contextual information about society which we as children must first learn. [External Meaning](./GLOSSARY.md#External-Meaning) is generally based on such Common Ontologies, thus requiring an agent containing a significant fraction of their massive breadth in order to competently interpret or generate.

## Ontology, Formal

For our purposes here, we consider an Ontological System to be "Formal" when its creation is the result of a deliberative process which seeks to minimize or eliminate self-inconsistency. All such systems are necessarily imperfect, because meaning itself ([External](./GLOSSARY.md#External-Meaning) or otherwise) emerges from interpretation of information. Such interpretation necessarily requires the bridging of domains, in which the symbols of one ontological system must be correlated, or [Grounded](./GLOSSARY.md#Symbolic-Grounding) to those of another. Any system which lacks this capacity to correlate elements within two epistemic domains (however trivial they might be) is necessarily devoid of meaning.

The inexorable misalignment between these domains (the Formal Ongology in question, and the foreign domain which it intends to describe) necessarily results in a sort of impedence mismatch. No matter how excuisite a given Formal Ongology might be, the opportunity for it to be subtly misaligned with some novel or reinterpreted facet of the other domain will always exist.

A Formal ontology could be said to be perpetually "chasing" said foreign domain, rather like a person chasing a rainbow – Always _just- out of reach.

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

## FuzzySet

TODO

## Symbolic Grounding

TODO

## MBQL

Mindbase Query Language

## Meaning

Mindbase project intends to consider two types of "Meaning" as defined by the following:

### External Meaning

"External Meaning" refers to meaning which emerges from cultural and social processes within the world.

You employ External Meaning every time you speak, listen, read, hear music, dance, or reach for the handle of that coffee cup. You learn the _meaning_ of road signs, body language, and that smokey smell coming from the kitchen. You do this via the staggeringly complex system of symbol exchange and manipulation which we refer to as "Culture" or "Civilization". Culture, and the participants thereof, perform this process via the confluence of a huge number of tiny stimuli. The first _meaningful_ words you spoke as a baby were likely in response to an agglomeration of such stimuli previously received. Your first words [Symbolized](./GLOSSARY.md#Symbol) those previous stimuli

### Internal Meaning

"Internal Meaning" refers to the meaning which emerges from within Mindbase itself as the sum of its constituent data.  

Of course there exists a yawning gulf in terms of scale, and a profound impedence mismatch between the excuisite processes of External Meaning, and the puny machinations of any kind of semantic database. For example, there are infinite senses and implications of the word "is", because symbols like "is", and the sound that a person makes when pronouncing or hearing are each _individual occurrances._ Prescriptivists should rightly tremble in fear from the mighty cultural 

So how on earth can we ever hope to put even a tiny dent in capturing this universe of meaning?

The goal of Mindbase is to representing meaning in a way that is BOTH programmatically useful, and able to interface with the nuance of the world _internally_, such that meaningful computation can be performed (emergently) inside the Mindbase system. Until such time as Mindbase expands to fill the entire observable universe (har har), there will necessarily be some logical boundary between its internal xpression of meaning, and that of the outside world. We cannot blindly subordinate the one to the other. That would put us right back where we started. We must build a bridge between these systems of internal and external meaning. This requires that such a system be cognizant that a lot of meaning is defined by human culture at large, or other external systems like the semantics of a docx file. Because of this necessity to represent `External Meaning` we have `Artifacts` and `Allegations`

### Symbol

A Symbol is a set of [Atoms](./GLOSSARY.md#Atom) – The agglomeration of which represents some abstract concept. Each of these Atoms represents some [Allegation](./GLOSSARY.md#Allegation) which documents the instance of something according to a given agent. Individually, each of these Atoms doesn't [mean](./GLOSSARY.md#internal-meaning) much. Collectively, they serve as tentpoles, expanding or limiting the degrees of freedom within some abstract semantic space.  
