use crate::{
    allegation::{
        Allegation,
        Body,
    },
    mbql::ast,
    AgentId,
    AllegationId,
    Analogy,
    ArtifactId,
    MBError,
    MindBase,
};

use ast::{
    GPair,
    GSymbolizable,
};

/// Context object for low level search operations
pub struct SearchContext<'a> {
    scan_min:  [u8; 64],
    scan_max:  [u8; 64],
    gs_agents: Vec<AgentId>,
    pub mb:    &'a MindBase,
}

impl<'a> SearchContext<'a> {
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

    fn find_matching_analogies(&self, search_item: GSymbolizable) -> Result<(), MBError> {
        // What am I doing:
        // I'm searching for existing analogies which pertain to these symbols
        // * L/R might itself be a type of symbol – If you think about it, this is an analogy.
        // So it's not [L,L,L] and [R,R,R] but [LR,LR,LR] - this would make much more sense
        // But where is L and R coming from?

        for analogy in self.all_gs_analogies() {
            let (analogy, id): (Analogy, AllegationId) = analogy?;
            // This is just one Atom. Is it one of the ones I'm looking for?

            // Associative Analogies have two parts
            let left = &analogy.left;
            let right = &analogy.right;

            match &search_item {
                GSymbolizable::Artifact(_) => unimplemented!(),
                GSymbolizable::SymbolVar(_) => unimplemented!(),
                GSymbolizable::Ground(_) => unimplemented!(),
                GSymbolizable::GroundPair(GPair { left, right, .. }) => {
                    //
                },
            }
        }

        unimplemented!()
    }

    /// Returns an iterator over all Analogies which were alledged by our ground-symbol agents
    fn all_gs_analogies(&self) -> impl Iterator<Item = Result<(Analogy, AllegationId), MBError>> {
        let gs_agents = self.gs_agents.clone();
        self.mb.allegation_iter().filter_map(move |allegation| {
                                     match allegation {
                                         Ok((id,
                                             Allegation { body: Body::Analogy(analogy),
                                                          agent_id,
                                                          .. }))
                                             if gs_agents.contains(&agent_id) =>
                                         {
                                             Some(Ok((analogy, id)))
                                         },
                                         Ok(_) => None,
                                         Err(e) => Some(Err(e)),
                                     }
                                 })
    }

    /// Returns a binary vector containing 16 all byte Atoms which represent symbolizations of the given artifact
    pub fn artifact_atom_vec(&mut self, search_artifact_id: &ArtifactId) -> Result<Vec<u8>, MBError> {
        self.scan_min[0..32].copy_from_slice(search_artifact_id.as_ref());
        self.scan_max[0..32].copy_from_slice(search_artifact_id.as_ref());

        let iter = self.mb.atoms_by_artifact_agent.range(&self.scan_min[..]..=&self.scan_max[..]);

        use inverted_index_util::entity_list::insert_entity_mut;

        use typenum::consts::U16;
        let mut out: Vec<u8> = Vec::new();
        for item in iter {
            let (key, atom_list) = item?;
            // atom_list is a Vec[u8] containing a sorted sequence of 16 bit atom ids

            // TODO - differentiate (keys or list items) based on the type and vicariousness of artifact -> atom
            // Is this a direct symbolization of that artifact? or an Analogy?
            // At present, we are only indexing direct symbolizations, so we can cheat and skip this

            let item_agent_id = &key[32..64];
            // Remember we're searching for a range of agent ids. Have to confirm it's in the list
            if let Err(_) = self.gs_agents.binary_search_by(|a| a.as_ref()[..].cmp(item_agent_id)) {
                // No, it's not present in the list. Punt
                continue;
            }

            if out.len() == 0 {
                out.extend(&atom_list[..])
            } else {
                for chunk in atom_list.chunks(16) {
                    insert_entity_mut::<U16>(&mut out, chunk)
                }
            }
        }

        Ok(out)
    }
}
