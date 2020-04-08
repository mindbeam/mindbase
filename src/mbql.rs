pub mod ast;
pub mod display;
pub mod error;
pub mod parse;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Query {
    pub artifact_statements: BTreeMap<String, ast::ArtifactStatement>,
}

impl Query {
    pub fn new<T: std::io::BufRead>(reader: T) -> Result<Self, self::error::Error> {
        let mut query = Query { artifact_statements: BTreeMap::new(), };
        self::parse::parse(reader, &mut query)?;

        Ok(query)
    }

    pub fn add_artifact_statement(&mut self, statement: ast::ArtifactStatement) {
        self.artifact_statements.insert(statement.var.var.clone(), statement);
    }

    pub fn dump<T: std::io::Write>(&self, mut writer: T) -> Result<(), crate::error::Error> {
        for (v, statement) in self.artifact_statements.iter() {
            statement.write(&mut writer)?;
        }

        Ok(())
    }
}

// fn parse_line(pair: Pair<Rule>) ->{}
