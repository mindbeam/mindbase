// The following MBQL statement
// $emote = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
//
// generates this AST Structure:
//                           Ground
//                             |
//                    GSymbolizable::GroundPair
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
    allegation::Body,
    mbql::{
        ast,
        error::{
            MBQLError,
            MBQLErrorKind,
        },
        Query,
    },
    AgentId,
    AllegationId,
    ArtifactId,
    MBError,
    MindBase,
    Symbol,
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
    pub fn symbolize(&mut self, symbolizable: &ast::GSymbolizable, query: &Query) -> Result<Symbol, MBError> {
        // As a temporary measure, we are doing a fairly inefficient process of building a Symbol for each symbolizable artifact
        // with all possible symbolic atoms and THEN narrowing that.
        //
        // Later, we should be able to improve this with strategic indexing such that the narrowing step is less burdensome (or
        // even unnecessary) and that roundtripping to the data storage layer is reduced

        // TODO - create a shared context which can be used for a rolling index intersection process
        // TODO - change this to not return a symbol, but rather to mutate the context
        let symbol = self.symbolize_recurse(symbolizable, query)?;

        // TODO - convert the rolling index intersection into a symbol and Return.

        Ok(symbol)
    }

    fn symbolize_recurse(&mut self, s: &ast::GSymbolizable, query: &Query) -> Result<Symbol, MBError> {
        //

        let symbol = match s {
            ast::GSymbolizable::Artifact(a) => {
                let artifact_id = a.apply(query)?;
                self.single_artifact(&artifact_id)?
            },
            ast::GSymbolizable::GroundPair(a) => {
                // Symbol grounding is the crux of the biscuit
                // We don't want to create new symbols if we can possibly help it
                // We want to try reeally hard to find existing symbols
                // And only create a new one if we positively must

                // Depth-first search

                let left = self.symbolize_recurse(&*a.left, query)?;
                let right = self.symbolize_recurse(&*a.right, query)?;

                // find symbols (Analogies) which refer to both of the above
                self.find_matching_analogy_symbol(&left, &right)?
            },
            ast::GSymbolizable::SymbolVar(sv) => {
                //
                if let Some(symbol) = query.get_symbol_var(&sv.var)? {
                    symbol
                } else {
                    return Err(MBQLError { position: sv.position.clone(),
                                           kind:     MBQLErrorKind::SymbolVarNotFound { var: sv.var.clone() }, }.into());
                }
            },
            ast::GSymbolizable::Ground(_) => {
                // Shouldn't be able to call this directly with a Ground statement
                unreachable!()
            },
        };

        if symbol.is_null() {
            panic!("It's bad mmkay");
        }

        Ok(symbol)
    }

    fn single_artifact(&mut self, search_artifact_id: &ArtifactId) -> Result<Symbol, MBError> {
        self.scan_min[0..32].copy_from_slice(search_artifact_id.as_ref());
        self.scan_max[0..32].copy_from_slice(search_artifact_id.as_ref());

        let iter = self.mb.atoms_by_artifact_agent.range(&self.scan_min[..]..=&self.scan_max[..]);

        use inverted_index_util::entity_list::insert_entity_mut;

        use typenum::consts::U16;
        let mut unified: Vec<u8> = Vec::new();
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
        let symbol = Symbol { members,
                              spread_factor: 0.0 };

        Ok(symbol)
    }

    // It's not really just one analogy that we're searching for, but a collection of N analogies which match left and right
    fn find_matching_analogy_symbol(&self, left: &Symbol, right: &Symbol) -> Result<Symbol, MBError> {
        // Brute force for now. This whole routine is insanely inefficient
        // TODO 2 - update this to be a sweet indexed query!

        let mut atoms: Vec<AllegationId> = Vec::new();

        for allegation in self.mb.allegation_iter() {
            let (allegation_id, allegation) = allegation?;

            match allegation.body {
                Body::Analogy(analogy) => {
                    //
                    if self.gs_agents.contains(&allegation.agent_id) {
                        // TODO 2 - This is crazy inefficient
                        if intersect_symbols(left, &analogy.left) && intersect_symbols(right, &analogy.right) {
                            // atoms.push(Regular(allegation_id))
                            atoms.push(allegation_id)
                        } else if intersect_symbols(left, &analogy.right) && intersect_symbols(right, &analogy.left) {
                            // TODO 2 - QUESTION - should we preserve chirality in the symbol member list? I think we may need to
                            // atoms.push(Reverse(allegation_id)) // Uno reverse card yo
                            atoms.push(allegation_id)
                        }
                    }
                },
                _ => {},
            }
        }

        // Create a Symbol which contains the composite symbol atoms of all Analogies made by ground symbol agents
        return Ok(Symbol { members:       atoms,
                           spread_factor: 0.0, });
    }
}

fn intersect_symbols(a: &Symbol, b: &Symbol) -> bool {
    // This is crazy inefficient. At least do a lexicographic presort
    // can probably eliminate this during rolling inverted index conversion
    // let mut out: Vec<AllegationId> = Vec::new();
    for member in a.members.iter() {
        if b.members.contains(member) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::{
        mbql::Query,
        MindBase,
    };
    use std::io::Cursor;

    #[test]
    fn ground1() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let mbql = Cursor::new(
                               r#"
            $foo = Allege(("Smile" : "Mouth") : ("Wink":"Eye"))
            $bar = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
            Diag($foo, $bar)
        "#,
        );

        let query = Query::new(&mb, mbql)?;
        query.apply()?;

        let bogus = query.get_symbol_var("bogus")?;
        assert_eq!(bogus, None);

        let foo = query.get_symbol_var("foo")?.expect("foo");
        let bar = query.get_symbol_var("bar")?.expect("bar");

        assert_eq!(foo, bar);
        assert!(foo.intersects(&bar));

        Ok(())
    }

    #[test]
    fn ground2() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let mbql = Cursor::new(r#"$gs = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))"#);

        let query = Query::new(&mb, mbql)?;
        query.apply()?;

        let _foo = query.get_symbol_var("gs")?.expect("gs");

        Ok(())
    }

    #[test]
    fn ground3() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath).unwrap();

        let mbql = Cursor::new(
                               r#"
            $foo = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
            $bar = Ground(("Smile" : "Mouth") : ("Wink" : "Eye"))
            Diag($foo, $bar)
        "#,
        );

        let query = Query::new(&mb, mbql)?;
        query.apply()?;

        let foo = query.get_symbol_var("foo")?.expect("foo");
        let bar = query.get_symbol_var("bar")?.expect("bar");

        assert_eq!(foo, bar);
        assert!(foo.intersects(&bar));

        Ok(())
    }
}
