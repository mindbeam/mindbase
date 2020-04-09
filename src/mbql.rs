pub mod ast;
pub mod error;
pub mod parse;
use std::collections::BTreeMap;

// #[derive(Debug)]
pub struct Query {
    pub artifact_statements_map: BTreeMap<String, ast::ArtifactStatement>,
    pub symbol_statements_map:   BTreeMap<String, usize>,
    pub symbol_statements:       Vec<ast::SymbolStatement>,
}

impl Query {
    pub fn new<T: std::io::BufRead>(reader: T) -> Result<Self, self::error::Error> {
        let mut query = Query { artifact_statements_map: BTreeMap::new(),
                                symbol_statements_map:   BTreeMap::new(),
                                symbol_statements:       Vec::new(), };
        self::parse::parse(reader, &mut query)?;

        Ok(query)
    }

    pub fn add_artifact_statement(&mut self, statement: ast::ArtifactStatement) {
        self.artifact_statements_map.insert(statement.var.var.clone(), statement);
    }

    pub fn add_symbol_statement(&mut self, statement: ast::SymbolStatement) {
        let idx = self.symbol_statements.len();
        if let Some(var) = &statement.var {
            self.symbol_statements_map.insert(var.to_string(), idx);
        }
        self.symbol_statements.push(statement);
    }

    pub fn dump<T: std::io::Write>(&self, mut writer: T) -> Result<(), crate::error::Error> {
        for (v, statement) in self.artifact_statements_map.iter() {
            statement.write(&mut writer)?;
        }
        for statement in self.symbol_statements.iter() {
            statement.write(&mut writer)?;
        }

        Ok(())
    }
}

// fn parse_line(pair: Pair<Rule>) ->{}
