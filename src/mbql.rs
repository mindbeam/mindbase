use crate::error::{
    Error,
    MBQLError,
};
use std::collections::HashMap;

use pest::Parser;
use pest_derive;

#[derive(Parser)]
#[grammar = "mbql.pest"]
pub struct MBQLParser;

#[derive(Debug)]
struct Item {
    id: String,
    expression: Expression,
}

#[derive(Debug)]
enum Expression {
    DefaultAgent,
    Alledge,
    GroundSymbol
}

#[derive(Debug)]
pub struct Query {
    // items: HashMap<String,>,
}

impl Query {
    pub fn new() -> Self {
        Query { 
            //items: HashMap::new()
              }
    }

    pub fn parse<T: std::io::BufRead>(&mut self, mut reader: T) -> Result<(), Error> {

        let mut items : Vec<Item> = Vec::new();

        for (line_number, line_str) in reader.lines().enumerate() {

            let line_str = line_str?;
            let line_number = line_number + 1;

            // don't really need span on the line itself
            let mut line = MBQLParser::parse(Rule::line, &line_str).unwrap().next().unwrap().into_inner();
            println!("{:?}", line);

            let id = line.next().unwrap();
            assert_eq!(id.as_rule(), Rule::id);
            
            let exp = line.next().unwrap();
            assert_eq!(exp.as_rule(), Rule::expression);

    
            let exp = exp.into_inner().next().unwrap();
            let expression = match exp.as_rule() {
                Rule::default_agent => Expression::DefaultAgent,
                Rule::alledge => Expression::Alledge,
                Rule::ground_symbol => Expression::GroundSymbol,
                _ => unreachable!(),
            };
            items.push(Item{ id: id.as_str().to_string(), expression});

            // inner.next().unwrap().as_rule() {
            //     Rule::id
            // }
        }

        println!("{:?}", items);
        Ok(())
    }
}

// fn parse_line(pair: Pair<Rule>) ->{}
