use super::{
    ast,
    error::{
        MBQLError,
        MBQLErrorKind,
    },
};
use crate::{
    ArtifactId,
    Concept,
    MBError,
    MindBase,
};
use std::{
    collections::BTreeMap,
    sync::Mutex,
};

#[derive(Debug)]
pub struct Query {
    pub symbol_statements:   Vec<ast::SymbolStatement>,
    pub artifact_statements: Vec<ast::ArtifactStatement>,
    pub artifact_var_map:    Mutex<BTreeMap<String, (usize, Option<ArtifactId>)>>,
    pub symbol_var_map:      Mutex<BTreeMap<String, (usize, Option<Concept>)>>,
}

impl Query {
    pub fn new<T: std::io::BufRead>(reader: T) -> Result<Self, MBQLError> {
        let mut query = Query { symbol_statements:   Vec::new(),
                                artifact_statements: Vec::new(),
                                artifact_var_map:    Mutex::new(BTreeMap::new()),
                                symbol_var_map:      Mutex::new(BTreeMap::new()), };
        super::parse::parse(reader, &mut query)?;

        Ok(query)
    }

    pub fn add_artifact_statement(&mut self, statement: ast::ArtifactStatement) {
        let idx = self.artifact_statements.len();
        self.artifact_var_map
            .lock()
            .unwrap()
            .insert(statement.var.var.clone(), (idx, None));
        self.artifact_statements.push(statement);
    }

    pub fn add_symbol_statement(&mut self, statement: ast::SymbolStatement) {
        let idx = self.symbol_statements.len();
        if let Some(var) = &statement.var {
            self.symbol_var_map.lock().unwrap().insert(var.to_string(), (idx, None));
        }
        self.symbol_statements.push(statement);
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

    pub fn get_artifact_var(&self, var: &ast::ArtifactVar, mb: &MindBase) -> Result<ArtifactId, MBQLError> {
        let offset = match self.artifact_var_map.lock().unwrap().get(&var.var) {
            None => {
                return Err(MBQLError { position: var.position.clone(),
                                       kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, })
            },
            Some((offset, maybe_artifact_id)) => {
                if let Some(artifact_id) = maybe_artifact_id {
                    return Ok(artifact_id.clone());
                }
                offset.clone()
            },
        };

        // Didn't have it yet. gotta calculate it
        let statement = self.artifact_statements.get(offset).unwrap();

        statement.apply(self, mb)
    }

    pub fn dump<T: std::io::Write>(&self, mut writer: T) -> Result<(), std::io::Error> {
        for statement in self.artifact_statements.iter() {
            statement.write(&mut writer)?;
        }
        for statement in self.symbol_statements.iter() {
            statement.write(&mut writer)?;
        }

        Ok(())
    }

    pub fn apply(&self, mb: &MindBase) -> Result<(), MBQLError> {
        // iterate over all artifact statements and store
        // iterate over all symbol statements and recurse

        // gotta start somewhere
        // could be a cyclic graph
        // even artifacts must be able to recurse symbols

        for statement in self.artifact_statements.iter() {
            let _artifact_id = statement.apply(self, mb)?;
            // Ignore this artifact_id because it's being stored inside the apply.
            // We have to do this because it's possible to have artifacts/symbols that recursively reference artifact variables
        }

        for statement in self.symbol_statements.iter() {
            let _symbol = statement.apply(self, mb)?;
            // Ignore this symbol because it's being stored inside the apply.
            // We have to do this because it's possible to have artifacts/symbols that recursively reference symbol variables
        }

        Ok(())
    }
}
