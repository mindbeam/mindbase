# Glossary

## Agent

A user, or other automated process which is capable of making allegations. Each agent has a cryptographic identity which is used to sign Allegations.  

## Atom

A reference to the unique identity of some specific Allegation.

## Artifact

Some blob of data which may be meaningful to the outside world, likely containing `External Meaning`, but is otherwise devoid of `Internal meaning`, at least by its existence.

## Allegation

An event or declaration which is being documented by an agent But it's also more than that – It's an instance of some payload. At present, such payload might be any of the following types:

* ArtifactId - Content-addressable ID of a given `Artifact`.  
* Analogy - Associate one `Symbol` with another `Symbol` (Eg: There exists some Symbol C that represents the connection of Symbol A and Symbol B)
* AgentId - PubKey of an Agent
* Unit - Empty payload.  

Though an Allegation may reference one or more `Artifacts`, implying that the allegation is somehow an _instance_ of that `Artifact` it doesn't even need to be an instance of something per se. It could be a `Unit` Allegation, which is just an anonymous enumeration.  
Each Allegation has a Payload.

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
