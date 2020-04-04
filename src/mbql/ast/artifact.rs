use super::symbol::*;

#[derive(Debug)]
pub enum Artifact {
    Agent(crate::agent::AgentId),
    Url(Url),
    FlatText(Text),
    DataNode(DataNode),
    DataGraph(DataGraph),
    DataNodeRelation(DataNodeRelation),
}

#[derive(Debug)]
pub struct Url {
    pub url: String,
}

#[derive(Debug)]
pub struct Text {
    text: String,
}

#[derive(Debug)]
pub struct DataNode {
    pub node_type: Box<Symbol>,
    pub data:      Vec<u8>,
    pub relations: Vec<DataNodeRelation>,
}

#[derive(Debug)]
pub struct DataGraph {
    pub graph_type: Box<Symbol>,
    pub bytes:      u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes:      Vec<Symbol>,
}

#[derive(Debug)]
pub struct DataNodeRelation {}
