use std::{fmt::Display, unimplemented};

use mindbase_artifact::{body::DataNode, body::SubGraph, Artifact, ArtifactId, NodeInstance, NodeType};
use mindbase_hypergraph::{
    entity::{directed, vertex},
    traits::Weight,
    EntityId, HyperGraph,
};
use mindbase_store::MemoryStore;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INCREMENT: Arc<Mutex<u32>> = Arc::new(Mutex::new(0u32));
}

#[derive(Serialize, Deserialize, Debug)]
enum JSONType {
    Document,
    Null,
    Bool,
    Number,
    String,
    Array,
    ArrayMember,
    ArrayOffset(usize),
    ArrNextMember,
    ArrPrevMember,
    ArrHead,
    ArrTail,
    Object,
    ObjectProperty(String),
    ObjectProperties,
    ObjectMembers,
    Value,
    RootElement,
}

impl Weight for JSONType {
    fn get_bytes(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        encoded
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl NodeType for JSONType {}
#[derive(Serialize, Deserialize, Debug)]
enum JSONWeight {
    Artifact(Artifact<JSONType>),
    Type(JSONType),
}
impl Weight for JSONWeight {
    fn get_bytes(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        encoded
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl<T> From<T> for JSONWeight
where
    T: Into<Artifact<JSONType>>,
{
    fn from(artifact: T) -> Self {
        JSONWeight::Artifact(artifact.into())
    }
}

impl Into<JSONWeight> for JSONType {
    fn into(self) -> JSONWeight {
        JSONWeight::Type(self)
    }
}

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let v: Value = serde_json::from_str(include_str!("./colors.json"))?;

    let mut graph: HyperGraph<MemoryStore, JSONWeight, ()> = HyperGraph::new(MemoryStore::new())?;

    let root_element = walk_json(&mut graph, v)?;

    let json_document = graph.insert_vertex(SubGraph {
        graph_type: JSONType::Document,
    })?;

    graph.insert_directed(JSONType::RootElement, [json_document], [root_element])?;

    let mut out = std::io::stdout();
    graph.dump_entities(&mut out)?;

    JSONRenderer::render(out, &mut graph, &json_document)?;

    Ok(())
}

fn walk_json(graph: &HyperGraph<MemoryStore, JSONWeight, ()>, v: Value) -> Result<EntityId, mindbase_hypergraph::error::Error> {
    match v {
        Value::Null => graph.insert_vertex(DataNode {
            data_type: JSONType::Null,
            data: None,
        }),
        Value::Bool(b) => graph.insert_vertex(DataNode {
            data_type: JSONType::Bool,
            data: Some(vec![b as u8]),
        }),
        Value::Number(n) => graph.insert_vertex(DataNode {
            data_type: JSONType::Number,
            data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
        }),
        Value::String(s) => graph.insert_vertex(DataNode {
            data_type: JSONType::String,
            data: Some(s.as_bytes().to_vec()),
        }),
        Value::Array(values) => {
            //First define the array node itself
            let arr = graph.insert_vertex(DataNode {
                data_type: JSONType::Array,
                data: None,
            })?;

            // now recurse

            let mut members: Vec<EntityId> = Vec::with_capacity(values.len());
            for (i, value) in values.into_iter().enumerate() {
                let member = walk_json(graph, value)?;

                // TODO1 - how does this work with symbolic types?
                graph.insert(directed(JSONType::ArrayOffset(i), [arr], [member]))?;

                if i == 0 {
                    graph.insert(directed(JSONType::ArrHead, [arr], [member]))?;
                } else {
                    let prev = *members.last().unwrap();
                    graph.insert_directed(JSONType::ArrNextMember, [prev], [member])?;
                    graph.insert_directed(JSONType::ArrPrevMember, [member], [prev])?;
                };

                members.push(member);
            }
            if let Some(tail) = members.last() {
                graph.insert_directed(JSONType::ArrTail, [arr], [*tail])?;
            }

            graph.insert(directed(JSONType::ArrayMember, [arr], members))?;

            Ok(arr)
        }
        Value::Object(values) => {
            //First define the array node itself
            let obj = graph.insert_vertex(DataNode {
                data_type: JSONType::Object,
                data: None,
            })?;
            let mut properties: Vec<EntityId> = Vec::with_capacity(values.len());
            let mut members: Vec<EntityId> = Vec::with_capacity(values.len());

            // now recurse
            for (key, value) in values {
                let member = walk_json(graph, value)?;

                // TODO1 - reconcile HyperedgeWeight with symbolic types
                let prop = graph.insert_directed(JSONType::ObjectProperty(key), [obj], [member])?;

                members.push(member);
                properties.push(prop);
            }
            graph.insert(directed(JSONType::ObjectMembers, [obj], members.clone()))?;
            graph.insert(directed(JSONType::ObjectProperties, [obj], members))?;

            Ok(obj)
        }
    }
}

struct JSONRenderer<'a, S, W>
where
    S: mindbase_store::Store,
    W: std::io::Write,
{
    anticycle: Vec<EntityId>,
    graph: &'a HyperGraph<S, JSONWeight, ()>,
    output: W,
}

impl<'a, S, W> JSONRenderer<'a, S, W>
where
    S: mindbase_store::Store,
    W: std::io::Write,
{
    pub fn render(output: W, graph: &'a HyperGraph<S, JSONWeight, ()>, entity: &EntityId) -> Result<(), std::io::Error> {
        let mut renderer = JSONRenderer {
            anticycle: Vec::new(),
            graph,
            output,
        };
        renderer.recurse(entity);

        Ok(())
    }

    fn anticycle_push(&mut self, entity: &EntityId) -> bool {
        match self.anticycle.binary_search(entity) {
            Ok(_) => false,
            Err(i) => {
                self.anticycle.insert(i, entity.clone());
                true
            }
        }
    }
    fn anticycle_pop(&mut self, entity_id: &EntityId) {
        match self.anticycle.binary_search(&entity_id) {
            Ok(i) => {
                self.anticycle.remove(i);
            }
            Err(i) => {}
        };
    }

    fn recurse(&mut self, entity: &EntityId) -> Result<(), std::io::Error> {
        let artifact = self.graph.get(entity)?;
        unimplemented!()
    }
}
