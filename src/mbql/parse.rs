use super::{
    ast,
    error::*,
};

use pest::{
    iterators::Pair,
    Parser,
};

#[derive(Parser)]
#[grammar = "mbql/mbql.pest"]
pub struct MBQLParser;

pub fn parse<T: std::io::BufRead>(reader: T, query: &mut super::Query) -> Result<(), Error> {
    for (line_number, line) in reader.lines().enumerate() {
        let line_str: String = line.map_err(|error| {
                                       Error { position: Position { row: line_number },
                                               kind:     ErrorKind::IOError { error }, }
                                   })?;
        parse_line(line_number + 1, &line_str, query)?;
    }

    Ok(())
}

fn parse_line(row: usize, input: &str, query: &mut super::Query) -> Result<(), Error> {
    let mut line = MBQLParser::parse(Rule::statement, &input).map_err(|pest_err| {
                                                                 Error { position: Position { row },
                                                                         kind:     ErrorKind::ParseRow { input:
                                                                                                             input.to_string(),
                                                                                                         pest_err }, }
                                                             })?;

    let inner = match line.next() {
        None => return Ok(()), // Comment or blank line
        Some(s) => s,
    };

    match inner.as_rule() {
        Rule::EOI => return Ok(()), // Comment or blank line
        Rule::artifactstatement => {
            ast::ArtifactStatement::parse(inner, query);
            // println!("artifact {}", inner);
        },
        Rule::symbolstatement => {
            // println!("symbol {}", inner);
        },
        _ => {
            println!("{}", inner);
            unreachable!();
        },
    }

    // // Ok to use unwrap with these, as they shouldn't vary
    // let id = line.next().unwrap();
    // assert_eq!(id.as_rule(), Rule::id);

    // let exp = line.next().unwrap();
    // assert_eq!(exp.as_rule(), Rule::expression);

    // let pair = exp.into_inner().next().unwrap();
    // // println!("{}: {:?}", id.as_str(), exp);

    // let expression = match pair.as_rule() {
    //     Rule::agent => ast::Expression::Agent(ast::Agent::parse(pair)?),
    //     Rule::alledge => ast::Expression::Alledge(ast::Alledge::parse(pair)?),
    //     Rule::ground => ast::Expression::GroundSymbol(ast::GroundSymbol::parse(pair)?),
    //     _ => unreachable!(),
    // };

    Ok(())
}

// trait Parse {
//     fn parse(pair: Pair<Rule>) -> Result<Self, Error>;
// }

impl ast::Variable {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        Ok(Self(pair.as_str()[1..].to_string()))
    }
}

impl ast::FlatText {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        let pair = pair.into_inner().next().unwrap();
        Ok(Self(pair.as_str().replace("\\\"", "\"")))
    }
}

// impl ast::Alledge {
//     fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
//         println!("{:?}", pair);
//         let thing_pair = pair.into_inner().next().unwrap();

//         unimplemented!()
//         // let thing = match thing_pair.as_rule() {
//         //     Rule::variable => ast::AlledgeThing::Variable(ast::Variable::parse(thing_pair)?), // skip the leading colon
//         //     Rule::string => ast::AlledgeThing::FlatText(ast::FlatText::parse(thing_pair)?),
//         //     Rule::agent => ast::AlledgeThing::Agent(ast::Agent::parse(thing_pair)?),
//         //     _ => unreachable!(),
//         // };

//         // let categorize = None;

//         // Ok(ast::Alledge { thing, categorize })
//     }
// }

impl ast::Agent {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        // let pair = pair.into_inner().next().unwrap();
        Ok(ast::Agent(pair.as_str().to_string()))
    }
}
impl ast::GroundSymbol {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        Ok(ast::GroundSymbol)
        // (exp.into_inner().next().unwrap().as_str().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::{
        MBQLParser,
        Rule,
    };
    use pest::{
        consumes_to,
        parses_to,
    };
    #[test]
    fn pest_basic() {
        parses_to! {
            parser: MBQLParser,
            input:  "@url : Url(\"test\")",
            rule:   Rule::artifactstatement,
            tokens: [artifactstatement(0,18,[
                artifactvar(0,4,[ literal(1,4)]),
                artifact(7,18, [url(7,18,[quoted_string(11,17,[string(12,16)])])])
            ])]
        }

        // TODO add more pest tests
    }
}
