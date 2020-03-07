@0xd505fc2e9324f891;

struct AllegationId{
    x0 @0 :UInt64;
    x1 @1 :UInt64;
}
struct AgentId{
    x0 @0 :UInt64;
    x1 @1 :UInt64;
    x2 @2 :UInt64;
    x3 @3 :UInt64;
}
struct ArtifactId{
    x0 @0 :UInt64;
    x1 @1 :UInt64;
    x2 @2 :UInt64;
    x3 @3 :UInt64;
}

struct Signature {
  x0 @0 :UInt64;
  x1 @1 :UInt64;
  x2 @2 :UInt64;
  x3 @3 :UInt64;
  x4 @4 :UInt64;
  x5 @5 :UInt64;
  x6 @6 :UInt64;
  x7 @7 :UInt64;
}

# An allegation is a "Symbol" or a proto-concept which is defined by a specific Agent
struct Allegation {
    id      @0 : AllegationId;
    agentId @1 : AgentId;
    # TODO 2 - Context (Date, time, place, etc)

    union {
        # A Unit Allegation a globally unique entity with no payload
        unit @2  : Void;

        # An Agent Allegation is a globally unique reference to an actual Agent
        # It is conceivabe that someone could want to construct different Allegations referencing the same AgentId
        # Which are otherwise identical except for their AllegationId.
        agent @3  : AgentId;

        # Your heart is like a Rose (both of are in the category of [red things])
        analogy @4 : Analogy;

        # Reference to a bucket of bits. Different allegations referencing the
        # same artifact may be referring to different semantic dimensions of it 
        artifact @5 : ArtifactId;
    }

    signature @ 6: Signature;
}

struct Analogy {
    confidence @0 : Float32;
    concept    @1 : Concept;
    memberof   @2 : Concept;
}

# Concept
# A container for Allegations (AKA proto-concepts) which are similar in some dimension(s) in semantic space

struct Concept {
    spreadFactor @0 : Float32;
    members       @1 : List(AllegationId);
}