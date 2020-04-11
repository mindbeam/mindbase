// The following MBQL statement
// $emote = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
//
// generates this AST Structure:
//                           Ground
//                             |
//                  GroundSymbolizable::GroundPair
//                                 |
//                GroundPair{ left    right    }
//                            /          \
//       GSymbolizable::GroundPair       GSymbolizable::GroundPair
//                 |                                    |
//       GPair{left right}                    GPair{left right}
//             /        \                         /        \
//    GSym::Artifact  GSym::Artifact   GSym::Artifact  GSym::Artifact
//                |           |             |           |
// Which should   |           |             |           |
// Result in      |           |             |           |
// these symbol   |           |             |           |
// being created: |           |             |           |
//                |           |             |           |
//         # S1[Smile]   S2[Mouth]      S3[Wink]    S4[Eye]
//         #  \__________/               \__________/
//         #      [A1]                       [A2]
//         #        \_________________________/
//         #                    [A3] (Emotive Movement)
//
//
// Game plan:
// * Recursively walk the AST
// * left / right Depth first
// * Given a symbol, use that symbol as is for comparison
// * Upon hitting an artifact, Look up all atoms (AllegationID) for the left and right item, filtered by ground agent
// * identify the set of Allegations which intersect(left0, left1) AND intersect(right0, right1) NOTE: how do we ensure that
//   intersect(left0, right1) AND intersect(right0, left1) are also checked?
// * The set of those passing allegations is returned as a Symbol

// Index `atoms_by_artifact_agent` indexes ALL allegations, keyed on ArtifactID + AgentID returned for `referenced_artifacts`
// which is vicarious for analogies (how many levels removed?)
// This might be doing close to what we want already
//
// Index `analogy_rev` contains only analogies (AllegationID) keyed on left-hand symbol atoms (AllegationIDs)
// I'm not certain if this is useful for much
