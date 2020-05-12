use super::{
    ast,
    error::{
        MBQLError,
        MBQLErrorKind,
    },
    Position,
};
use crate::{
    search::SearchContext,
    ArtifactId,
    MBError,
    MindBase,
    Symbol,
};
use std::{
    collections::BTreeMap,
    io::Cursor,
    sync::Mutex,
};

struct ArtifactVarMapItem {
    offset: usize,
    id:     Option<ArtifactId>,
}

struct SymbolVarMapItem {
    offset:   usize,
    symbol:   Option<Symbol>,
    is_bound: bool,
}

use std::rc::Rc;

// #[derive(Debug, Clone)]
// pub enum Bindable {
//     Symbolizable(Rc<ast::Symbolizable>),
//     GSymbolizable(Rc<ast::GSymbolizable>),
// }

// impl Bindable {
//     pub fn position(&self) -> &Position {
//         match self {
//             Bindable::Symbolizable(s) => s.position(),
//             Bindable::GSymbolizable(s) => s.position(),
//         }
//     }
// }

pub struct Query<'a> {
    pub statements:     Vec<ast::Statement>,
    artifact_var_map:   Mutex<BTreeMap<String, ArtifactVarMapItem>>,
    symbol_var_map:     Mutex<BTreeMap<String, SymbolVarMapItem>>,
    pub search_context: Mutex<SearchContext<'a>>,
    pub mb:             &'a MindBase,
}

pub enum BindResult {
    Bound(Rc<ast::GSymbolizable>),
    Symbol(Symbol),
}

impl<'a> Query<'a> {
    pub fn new<T: std::io::BufRead>(mb: &'a MindBase, reader: T) -> Result<Self, MBQLError> {
        let mut query = Query { statements: Vec::new(),
                                artifact_var_map: Mutex::new(BTreeMap::new()),
                                symbol_var_map: Mutex::new(BTreeMap::new()),
                                search_context: Mutex::new(SearchContext::new(mb)),
                                mb };
        super::parse::parse(reader, &mut query)?;

        Ok(query)
    }

    pub fn from_str(mb: &'a MindBase, mbql_string: &str) -> Result<Self, MBQLError> {
        let cur = Cursor::new(mbql_string);
        Self::new(mb, cur)
    }

    pub fn add_statement(&mut self, statement: ast::Statement) {
        let offset = self.statements.len();

        match &statement {
            ast::Statement::Artifact(s) => {
                let mut avm = self.artifact_var_map.lock().unwrap();
                avm.insert(s.var.var.clone(), ArtifactVarMapItem { offset, id: None });
            },

            ast::Statement::Bind(s) => {
                let mut svm = self.symbol_var_map.lock().unwrap();
                svm.insert(s.sv.var.to_string(),
                           SymbolVarMapItem { offset,
                                              symbol: None,
                                              is_bound: false });
            },
            ast::Statement::Symbol(s) => {
                if let Some(var) = &s.var {
                    let mut svm = self.symbol_var_map.lock().unwrap();
                    svm.insert(var.to_string(),
                               SymbolVarMapItem { offset,
                                                  symbol: None,
                                                  is_bound: false });
                }
            },

            ast::Statement::Diag(_) => {},
        };

        self.statements.push(statement);
    }

    // Have to be able to write independently, as Artifact variables may be evaluated recursively
    pub fn stash_artifact_for_var(&self, var: &ast::ArtifactVar, artifact_id: ArtifactId) -> Result<(), MBQLError> {
        match self.artifact_var_map.lock().unwrap().get_mut(&var.var) {
            None => {
                return Err(MBQLError { position: var.position.clone(),
                                       kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, })
            },
            Some(v) => v.id = Some(artifact_id),
        }
        Ok(())
    }

    pub fn get_artifact_var(&self, var: &str) -> Result<Option<ArtifactId>, MBError> {
        let offset = match self.artifact_var_map.lock().unwrap().get(var) {
            None => return Ok(None),
            Some(ArtifactVarMapItem { offset, id }) => {
                if let Some(artifact_id) = id {
                    return Ok(Some(artifact_id.clone()));
                }
                offset.clone()
            },
        };

        // Didn't have it yet. gotta calculate it
        match self.statements.get(offset).unwrap() {
            ast::Statement::Artifact(statement) => {
                return Ok(Some(statement.apply(self)?));
            },
            _ => {
                panic!("Sanity error");
            },
        }
    }

    // pub fn get_symbolizable_for_var{}

    pub fn stash_symbol_for_var(&self, var: &ast::SymbolVar, symbol: Symbol) -> Result<(), MBQLError> {
        match self.symbol_var_map.lock().unwrap().get_mut(&var.var) {
            None => {
                return Err(MBQLError { position: var.position.clone(),
                                       kind:     MBQLErrorKind::SymbolVarNotFound { var: var.var.clone() }, })
            },
            Some(v) => v.symbol = Some(symbol),
        }
        Ok(())
    }

    pub fn get_symbol_for_var(&self, var: &str) -> Result<Option<Symbol>, MBError> {
        match self.symbol_var_map.lock().unwrap().get(var) {
            None => return Ok(None),
            Some(SymbolVarMapItem { symbol, .. }) => {
                if let Some(symbol) = symbol {
                    // This could be a SymbolStatement or a BindStatement. Either way it must already be symbolized
                    return Ok(Some(symbol.clone()));
                }
            },
        };

        // We see it, but it's not set
        // Was previously doing lazy/out of order execution, but that's hard for the user to reason about
        // So we are insisting that they refer only to symbol vars which were previously set by their execution

        Err(MBError::SymbolVarNotFound)
    }

    // Bind to a `BindStatement` or return the Symbol from a SymbolStatement
    pub fn bind_symbolvar(&self, var: &str) -> Result<BindResult, MBError> {
        // Look up the symbolvar by string

        match self.symbol_var_map.lock().unwrap().get_mut(var) {
            None => {
                // TODO change this to MBError?
                // Why the distinction?
                // because it's strange to send in the bind_to only for its position?
                Err(MBError::SymbolVarNotFound)
            },
            Some(SymbolVarMapItem { offset,
                                    symbol,
                                    is_bound, }) => {
                // It should be unbound, otherwise throw an error

                if *is_bound {
                    return Err(MBError::SymbolVarAlreadyBound);
                }

                match self.statements.get(*offset).unwrap() {
                    ast::Statement::Bind(ast::BindStatement { gsymz, .. }) => {
                        // for now we're only supporting binding to other ground statements
                        *is_bound = true;
                        let foo = gsymz.clone();
                        Ok(BindResult::Bound(foo))
                    },
                    ast::Statement::Symbol(_) => {
                        match symbol {
                            Some(symbol) => Ok(BindResult::Symbol(symbol.clone())),
                            None => Err(MBError::SymbolVarNotFound),
                        }
                    },
                    _ => {
                        panic!("Sanity error");
                    },
                }
            },
        }
    }

    pub fn dump<T: std::io::Write>(&self, mut writer: T) -> Result<(), std::io::Error> {
        for statement in self.statements.iter() {
            statement.write(&mut writer)?;
        }

        Ok(())
    }

    pub fn apply(&self) -> Result<(), MBQLError> {
        // TODO 2 - Validate all possible MBQLErrors at query creation time so that all remaining errors are MBErrors
        // and then change this to return Result<(),MBError>

        // iterate over all artifact statements and store
        // iterate over all symbol statements and recurse

        // gotta start somewhere
        // could be a cyclic graph
        // even artifacts must be able to recurse symbols

        for statement in self.statements.iter() {
            statement.apply(self)?;
        }

        Ok(())
    }
}
