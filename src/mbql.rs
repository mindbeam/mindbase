pub mod ast;
pub mod display;
pub mod error;
pub mod parse;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Query {
    pub artifacts: BTreeMap<String, ast::Artifact>,
}

impl Query {
    pub fn new<T: std::io::BufRead>(reader: T) -> Result<Self, self::error::Error> {
        let query = Query { artifacts: BTreeMap::new(), };
        self::parse::parse(reader, query)?;

        Ok(query)
    }
}

// fn parse_line(pair: Pair<Rule>) ->{}
