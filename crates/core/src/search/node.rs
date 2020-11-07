use super::SearchContext;
use crate::{
    analogy::Analogy,
    artifact::ArtifactId,
    claim::{Claim, ClaimId},
    error::MBError,
    mbql::{
        ast,
        error::{MBQLError, MBQLErrorKind},
        query::BindResult,
        Query,
    },
    symbol::Symbol,
};
use std::rc::Rc;

pub enum SearchNode {
    Artifact {
        binary_concatenated_atomid_list: Option<Vec<u8>>,
        artifact_id: ArtifactId,
    },
    Pair {
        vec: Option<Vec<u8>>,
        left: Box<SearchNode>,
        right: Box<SearchNode>,
    },

    Bound {
        node: Box<SearchNode>,
        sv: Rc<ast::SymbolVar>,
    },

    // Someone gave us this symbol, and said "use it", so there's nothing to be done
    Given {
        vec: Option<Vec<u8>>,
    },
}

impl SearchNode {
    pub fn search(query: &Query, symz: &Rc<ast::GSymbolizable>) -> Result<SearchNode, MBQLError> {
        println!("SEARCH {:?}", *symz);
        let node = match &**symz {
            ast::GSymbolizable::Artifact(a) => SearchNode::artifact_search(query, a)?,
            ast::GSymbolizable::GroundPair(a) => SearchNode::pair_search(query, a)?,
            ast::GSymbolizable::SymbolVar(sv) => SearchNode::symbolvar_search(query, sv)?,

            ast::GSymbolizable::Ground(_) => {
                // Shouldn't be able to call this directly with a Ground statement
                unreachable!()
            }
        };

        Ok(node)
    }

    pub fn artifact_search(query: &Query, artifact: &ast::Artifact) -> Result<Self, MBQLError> {
        let artifact_id = artifact.apply(query)?;

        let binary_concatenated_atomid_list = {
            let mut ctx = query.search_context.lock().unwrap();
            ctx.query_atoms_by_artifact(&artifact_id)?
        };

        Ok(SearchNode::Artifact {
            artifact_id,
            binary_concatenated_atomid_list: Some(binary_concatenated_atomid_list),
        })
    }

    /// Search for symbols for a given symbol variable. Said variable is either a given, or a Bound variable, depending on how
    /// it's defined in the query
    pub fn symbolvar_search(query: &Query, sv: &Rc<ast::SymbolVar>) -> Result<Self, MBQLError> {
        match query.bind_symbolvar(&sv.var) {
            Err(e) => {
                return Err(MBQLError {
                    position: sv.position().clone(),
                    kind: MBQLErrorKind::SymbolVarNotFound { var: sv.var.to_string() },
                });
            }
            Ok(BindResult::Bound(gsymz)) => {
                let node = SearchNode::search(query, &gsymz)?;

                Ok(SearchNode::Bound {
                    node: Box::new(node),
                    sv: sv.clone(),
                })
            }
            Ok(BindResult::Symbol(symbol)) => Ok(SearchNode::Given { vec: symbol.as_vec() }),
        }
    }

    pub fn pair_search(query: &Query, gpair: &ast::GPair) -> Result<Self, MBQLError> {
        // Depth first recursion to find possible leaf symbols
        let left = SearchNode::search(query, &gpair.left)?;
        let right = SearchNode::search(query, &gpair.right)?;

        let union = left.union_vec(&right);

        match union {
            None => Ok(SearchNode::Pair {
                vec: None,
                left: Box::new(left),
                right: Box::new(right),
            }),
            Some(v) => {
                // find symbols (Analogies) which refer to BOTH of the above
                println!("{:?}", v);
                unimplemented!()
            }
        }

        // I'm searching for Analogies which match both the left and the right
        // AND I'm also searching for that set of left/right atoms which match said analogies, which I need to call
        // store_symbol_for_var on if they're GSNode::Bound
        // let opt_symbol = ctx.find_matching_analogy_symbol(&left, &right, query)?;

        // if let Some(symbol) = opt_symbol {
        //     println!("FOUND MATCH {}", symbol);
        //     return Ok(SearchNode::Pair { left:  Box::new(left),
        //                                  right: Box::new(right), });
        // }
    }

    fn intersect(&mut self) {}

    pub fn stash_bindings(&self, query: &Query) -> Result<(), MBError> {
        match self {
            SearchNode::Pair { left, right, .. } => {
                left.stash_bindings(query)?;
                right.stash_bindings(query)?;
                Ok(())
            }
            SearchNode::Bound { node, sv } => match node.symbol() {
                None => Err(MBError::Other),
                Some(symbol) => {
                    query.stash_symbol_for_var(&sv, symbol)?;
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }

    pub fn symbol(&self) -> Option<Symbol> {
        match self {
            SearchNode::Artifact {
                binary_concatenated_atomid_list: vec,
                ..
            }
            | SearchNode::Pair { vec, .. }
            | SearchNode::Given { vec, .. } => match vec {
                None => None,
                Some(v) => Symbol::new_from_vec(v.clone()),
            },
            SearchNode::Bound { node, .. } => node.symbol(),
        }
    }

    pub fn vivify_symbols(&mut self, query: &Query) -> Result<(), MBError> {
        match self {
            SearchNode::Artifact {
                artifact_id,
                binary_concatenated_atomid_list: vec,
            } => {
                let atom = query.mb.symbolize_atom(&*artifact_id)?;
                overwrite_vec(vec, atom.id());
            }
            SearchNode::Bound { node, .. } => {
                node.vivify_symbols(query)?;
            }
            SearchNode::Pair { left, right, vec } => {
                left.vivify_symbols(query)?;
                right.vivify_symbols(query)?;

                let atom = query
                    .mb
                    .symbolize_atom(Analogy::declarative(left.symbol().unwrap(), right.symbol().unwrap()))?;

                overwrite_vec(vec, atom.id());
            }
            SearchNode::Given { .. } => {
                // Can't resymbolize/vivify a given
            }
        };

        Ok(())
    }

    fn vec(&self) -> Option<&Vec<u8>> {
        match self {
            SearchNode::Artifact {
                binary_concatenated_atomid_list: vec,
                ..
            }
            | SearchNode::Pair { vec, .. }
            | SearchNode::Given { vec, .. } => vec.as_ref(),
            SearchNode::Bound { node, .. } => node.vec(),
        }
    }

    fn vec_mut(&mut self) -> &mut Option<Vec<u8>> {
        match self {
            SearchNode::Artifact {
                binary_concatenated_atomid_list: vec,
                ..
            }
            | SearchNode::Pair { vec, .. }
            | SearchNode::Given { vec, .. } => vec,
            SearchNode::Bound { node, .. } => node.vec_mut(),
        }
    }

    fn union_vec(&self, other: &Self) -> Option<Vec<u8>> {
        let a = self.vec();
        let b = other.vec();

        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (Some(a), Some(b)) => {
                let mut merged = Vec::with_capacity(a.len() + b.len());
                merged.extend(a.iter().copied());

                use inverted_index_util::entity_list::insert_entity_mut;
                use typenum::consts::U16;
                for chunk in b.chunks(16) {
                    insert_entity_mut::<U16>(&mut merged, chunk)
                }

                Some(merged)
            }
        }
    }
}

fn overwrite_vec(vec: &mut Option<Vec<u8>>, atom: &ClaimId) {
    match vec {
        None => {
            let mut v = Vec::new();
            v.extend(atom.as_ref());
            *vec = Some(v);
        }
        Some(v) => {
            v.truncate(0);
            v.extend(atom.as_bytes());
        }
    }
}
// At each stage, I am searching for a set of
//   * Instantiated artifacts (ID)
//   * Associative Analogies (ID)
//   * Catagorical Analogies (ID)
// Lets ignore Given symbols for now
//
// so what are our cardinalities here?
// If I were to start at the root of a given GSymz tree, It would initially encompass all the data in the system
// Lets assume this for a moment. What do you do next?
// you iterate over each record, then recursively check its contents for matching
// Lets just fucking do this, but do it as a module!
// Make it work, make it correct, make it fast. Not the reverse :facepalm:
