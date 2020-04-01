use super::{
    ast,
    error::*,
};

use pest::{
    iterators::Pair,
    Parser,
};
// use pest_derive;

#[derive(Parser)]
#[grammar = "mbql/mbql.pest"]
pub struct MBQLParser;

pub fn parse<T: std::io::BufRead>(reader: T) -> Result<Vec<ast::Item>, Error> {
    let mut items: Vec<ast::Item> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line_str: String = line.map_err(|error| {
                                       Error { position: Position::none(),
                                               kind:     ErrorKind::IOError { error }, }
                                   })?;
        let item = parse_line(line_number + 1, &line_str)?;
        items.push(item);
    }

    // println!("{:?}", items);
    Ok(items)
}

fn parse_line(row: usize, input: &str) -> Result<ast::Item, Error> {
    // don't really need span on the line itself

    let mut line = MBQLParser::parse(Rule::line, &input).map_err(|_e| {
                                                            Error { position: Position { row },
                                                                    kind:     ErrorKind::ParseRow { input: input.to_string(), }, }
                                                        })?;

    // Ok to use unwrap with these, as they shouldn't vary
    let id = line.next().unwrap();
    assert_eq!(id.as_rule(), Rule::id);

    let exp = line.next().unwrap();
    assert_eq!(exp.as_rule(), Rule::expression);

    let pair = exp.into_inner().next().unwrap();
    // println!("{}: {:?}", id.as_str(), exp);

    let expression = match pair.as_rule() {
        Rule::agent => ast::Expression::Agent(ast::Agent::parse(pair)?),
        Rule::alledge => ast::Expression::Alledge(ast::Alledge::parse(pair)?),
        Rule::ground_symbol => ast::Expression::GroundSymbol(ast::GroundSymbol::parse(pair)?),
        _ => unreachable!(),
    };

    Ok(ast::Item { key: id.as_str().to_string(),
                   expression })
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

impl ast::Alledge {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        println!("{:?}", pair);
        let thing_pair = pair.into_inner().next().unwrap();

        let thing = match thing_pair.as_rule() {
            Rule::variable => ast::AlledgeThing::Variable(ast::Variable::parse(thing_pair)?), // skip the leading colon
            Rule::string => ast::AlledgeThing::FlatText(ast::FlatText::parse(thing_pair)?),
            Rule::agent => ast::AlledgeThing::Agent(ast::Agent::parse(thing_pair)?),
            _ => unreachable!(),
        };

        let categorize = None;

        Ok(ast::Alledge { thing, categorize })
    }
}

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
