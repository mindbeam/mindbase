use crate::error::{
    Error,
    MBQLError,
};
use regex::Regex;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum Item {
    DefaultAgent(DefaultAgent),
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultAgent;

// #[typetag::serde]
// impl Item for DefaultAgent {}

#[derive(Serialize, Deserialize, Debug)]
struct Alledge {}
#[derive(Serialize, Deserialize, Debug)]
struct BaseSymbol();

pub struct Query {
    items: HashMap<String, Item>,
}

impl Query {
    pub fn new() -> Self {
        Query { items: HashMap::new() }
    }

    pub fn parse<T: std::io::BufRead>(&mut self, mut reader: T) -> Result<(), Error> {
        for (line_number, line) in reader.lines().enumerate() {
            let re = Regex::new(r"[ \t]*:[ \t]*").unwrap();
            let line = line?;

            let parts: Vec<&str> = re.splitn(&line, 2).collect();

            println!("LOOK {:?}, {}", parts, line);
            if parts.len() != 2 {
                return Err(MBQLError::InvalidLine { line_number,
                                                    line: line.to_string() }.into());
            }

            let temp_id: &str = parts.get(0).unwrap();
            let line_body: &str = parts.get(1).unwrap();

            // HACK
            let tl_command_re = Regex::new("([A-Za-z]+)").unwrap();
            let captures = tl_command_re.captures(line_body).unwrap();

            let tl_command = match captures.get(1) {
                Some(i) => i.as_str(),
                None => {
                    return Err(MBQLError::InvalidCommand { line_number,
                                                           command: line_body.to_string() }.into())
                },
            };

            let item = match tl_command {
                "DefaultAgent" => {
                    Item::DefaultAgent(ron::de::from_str(line_body).map_err(|e| MBQLError::CommandParse { line_number, ron: e })?)
                },
                _ => {
                    return Err(MBQLError::UnknownCommand { line_number,
                                                           command: line_body.to_string() }.into())
                },
            };
            println!("Item: {:?}", item);
        }
        Ok(())
    }
}
