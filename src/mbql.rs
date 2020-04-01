pub mod ast;
pub mod display;
pub mod error;
pub mod parse;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Query {
    pub items: BTreeMap<String, ast::Item>,
}

impl Query {
    pub fn new<T: std::io::BufRead>(reader: T) -> Result<Self, self::error::Error> {
        let items = self::parse::parse(reader)?;

        let items = items.into_iter().map(|i| (i.key.clone(), i)).collect();

        Ok(Query { items })
    }
}

// fn parse_line(pair: Pair<Rule>) ->{}
