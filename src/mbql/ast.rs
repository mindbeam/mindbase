pub mod artifact;

use crate::{
    mbql::{
        error::MBQLError,
        parse::{
            self,
            Rule,
        },
        Position,
        Query,
    },
    AgentId,
    Analogy,
    ArtifactId,
    Concept,
    MBError,
    MindBase,
};

use super::error::MBQLErrorKind;
use pest::iterators::Pair;

pub enum Statement {
    Diag(DiagStatement),
    Symbol(SymbolStatement),
    Artifact(ArtifactStatement),
}

impl Statement {
    pub fn parse(element: Pair<Rule>, query: &mut crate::mbql::Query, position: Position) -> Result<(), MBQLError> {
        let me = match element.as_rule() {
            Rule::EOI => return Ok(()), // Comment or blank line
            Rule::artifactstatement => Statement::Artifact(ArtifactStatement::parse(element, position)?),
            Rule::symbolstatement => Statement::Symbol(SymbolStatement::parse(element, position)?),
            Rule::diagstatement => Statement::Diag(DiagStatement::parse(element, position)?),
            _ => {
                panic!("Invalid parse element {}", element);
            },
        };

        query.add_statement(me);

        Ok(())
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        match self {
            Statement::Artifact(s) => s.write(writer)?,
            Statement::Symbol(s) => s.write(writer)?,
            Statement::Diag(s) => s.write(writer)?,
        }
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<(), MBQLError> {
        match self {
            Statement::Artifact(s) => {
                // Ignore this artifact_id because it's being stored inside the apply.
                // We have to do this because it's possible to have artifacts/symbols that recursively reference artifact
                // variables
                s.apply(query)?;
            },
            Statement::Symbol(s) => {
                // Ignore this symbol because it's being stored inside the apply.
                // We have to do this because it's possible to have artifacts/symbols that recursively reference symbol variables
                s.apply(query)?;
            },
            Statement::Diag(s) => s.apply(query)?,
        }
        Ok(())
    }
}

pub struct DiagStatement {
    #[allow(unused)]
    position: Position,
    diag:     Diag,
}

struct Diag {
    elements: Vec<DiagElement>,
}

enum DiagElement {
    ArtifactVar(ArtifactVar),
    SymbolVar(SymbolVar),
}

impl DiagStatement {
    pub fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::diagstatement);

        let mut items = pair.into_inner();
        let mut elements = Vec::new();
        while let Some(d) = items.next() {
            let e = match d.as_rule() {
                Rule::artifactvar => DiagElement::ArtifactVar(ArtifactVar::parse(d, position.clone())?),
                Rule::symbolvar => DiagElement::SymbolVar(SymbolVar::parse(d, position.clone())?),
                _ => {
                    println!("{:?}", d);
                    unreachable!()
                },
            };

            elements.push(e)
        }

        Ok(DiagStatement { position,
                           diag: Diag { elements } })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(b"Diag(")?;
        let mut seen = false;
        for item in self.diag.elements.iter() {
            if seen {
                writer.write(b", ")?;
            }
            seen = true;

            item.write(writer)?;
        }
        writer.write(b")\n")?;
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<(), MBQLError> {
        let mut out = String::new();
        let mut seen = false;
        for item in self.diag.elements.iter() {
            if seen {
                out.push_str(", ");
            }
            seen = true;

            match item {
                DiagElement::ArtifactVar(var) => {
                    if let Some(artifact_id) = query.get_artifact_var(&var.var)? {
                        out.push_str(&format!("{} = {}", var, artifact_id));
                    } else {
                        return Err(MBQLError { position: var.position.clone(),
                                               kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, });
                    }
                },
                DiagElement::SymbolVar(v) => {
                    if let Some(symbol) = query.get_symbol_var(&v.var)? {
                        out.push_str(&format!("{} = {}", v, symbol));
                    } else {
                        return Err(MBQLError { position: v.position.clone(),
                                               kind:     MBQLErrorKind::SymbolVarNotFound { var: v.var.clone() }, });
                    }
                },
            }
        }
        println!("DIAG: {}", out);
        Ok(())
    }
}

impl DiagElement {
    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        match self {
            DiagElement::ArtifactVar(v) => v.write(writer)?,
            DiagElement::SymbolVar(v) => v.write(writer)?,
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct ArtifactVar {
    pub var:      String,
    pub position: Position,
}

impl ArtifactVar {
    fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::artifactvar);
        Ok(Self { var: pair.into_inner().next().unwrap().as_str().to_string(),
                  position })
    }

    fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(format!("@{}", self.var).as_bytes())?;
        Ok(())
    }
}

impl std::fmt::Display for ArtifactVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.var)
    }
}

#[derive(Debug)]
pub struct SymbolVar {
    pub var:      String,
    pub position: Position,
}

impl SymbolVar {
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::symbolvar);
        Ok(Self { var: pair.into_inner().next().unwrap().as_str().to_string(),
                  position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writer.write(format!("${}", self.var).as_bytes())?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        self.var.clone()
    }

    pub fn apply(&self, _query: &Query) -> Result<Concept, MBQLError> {
        unimplemented!()
    }
}

impl std::fmt::Display for SymbolVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.var)
    }
}
#[derive(Debug)]
pub struct ArtifactStatement {
    pub var:      ArtifactVar,
    pub artifact: Artifact,
    pub position: Position,
}

impl ArtifactStatement {
    pub fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::artifactstatement);

        let mut pairs = pair.into_inner();
        let var = ArtifactVar::parse(pairs.next().unwrap(), position.clone())?;

        let artifact = Artifact::parse(pairs.next().unwrap(), position.clone())?;

        Ok(ArtifactStatement { var, artifact, position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        self.var.write(writer)?;
        writer.write(b" = ")?;
        self.artifact.write(writer, true)?;
        writer.write(b"\n")?;
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<ArtifactId, MBQLError> {
        let artifact_id = self.artifact.apply(query)?;
        query.store_artifact_for_var(&self.var, artifact_id.clone())?;
        Ok(artifact_id)
    }
}

#[derive(Debug)]
pub struct SymbolStatement {
    pub var:    Option<SymbolVar>,
    pub symbol: Symbolizable,
}

impl SymbolStatement {
    pub fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::symbolstatement);

        let mut pairs = pair.into_inner();

        let next = pairs.next().unwrap();

        let (var, next) = if let Rule::symbolvar = next.as_rule() {
            (Some(SymbolVar::parse(next, position.clone())?), pairs.next().unwrap())
        } else {
            (None, next)
        };

        // based on the grammar, we are guaranteed to have allege | ground | symbolize
        let symbol = match next.as_rule() {
            Rule::allege => Symbolizable::Allege(Allege::parse(next, position.clone())?),
            Rule::ground => Symbolizable::Ground(Ground::parse(next, position.clone())?),
            Rule::symbolize => Symbolizable::Symbolize(Symbolize::parse(next, position.clone())?),
            _ => unreachable!(),
        };

        Ok(SymbolStatement { var, symbol })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        if let Some(var) = &self.var {
            var.write(writer)?;
            writer.write(b" = ")?;
        }
        self.symbol.write(writer, true, false)?;
        writer.write(b"\n")?;
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
        let symbol = self.symbol.apply(query)?;

        if let Some(var) = &self.var {
            query.store_symbol_for_var(var, symbol.clone())?;
        }

        Ok(symbol)
    }
}

#[derive(Debug)]
pub struct Ground {
    symbolizable: Box<GSymbolizable>,
    position:     Position,
}

impl Ground {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::ground);

        Ok(Ground { symbolizable: Box::new(GSymbolizable::parse(pair.into_inner().next().unwrap(), position.clone())?),
                    position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(b"Ground(")?;
            self.symbolizable.write(writer, false, false)?;
            writer.write(b")")?;
        } else {
            writer.write(b"{")?;
            self.symbolizable.write(writer, false, false)?;
            writer.write(b"}")?;
        }
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
        let concept = query.gscontext.lock().unwrap().symbolize(&*self.symbolizable, query)?;
        Ok(concept)
    }
}

#[derive(Debug)]
pub struct Allege {
    left:  Box<Symbolizable>,
    right: Box<Symbolizable>,
}

impl Allege {
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::allege);

        let mut symbol_pair = pair.into_inner().next().unwrap().into_inner();

        // According to the grammar, Allege may only contain symbol_pair
        let left = Symbolizable::parse(symbol_pair.next().unwrap(), position.clone())?;
        let right = Symbolizable::parse(symbol_pair.next().unwrap(), position.clone())?;

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

    pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
        let left = self.left.apply(query)?;
        let right = self.right.apply(query)?;

        let symbol = query.mb.symbolize(Analogy::declarative(left, right))?;
        Ok(symbol)
    }
}

#[derive(Debug)]
pub struct Symbolize {
    symbolizable: Box<Symbolizable>,
    position:     Position,
}
impl Symbolize {
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::symbolize);
        Ok(Symbolize { symbolizable: Box::new(Symbolizable::parse(pair.into_inner().next().unwrap(), position.clone())?),
                       position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), std::io::Error> {
        if verbose {
            writer.write(b"Symbolize(")?;
        }

        self.symbolizable.write(writer, false, false)?;

        if verbose {
            writer.write(b")")?;
        }
        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
        self.symbolizable.apply(query)
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
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        // because of left-recursion issues, we had to construct symbolizable in a slightly odd way
        // which necessitates allege and ground to support symbol_pair AND symbolizable as potential child elements
        // So we are handling symbol_pair if they were symbolizable
        let s = match pair.as_rule() {
            Rule::symbol_pair => {
                let mut inner = pair.into_inner();
                let left = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
                let right = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
                Symbolizable::Allege(Allege { left:  Box::new(left),
                                              right: Box::new(right), })
            },
            Rule::symbolizable => {
                let element = pair.into_inner().next().unwrap();

                match element.as_rule() {
                    Rule::artifact => Symbolizable::Artifact(Artifact::parse(element, position)?),
                    Rule::symbolvar => Symbolizable::SymbolVar(SymbolVar::parse(element, position)?),
                    Rule::ground => Symbolizable::Ground(Ground::parse(element, position)?),
                    Rule::symbolize => Symbolizable::Symbolize(Symbolize::parse(element, position)?),
                    Rule::allege => Symbolizable::Allege(Allege::parse(element, position)?),
                    Rule::symbol_pair => {
                        let mut inner = element.into_inner();
                        let left = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
                        let right = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
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
            Symbolizable::SymbolVar(sv) => sv.write(writer)?,
            Symbolizable::Ground(g) => g.write(writer, verbose)?,
            Symbolizable::Symbolize(s) => s.write(writer, verbose)?,
        }

        Ok(())
    }

    pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
        let symbol = match self {
            Symbolizable::Artifact(a) => {
                let artifact_id = a.apply(query)?;
                println!("SYMBOLIZE: {}", artifact_id);
                query.mb.symbolize(artifact_id)?
            },
            Symbolizable::Allege(a) => a.apply(query)?,
            // Symbolizable::SymbolVar(sv) => sv.apply(query),
            Symbolizable::Ground(g) => g.apply(query)?,
            Symbolizable::Symbolize(s) => s.apply(query)?,
            _ => unimplemented!(),
        };

        Ok(symbol)
    }
}

#[derive(Debug)]
pub enum GSymbolizable {
    Artifact(Artifact),
    SymbolVar(SymbolVar),
    Ground(Ground),
    GroundPair(GPair),
}

impl GSymbolizable {
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        let s = match pair.as_rule() {
            Rule::ground_symbol_pair => {
                let mut inner = pair.into_inner();
                let left = GSymbolizable::parse(inner.next().unwrap(), position.clone())?;
                let right = GSymbolizable::parse(inner.next().unwrap(), position.clone())?;
                GSymbolizable::GroundPair(GPair { left: Box::new(left),
                                                  right: Box::new(right),
                                                  position })
            },
            Rule::ground_symbolizable => {
                let element = pair.into_inner().next().unwrap();

                match element.as_rule() {
                    Rule::artifact => GSymbolizable::Artifact(Artifact::parse(element, position)?),
                    Rule::symbolvar => GSymbolizable::SymbolVar(SymbolVar::parse(element, position)?),
                    Rule::ground => GSymbolizable::Ground(Ground::parse(element, position)?),
                    Rule::ground_symbol_pair => {
                        let mut inner = element.into_inner();
                        let left = GSymbolizable::parse(inner.next().unwrap(), position.clone())?;
                        let right = GSymbolizable::parse(inner.next().unwrap(), position.clone())?;
                        GSymbolizable::GroundPair(GPair { left: Box::new(left),
                                                          right: Box::new(right),
                                                          position })
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
            GSymbolizable::Artifact(a) => a.write(writer, verbose)?,
            GSymbolizable::GroundPair(p) => p.write(writer, nest)?,
            GSymbolizable::SymbolVar(sv) => sv.write(writer)?,
            GSymbolizable::Ground(g) => g.write(writer, verbose)?,
        }

        Ok(())
    }

    // pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
    //     let symbol = match self {
    //         GroundSymbolizable::Artifact(a) => query.mb.get_ground_symbol(a)?,
    //         GroundSymbolizable::GroundPair(a) => a.apply(query),
    //         //     GroundSymbolizable::SymbolVar(sv) => sv.apply(query),
    //         //     GroundSymbolizable::Ground(g) => g.apply(query),
    //         _ => unimplemented!(),
    //     };
    //     Ok(symbol)
    // }
}

// impl GroundSymbolize for GroundSymbolizable {
//     fn symbol(&self) -> Option<Concept> {
//         None
//     }

//     fn symbolize(&self, context: &mut crate::GSContext) -> Result<Concept, crate::MBError> {
//         unimplemented!()
//     }
// }

#[derive(Debug)]
pub struct GPair {
    pub left:  Box<GSymbolizable>,
    pub right: Box<GSymbolizable>,
    position:  Position,
}

impl GPair {
    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::allege);

        let mut ground_symbol_pair = pair.into_inner().next().unwrap().into_inner();

        // According to the grammar, Allege may only contain symbol_pair
        let left = GSymbolizable::parse(ground_symbol_pair.next().unwrap(), position.clone())?;
        let right = GSymbolizable::parse(ground_symbol_pair.next().unwrap(), position.clone())?;

        Ok(GPair { left: Box::new(left),
                   right: Box::new(right),
                   position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, nest: bool) -> Result<(), std::io::Error> {
        if nest {
            writer.write(b"(")?;
        }

        self.left.write(writer, false, true)?;
        writer.write(b" : ")?;
        self.right.write(writer, false, true)?;

        if nest {
            writer.write(b")")?;
        }
        Ok(())
    }

    // pub fn apply(&self, query: &Query) -> Result<Concept, MBQLError> {
    //     unimplemented!()
    // }
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

    pub fn parse(pair: Pair<parse::Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::artifact);
        let child = pair.into_inner().next().unwrap();

        let a = match child.as_rule() {
            Rule::artifactvar => Artifact::ArtifactVar(ArtifactVar::parse(child, position)?),
            Rule::agent => Artifact::Agent(Agent::parse(child, position)?),
            Rule::datanode => Artifact::DataNode(DataNode::parse(child, position)?),
            Rule::datarelation => Artifact::DataRelation(DataRelation::parse(child, position)?),
            Rule::text => Artifact::Text(Text::parse(child, position)?),
            Rule::url => Artifact::Url(Url::parse(child, position)?),
            _ => unreachable!(),
        };

        Ok(a)
    }

    pub fn apply(&self, query: &Query) -> Result<ArtifactId, MBQLError> {
        let artifact_id = match self {
            Artifact::Agent(agent) => query.mb.put_artifact(agent.get_agent_id(query.mb)?)?,
            Artifact::Url(url) => query.mb.put_artifact(crate::artifact::Url { url: url.url.clone() })?,
            Artifact::Text(text) => query.mb.put_artifact(crate::artifact::Text::new(&text.text))?,
            Artifact::DataNode(node) => {
                let data_type = node.data_type.apply(query)?;
                query.mb.put_artifact(crate::artifact::DataNode { data_type,
                                                                   data: node.data.clone() })?
            },
            // Artifact::DataRelation(relation) => relation.write(writer)?,
            Artifact::ArtifactVar(var) => {
                match query.get_artifact_var(&var.var)? {
                    None => {
                        return Err(MBQLError { position: var.position.clone(),
                                               kind:     MBQLErrorKind::ArtifactVarNotFound { var: var.var.clone() }, })
                    },
                    Some(a) => a,
                }
            },
            _ => unimplemented!(),
        };

        Ok(artifact_id)
    }
}

// impl GroundSymbolize for Artifact {
//     fn symbol(&self) -> Option<Concept> {
//         None
//     }

//     fn symbolize(&self, context: &mut crate::ground::GSContext) -> Result<Concept, MBError> {
//         Ok(a.apply(query)?.subjective())
//     }
// }
// impl Into<crate::artifact::Artifact> for Agent {
//     fn into(self) -> crate::artifact::Artifact {
//         crate::artifact::Artifact::Agent(crate::agent::Agent)
//     }
// }

#[derive(Debug)]
pub struct Agent {
    pub(crate) ident: String,
    position:         Position,
}

impl Agent {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        assert_eq!(pair.as_rule(), Rule::agent);
        Ok(Agent { ident: pair.into_inner().next().unwrap().as_str().to_string(),
                   position })
    }

    pub fn write<T: std::io::Write>(&self, mut writer: T) -> Result<(), std::io::Error> {
        writer.write(format!("Agent({})", self.ident).as_bytes())?;
        Ok(())
    }

    pub fn get_agent_id(&self, mb: &MindBase) -> Result<AgentId, MBError> {
        let agent_id = if self.ident == "default" {
            mb.default_agent()?.id()
        } else {
            AgentId::from_base64(&self.ident)?
        };

        Ok(agent_id)
    }
}

#[derive(Debug)]
pub struct Url {
    pub url:      String,
    pub position: Position,
}

impl Url {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        let pair = pair.into_inner().next().unwrap();
        Ok(Self { url: pair.as_str().replace("\\\"", "\""),
                  position })
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, _verbose: bool) -> Result<(), std::io::Error> {
        writer.write(format!("Url(\"{}\")", self.url.replace("\"", "\\\"")).as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Text {
    text:     String,
    position: Position,
}

impl Text {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        let qs = pair.into_inner().next().unwrap();
        let s = qs.into_inner().next().unwrap();

        Ok(Text { text: s.as_str().to_string().replace("\\\"", "\""),
                  position })
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
    pub position:  Position,
}

impl DataNode {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        let mut inner = pair.into_inner();
        let data_type = Symbolizable::parse(inner.next().unwrap(), position.clone())?;

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
                      data,
                      position })
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
    pub position:      Position,
}

impl DataRelation {
    fn parse(pair: Pair<Rule>, position: Position) -> Result<Self, MBQLError> {
        let mut inner = pair.into_inner();

        let relation_type = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
        let from = Symbolizable::parse(inner.next().unwrap(), position.clone())?;
        let to = Symbolizable::parse(inner.next().unwrap(), position.clone())?;

        Ok(DataRelation { relation_type: Box::new(relation_type),
                          from: Box::new(from),
                          to: Box::new(to),
                          position })
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
