use super::{
    ast,
    error::{
        MBQLError,
        MBQLErrorKind,
    },
};
use crate::{
    ground::GSContext,
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

pub struct Query<'a> {
    pub statements:       Vec<ast::Statement>,
    pub artifact_var_map: Mutex<BTreeMap<String, (usize, Option<ArtifactId>)>>,
    pub symbol_var_map:   Mutex<BTreeMap<String, (usize, Option<Symbol>)>>,
    pub gscontext:        Mutex<GSContext<'a>>,
    pub mb:               &'a MindBase,
}

impl<'a> Query<'a> {
    pub fn new<T: std::io::BufRead>(mb: &'a MindBase, reader: T) -> Result<Self, MBQLError> {
        let mut query = Query { statements: Vec::new(),
                                artifact_var_map: Mutex::new(BTreeMap::new()),
                                symbol_var_map: Mutex::new(BTreeMap::new()),
                                gscontext: Mutex::new(GSContext::new(mb)),
                                mb };
        super::parse::parse(reader, &mut query)?;

        Ok(query)
    }

    pub fn from_str(mb: &'a MindBase, mbql_string: &str) -> Result<Self, MBQLError> {
        let cur = Cursor::new(mbql_string);
        Self::new(mb, cur)
    }

    pub fn add_statement(&mut self, statement: ast::Statement) {
        let idx = self.statements.len();

        match &statement {
            ast::Statement::Artifact(s) => {
                self.artifact_var_map.lock().unwrap().insert(s.var.var.clone(), (idx, None));
            },
            ast::Statement::Symbol(s) => {
                if let Some(var) = &s.var {
                    self.symbol_var_map.lock().unwrap().insert(var.to_string(), (idx, None));
                }
            },

            ast::Statement::Diag(_) => {},
        };

        self.statements.push(statement);
    }

    // Have to be able to write independently, as Artifact variables may be evaluated recursively
    pub fn store_artifact_for_var(&self, var: &ast::ArtifactVar, artifact_id: ArtifactId) -> Result<(), MBQLError> {
        match self.artifact_var_map.lock().unwrap().get_mut(&var.var) {
            None => {
                return Err(MBQLError { position: var.position.clone(),
                                       kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, })
            },
            Some(v) => v.1 = Some(artifact_id),
        }
        Ok(())
    }

    pub fn get_artifact_var(&self, var: &str) -> Result<Option<ArtifactId>, MBError> {
        let offset = match self.artifact_var_map.lock().unwrap().get(var) {
            None => return Ok(None),
            Some((offset, maybe_artifact_id)) => {
                if let Some(artifact_id) = maybe_artifact_id {
                    return Ok(Some(artifact_id.clone()));
                }
                offset.clone()
            },
        };

        // Didn't have it yet. gotta calculate it
        if let ast::Statement::Artifact(statement) = self.statements.get(offset).unwrap() {
            return Ok(Some(statement.apply(self)?));
        } else {
            panic!("Sanity error");
        }
    }

    pub fn store_symbol_for_var(&self, var: &ast::SymbolVar, symbol: Symbol) -> Result<(), MBQLError> {
        match self.symbol_var_map.lock().unwrap().get_mut(&var.var) {
            None => {
                return Err(MBQLError { position: var.position.clone(),
                                       kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, })
            },
            Some(v) => v.1 = Some(symbol),
        }
        Ok(())
    }

    pub fn get_symbol_var(&self, var: &str) -> Result<Option<Symbol>, MBError> {
        let offset = match self.symbol_var_map.lock().unwrap().get(var) {
            None => return Ok(None),
            Some((offset, maybe_symbol)) => {
                if let Some(symbol) = maybe_symbol {
                    return Ok(Some(symbol.clone()));
                }
                offset.clone()
            },
        };

        // Didn't have it yet. gotta calculate it
        if let ast::Statement::Symbol(statement) = self.statements.get(offset).unwrap() {
            return Ok(Some(statement.apply(self)?));
        } else {
            panic!("Sanity error");
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
