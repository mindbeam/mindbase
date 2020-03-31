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
        for (line_number, line) in reader.lines().enumerate() {

            let line = line?;
            let line_number = line_number + 1;

            let successful_parse = MBQLParser::parse(Rule::line, &line);
            println!("{:?}", successful_parse);

        //     let re = Regex::new(r"[ \t]*:[ \t]*").unwrap();

        //     let parts: Vec<&str> = re.splitn(&line, 2).collect();

        //     if parts.len() != 2 {
        //         return Err(MBQLError::InvalidLine { line_number,
        //                                             line: line.to_string() }.into());
        //     }

        //     let temp_id: &str = parts.get(0).unwrap();
        //     let line_body: &str = parts.get(1).unwrap();

        //     // HACK
        //     let tl_command_re = Regex::new("([A-Za-z]+)").unwrap();
        //     let captures = tl_command_re.captures(line_body).unwrap();

        //     let tl_command = match captures.get(1) {
        //         Some(i) => i.as_str(),
        //         None => {
        //             return Err(MBQLError::InvalidCommand { line_number,
        //                                                    command: line_body.to_string() }.into())
        //         },
        //     };

        //     let item = match tl_command {
        //         "DefaultAgent" => Item::DefaultAgent,
        //         "Alledge" => {
        //             Item::Alledge(ron::de::from_str(line_body).map_err(|e| {
        //                                                           MBQLError::CommandParse { line_number,
        //                                                                                     ron: e,
        //                                                                                     body: line_body.to_string() }
        //                                                       })?)
        //         },
        //         "GroundSymbol" => {
        //             Item::GroundSymbol(ron::de::from_str(line_body).map_err(|e| {
        //                                                                MBQLError::CommandParse { line_number,
        //                                                                                          ron: e,
        //                                                                                          body: line_body.to_string() }
        //                                                            })?)
        //         },
        //         _ => {
        //             return Err(MBQLError::UnknownCommand { line_number,
        //                                                    command: line_body.to_string() }.into())
        //         },
        //     };
        //     println!("Item: {:?}", item);
        }
        Ok(())
    }
}
