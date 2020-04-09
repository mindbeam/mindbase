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
    println!("LINE {}", row);
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
            ast::ArtifactStatement::parse(inner, query)?;
            // println!("artifact {}", inner);
        },
        Rule::symbolstatement => {
            ast::SymbolStatement::parse(inner, query)?;
            // println!("symbol {}", inner);
        },
        _ => {
            println!("{}", inner);
            unreachable!();
        },
    }
    Ok(())
}

// trait Parse {
//     fn parse(pair: Pair<Rule>) -> Result<Self, Error>;
// }

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
