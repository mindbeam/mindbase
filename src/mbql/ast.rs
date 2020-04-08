pub mod artifact;

use super::parse;
use parse::Rule;
use pest::iterators::{
    Pair,
    Pairs,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct ArtifactVar {
    pub var: String,
}

impl Display for ArtifactVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.var)
    }
}

impl ArtifactVar {
    pub fn parse(pair: Pair<parse::Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::artifactvar);
        Self { var: pair.into_inner().next().unwrap().as_str().to_string(), }
    }
}

#[derive(Debug)]
pub struct ArtifactStatement {
    pub var:      ArtifactVar,
    pub artifact: Artifact,
}

impl ArtifactStatement {
    pub fn parse(pair: Pair<Rule>, query: &mut crate::mbql::Query) -> Result<(), crate::mbql::error::Error> {
        assert_eq!(pair.as_rule(), Rule::artifactstatement);

        let mut pairs = pair.into_inner();
        let var = ArtifactVar::parse(pairs.next().unwrap());

        let artifact = Artifact::parse(pairs.next().unwrap());

        let me = ArtifactStatement { var, artifact };

        query.add_artifact_statement(me);

        Ok(())
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), crate::error::Error> {
        writer.write(format!("@{} = ", self.var.var).as_bytes())?;
        self.artifact.write(writer, true)?;
        writer.write(b"\n")?;
        Ok(())
    }
}
impl Display for ArtifactStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t{}", self.var, self.artifact)
    }
}

#[derive(Debug)]
pub struct SymbolVar {
    var: String,
}
pub struct SymbolStatement {
    pub var:    Option<SymbolVar>,
    pub symbol: Symbolizable,
}

impl SymbolVar {
    pub fn parse(pair: Pair<parse::Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::symbolvar);
        Self { var: pair.into_inner().next().unwrap().as_str().to_string(), }
    }
}

impl Display for SymbolVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.var)
    }
}

impl Display for SymbolStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
        // match &self.var {
        //     Some(var) => write!(f, "{}:\t{}", var, self.symbol),
        //     None => write!(f, "\t{}", self.symbol),
        // }
    }
}

#[derive(Debug)]
pub struct Variable(pub(crate) String);

#[derive(Debug)]
pub struct FlatText(pub(crate) String);

#[derive(Debug)]
pub struct Category {}

#[derive(Debug)]
pub struct Agent(pub(crate) String);

#[derive(Debug)]
pub struct GroundSymbol;

#[derive(Debug)]
pub enum Symbolizable {
    Artifact(Artifact),
    SymbolPair {
        left:  Box<Symbolizable>,
        right: Box<Symbolizable>,
    }, // (Alledge/ Analogy)

    // TODO 1 - determine if we want to flatten/variablize/pointerize the tree as we parse it
    // or if we flatten that structure at a later phase?
    SymbolVar(SymbolVar),
    Ground,
    Symbolize,
}

impl Symbolizable {
    pub fn parse(pair: Pair<parse::Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::symbolizable);
        let element = pair.into_inner().next().unwrap();

        match element.as_rule() {
            Rule::artifact => unimplemented!(),
            Rule::symbolvar => Symbolizable::SymbolVar(SymbolVar::parse(element)),
            Rule::ground => unimplemented!(),
            Rule::symbolize => unimplemented!(),
            Rule::alledge => unimplemented!(),
            Rule::symbol_pair => unimplemented!(),
            _ => unreachable!(),
        }
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), crate::error::Error> {
        match self {
            Symbolizable::Artifact(a) => unimplemented!(),
            Symbolizable::SymbolPair { left, right } => unimplemented!(),
            Symbolizable::SymbolVar(sv) => unimplemented!(),
            Symbolizable::Ground => unimplemented!(),
            Symbolizable::Symbolize => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolPair {
    pub(crate) thing:      Symbolizable,
    pub(crate) categorize: Symbolizable,
}

#[derive(Debug)]
pub enum Artifact {
    Agent(Agent),
    Url(Url),
    Text(Text),
    DataNode(DataNode),
    DataGraph(DataGraph),
    DataNodeRelation(DataNodeRelation),
}

impl Artifact {
    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), crate::error::Error> {
        match self {
            Artifact::Agent(agent) => agent.write(writer)?,
            Artifact::Url(url) => unimplemented!(),
            Artifact::Text(text) => text.write(writer, verbose)?,
            Artifact::DataNode(datanode) => datanode.write(writer)?,
            _ => unimplemented!(),
        }
        Ok(())
    }

    pub fn parse(pair: Pair<parse::Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::artifact);
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::artifactvar => unimplemented!(),
            Rule::agent => {
                let agent_ident = inner.into_inner().next().unwrap();
                Artifact::Text(Text { text: agent_ident.as_str().to_string(), })
            },
            Rule::datagraph => unimplemented!(),
            Rule::datanode => {
                let mut inner = inner.into_inner();
                let data_type = Symbolizable::parse(inner.next().unwrap());
                let b64 = inner.next().unwrap();
                assert_eq!(b64.as_rule(), Rule::base64);
                let data = base64::decode(b64.as_str()).unwrap();

                Artifact::DataNode(DataNode { data_type: Box::new(data_type),
                                              data,
                                              relations: Vec::new() })
                // TODO 1 - handle relations. Do we want to get rid of that field in favor of DataRelation?
            },
            Rule::datarelation => unimplemented!(),
            Rule::text => {
                let qs = inner.into_inner().next().unwrap();
                let s = qs.into_inner().next().unwrap();

                Artifact::Text(Text { text: s.as_str().to_string(), })
            },
            Rule::url => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

impl Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Agent(agent_id) => write!()
            _ => unimplemented!(),
        }
    }
}

impl Agent {
    pub fn write<T: std::io::Write>(&self, mut writer: T) -> Result<(), crate::error::Error> {
        writer.write(format!("Agent({})", self.0).as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Url {
    pub url: String,
}

impl Url {
    pub fn render(&self) -> String {
        unimplemented!()
        // self.url
    }
}

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn write<T: std::io::Write>(&self, writer: &mut T, verbose: bool) -> Result<(), crate::error::Error> {
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
    pub data:      Vec<u8>,
    pub relations: Vec<DataNodeRelation>,
}

impl DataNode {
    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), crate::error::Error> {
        writer.write(b"DataNode(");
        self.data_type.write(writer)?;
        writer.write(b"; ")?;

        {
            let mut enc = base64::write::EncoderWriter::new(writer, base64::STANDARD);
            use std::io::Write;
            enc.write_all(&self.data)?;
        }
        writer.write(b")")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct DataGraph {
    pub graph_type: Box<Symbolizable>,
    pub bytes:      u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes:      Vec<Symbolizable>,
}

#[derive(Debug)]
pub struct DataNodeRelation {}
