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

use crate::{
    mbql::{
        ast,
        Query,
    },
    AgentId,
    AllegationId,
    ArtifactId,
    Concept,
    MBError,
    MindBase,
};
use std::convert::TryInto;

pub struct GSContext<'a> {
    scan_min:  [u8; 64],
    scan_max:  [u8; 64],
    gs_agents: Vec<AgentId>,
    mb:        &'a MindBase,
}

impl<'a> GSContext<'a> {
    pub fn new(mb: &'a MindBase) -> Self {
        let gs_agents = mb.ground_symbol_agents.lock().unwrap().clone();

        let mut scan_min: [u8; 64] = [0; 64];
        scan_min[32..64].copy_from_slice(gs_agents.first().unwrap().as_ref());
        let mut scan_max: [u8; 64] = [0; 64];
        scan_max[32..64].copy_from_slice(gs_agents.last().unwrap().as_ref());

        Self { scan_min,
               scan_max,
               gs_agents,
               mb }
    }

    /// Call this with the top level GroundSymbolizable within a ground symbol statement
    pub fn symbolize(&mut self, symbolizable: &ast::GroundSymbolizable, query: &Query) -> Result<Concept, MBError> {
        // -
        // WHAT DO I WANT TO DO HERE?
        //
        // Scenario 1 - single Artifact: Ground("Foo")
        // Just go and look up all base agent symbolizations of artifact ID - No analogical traversal!
        //
        // Scenario 2 - Analogy / Artifact pair: Ground( "Foo" : "Bar" )
        //
        //
        // Scenario 3 - Nested Analogy: Ground( ("Foo" : "Bar" ) : "Blah")

        match symbolizable {
            ast::GroundSymbolizable::Artifact(a) => {
                let artifact_id = a.apply(query)?;
                self.single_artifact(artifact_id)
            },
            ast::GroundSymbolizable::GroundPair(a) => {},
            ast::GroundSymbolizable::SymbolVar(sv) => {},
            ast::GroundSymbolizable::Ground(g) => {
                // Shouldn't be able to call this directly with a Ground statement
                unreachable!()
            },
        }

        unimplemented!()
    }

    pub fn single_artifact(&mut self, search_artifact_id: &ArtifactId) -> Result<Concept, MBError> {
        self.scan_min[0..32].copy_from_slice(search_artifact_id.as_ref());
        self.scan_max[0..32].copy_from_slice(search_artifact_id.as_ref());

        let iter = self.mb.atoms_by_artifact_agent.range(&self.scan_min[..]..=&self.scan_max[..]);

        use inverted_index_util::entity_list::insert_entity_mut;

        use typenum::consts::U16;
        let mut unified: Vec<u8> = Vec::new();
        for item in iter {
            let (key, atom_list) = item?;
            // atom_list is a Vec[u8] containing a sorted sequence of 16 bit atom ids

            let item_agent_id = &key[32..64];
            // Remember we're searching for a range of agent ids. Have to confirm it's in the list
            if let Err(_) = self.gs_agents.binary_search_by(|a| a.as_ref()[..].cmp(item_agent_id)) {
                // No, it's not present in the list. Punt
                continue;
            }

            if unified.len() == 0 {
                unified.extend(&atom_list[..])
            } else {
                for chunk in atom_list.chunks(16) {
                    insert_entity_mut::<U16>(&mut unified, chunk)
                }
            }
        }

        let members: Vec<AllegationId> = unified.chunks_exact(16)
                                                .map(|c| AllegationId::from_bytes(c.try_into().unwrap()))
                                                .collect();
        let mut concept = Concept { members,
                                    spread_factor: 0.0 };

        if concept.is_null() {
            // Extend this with a new allegation so we can continue
            // We are doing this because the caller is essentially saying that there is a taxonomic relationship between
            // subsequent allegations
            concept.extend(self.mb.alledge(search_artifact_id)?.id().clone());

            // if let Some(parent) = last_concept {
            //     self.mb.symbolize(Analogy::declarative(concept.clone(), parent))?;
            // }
        }

        Ok(concept)
    }
}
// pub trait GroundSymbolize {
//     fn symbol(&self) -> Option<Concept>;
//     fn symbolize(&self, context: &mut GSContext) -> Result<Concept, MBError>;
// }

// impl GroundSymbolize for ArtifactId {
//     fn symbol(&self) -> Option<Concept> {
//         None
//     }

//     fn symbolize(&self, context: &mut GSContext) -> Result<Concept, MBError> {
//         context.single_artifact(self)
//     }
// }
