pub mod artifact;

use super::parse;
use crate::mbql::error::Error;
use parse::Rule;
use pest::iterators::Pair;

// trait ParseWrite {
//     fn parse(pair: Pair<parse::Rule>) -> Self;
//     fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error>;
// }

#[derive(Debug)]
pub struct ArtifactVar {
    pub var: String,
}

impl ArtifactVar {
    fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::artifactvar);
        Ok(Self { var: pair.into_inner().next().unwrap().as_str().to_string(), })
    }

    fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(format!("@{}", self.var).as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SymbolVar {
    var: String,
}

impl SymbolVar {
    pub fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::symbolvar);
        Ok(Self { var: pair.into_inner().next().unwrap().as_str().to_string(), })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, _verbose: bool) -> Result<(), std::io::Error> {
        writer.write(format!("${}", self.var).as_bytes())?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        self.var.clone()
    }
}

pub struct ArtifactStatement {
    pub var:      ArtifactVar,
    pub artifact: Artifact,
}

impl ArtifactStatement {
    pub fn parse(pair: Pair<Rule>, query: &mut crate::mbql::Query) -> Result<(), Error> {
        assert_eq!(pair.as_rule(), Rule::artifactstatement);

        let mut pairs = pair.into_inner();
        let var = ArtifactVar::parse(pairs.next().unwrap())?;

        let artifact = Artifact::parse(pairs.next().unwrap())?;

        let me = ArtifactStatement { var, artifact };

        query.add_artifact_statement(me);

        Ok(())
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        self.var.write(writer)?;
        writer.write(b" = ")?;
        self.artifact.write(writer, true)?;
        writer.write(b"\n")?;
        Ok(())
    }
}

pub struct SymbolStatement {
    pub var:    Option<SymbolVar>,
    pub symbol: Symbolizable,
}

impl SymbolStatement {
    pub fn parse(pair: Pair<Rule>, query: &mut crate::mbql::Query) -> Result<(), Error> {
        assert_eq!(pair.as_rule(), Rule::symbolstatement);

        let mut pairs = pair.into_inner();

        let next = pairs.next().unwrap();

        let (var, next) = if let Rule::symbolvar = next.as_rule() {
            (Some(SymbolVar::parse(next)?), pairs.next().unwrap())
        } else {
            (None, next)
        };

        // based on the grammar, we are guaranteed to have allege | ground | symbolize
        let symbol = match next.as_rule() {
            Rule::allege => Symbolizable::Allege(Allege::parse(next)?),
            Rule::ground => Symbolizable::Ground(Ground::parse(next)?),
            Rule::symbolize => Symbolizable::Symbolize(Symbolize::parse(next)?),
            _ => unreachable!(),
        };

        let me = SymbolStatement { var, symbol };

        query.add_symbol_statement(me);

        Ok(())
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        if let Some(var) = &self.var {
            var.write(writer, false)?;
            writer.write(b" = ")?;
        }
        self.symbol.write(writer, true, false)?;
        writer.write(b"\n")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Ground(Box<Symbolizable>);

impl Ground {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::ground);
        Ok(Ground(Box::new(Symbolizable::parse(pair.into_inner().next().unwrap())?)))
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(b"Ground(")?;
            self.0.write(writer, false, false)?;
            writer.write(b")")?;
        } else {
            writer.write(b"{")?;
            self.0.write(writer, false, false)?;
            writer.write(b"}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Allege {
    left:  Box<Symbolizable>,
    right: Box<Symbolizable>,
}

impl Allege {
    pub fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::allege);

        let mut symbol_pair = pair.into_inner().next().unwrap().into_inner();

        // According to the grammar, Allege may only contain symbol_pair
        let left = Symbolizable::parse(symbol_pair.next().unwrap())?;
        let right = Symbolizable::parse(symbol_pair.next().unwrap())?;

        Ok(Allege { left:  Box::new(left),
                    right: Box::new(right), })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool, nest: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(b"Allege(")?;
        } else if nest {
            writer.write(b"(")?;
        }

        self.left.write(writer, false, true)?;
        writer.write(b" : ")?;
        self.right.write(writer, false, true)?;

        if verbose {
            writer.write(b")")?;
        } else if nest {
            writer.write(b")")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Symbolize(Box<Symbolizable>);
impl Symbolize {
    pub fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::symbolize);
        Ok(Symbolize(Box::new(Symbolizable::parse(pair.into_inner().next().unwrap())?)))
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(b"Symbolize(")?;
        }

        self.0.write(writer, false, false)?;

        if verbose {
            writer.write(b")")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Symbolizable {
    Artifact(Artifact),
    Allege(Allege),
    // TODO 1 - determine if we want to flatten/variablize/pointerize the tree as we parse it
    // or if we flatten that structure at a later phase?
    SymbolVar(SymbolVar),
    Ground(Ground),
    Symbolize(Symbolize),
}

impl Symbolizable {
    pub fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        // because of left-recursion issues, we had to construct symbolizable in a slightly odd way
        // which necessitates allege and ground to support symbol_pair AND symbolizable as potential child elements
        // So we are handling symbol_pair if they were symbolizable
        let s = match pair.as_rule() {
            Rule::symbol_pair => {
                let mut inner = pair.into_inner();
                let left = Symbolizable::parse(inner.next().unwrap())?;
                let right = Symbolizable::parse(inner.next().unwrap())?;
                Symbolizable::Allege(Allege { left:  Box::new(left),
                                              right: Box::new(right), })
            },
            Rule::symbolizable => {
                let element = pair.into_inner().next().unwrap();

                match element.as_rule() {
                    Rule::artifact => Symbolizable::Artifact(Artifact::parse(element)?),
                    Rule::symbolvar => Symbolizable::SymbolVar(SymbolVar::parse(element)?),
                    Rule::ground => Symbolizable::Ground(Ground::parse(element)?),
                    Rule::symbolize => Symbolizable::Symbolize(Symbolize::parse(element)?),
                    Rule::allege => Symbolizable::Allege(Allege::parse(element)?),
                    Rule::symbol_pair => {
                        let mut inner = element.into_inner();
                        let left = Symbolizable::parse(inner.next().unwrap())?;
                        let right = Symbolizable::parse(inner.next().unwrap())?;
                        Symbolizable::Allege(Allege { left:  Box::new(left),
                                                      right: Box::new(right), })
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        };

        Ok(s)
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool, nest: bool) -> Result<(), std::io::Error> {
        match self {
            Symbolizable::Artifact(a) => a.write(writer, verbose)?,
            Symbolizable::Allege(a) => a.write(writer, verbose, nest)?,
            Symbolizable::SymbolVar(sv) => sv.write(writer, verbose)?,
            Symbolizable::Ground(g) => g.write(writer, verbose)?,
            Symbolizable::Symbolize(s) => s.write(writer, verbose)?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Artifact {
    Agent(Agent),
    Url(Url),
    Text(Text),
    DataNode(DataNode),
    DataRelation(DataRelation),
    ArtifactVar(ArtifactVar),
}

impl Artifact {
    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        match self {
            Artifact::Agent(agent) => agent.write(writer)?,
            Artifact::Url(url) => url.write(writer, false)?,
            Artifact::Text(text) => text.write(writer, verbose)?,
            Artifact::DataNode(node) => node.write(writer)?,
            Artifact::DataRelation(relation) => relation.write(writer)?,
            Artifact::ArtifactVar(var) => var.write(writer)?,
        }
        Ok(())
    }

    pub fn parse(pair: Pair<parse::Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::artifact);
        let child = pair.into_inner().next().unwrap();

        let a = match child.as_rule() {
            Rule::artifactvar => Artifact::ArtifactVar(ArtifactVar::parse(child)?),
            Rule::agent => Artifact::Agent(Agent::parse(child)?),
            Rule::datanode => Artifact::DataNode(DataNode::parse(child)?),
            Rule::datarelation => Artifact::DataRelation(DataRelation::parse(child)?),
            Rule::text => Artifact::Text(Text::parse(child)?),
            Rule::url => Artifact::Url(Url::parse(child)?),
            _ => unreachable!(),
        };

        Ok(a)
    }
}

#[derive(Debug)]
pub struct Agent(pub(crate) String);
impl Agent {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        assert_eq!(pair.as_rule(), Rule::agent);
        Ok(Agent(pair.into_inner().next().unwrap().as_str().to_string()))
    }

    pub fn write<T: std::io::Write>(&self, mut writer: T) -> Result<(), std::io::Error> {
        writer.write(format!("Agent({})", self.0).as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Url {
    pub url: String,
}

impl Url {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        let pair = pair.into_inner().next().unwrap();
        Ok(Self { url: pair.as_str().replace("\\\"", "\""), })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, _verbose: bool) -> Result<(), std::io::Error> {
        writer.write(format!("Url(\"{}\")", self.url.replace("\"", "\\\"")).as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl Text {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        let qs = pair.into_inner().next().unwrap();
        let s = qs.into_inner().next().unwrap();

        Ok(Text { text: s.as_str().to_string().replace("\\\"", "\""), })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(format!("Text(\"{}\")", self.text.replace("\"", "\\\"")).as_bytes())?;
        } else {
            writer.write(format!("\"{}\"", self.text.replace("\"", "\\\"")).as_bytes())?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DataNode {
    pub data_type: Box<Symbolizable>,
    pub data:      Option<Vec<u8>>,
}

impl DataNode {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let data_type = Symbolizable::parse(inner.next().unwrap())?;

        let data = match inner.next() {
            Some(next) => {
                match next.as_rule() {
                    Rule::base64 => Some(base64::decode(next.as_str()).unwrap()),
                    Rule::quoted_string => Some(next.as_str().replace("\\\"", "\"").as_bytes().to_owned()),
                    _ => unreachable!(),
                }
            },
            None => None,
        };

        Ok(DataNode { data_type: Box::new(data_type),
                      data })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(b"DataNode(")?;
        self.data_type.write(writer, false, false)?;

        if let Some(data) = &self.data {
            writer.write(b";")?;
            let mut enc = base64::write::EncoderWriter::new(writer, base64::STANDARD);
            use std::io::Write;
            enc.write_all(data)?;
        }
        writer.write(b")")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct DataRelation {
    pub relation_type: Box<Symbolizable>,
    pub from:          Box<Symbolizable>,
    pub to:            Box<Symbolizable>,
}

impl DataRelation {
    fn parse(pair: Pair<Rule>) -> Result<Self, Error> {
        let mut inner = pair.into_inner();

        let relation_type = Symbolizable::parse(inner.next().unwrap())?;
        let from = Symbolizable::parse(inner.next().unwrap())?;
        let to = Symbolizable::parse(inner.next().unwrap())?;

        Ok(DataRelation { relation_type: Box::new(relation_type),
                          from:          Box::new(from),
                          to:            Box::new(to), })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(b"DataRelation(")?;
        self.relation_type.write(writer, false, false)?;
        writer.write(b";")?;

        self.from.write(writer, false, false)?;
        writer.write(b" > ")?;

        self.to.write(writer, false, false)?;
        writer.write(b")")?;

        Ok(())
    }
}
