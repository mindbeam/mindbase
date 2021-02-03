use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::Display,
};

use mindbase_artifact::{body::DataNode, body::SubGraph, Artifact, ArtifactId, NodeInstance, NodeType};
use mindbase_hypergraph::{hyperedge::directed, traits::Weight, EntityId, HyperGraph, VertexId};
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

// impl mindbase_hypergraph::traits::Entity<Artifact<JSONType>> for JNE {
//     type ID = [u8; 4];

//     fn id(&self) -> Self::ID {
//         self.id.to_ne_bytes()
//     }

//     fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>) {
//         (self.id(), bincode::serialize(self).unwrap())
//     }

//     fn from_id_and_bytes<B: AsRef<u8>>(id: Self::ID, bytes: B) {
//         todo!()
//     }
// }

// impl JNE {
//     pub fn new(artifact_id: ArtifactId) -> Self {
//         let mut inc = INCREMENT.lock().unwrap();
//         let id = *inc;
//         *inc += 1;

//         JNE { id, artifact_id }
//     }
// }

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let v: Value = serde_json::from_str(include_str!("./colors.json"))?;

    let mut graph: HyperGraph<MemoryStore, Artifact<JSONType>, JSONType, ()> = HyperGraph::new(MemoryStore::new())?;

    let root_element = walk_json(&mut graph, v)?;

    let doc = graph.add_vertex(SubGraph {
        graph_type: JSONType::Document,
    })?;

    graph.add_hyperedge(directed(JSONType::RootElement, [doc], [root_element]));

    let mut out = std::io::stdout();
    graph.dump_vertexes(&mut out);
    graph.dump_hyperedges(&mut out);

    // JSONRenderer::render(out, &mut graph, &document);

    Ok(())
}

fn walk_json(
    graph: &HyperGraph<MemoryStore, Artifact<JSONType>, JSONType, ()>, v: Value,
) -> Result<EntityId, mindbase_hypergraph::error::Error> {
    match v {
        Value::Null => graph.add_vertex(DataNode {
            data_type: JSONType::Null,
            data: None,
        }),
        Value::Bool(b) => graph.add_vertex(DataNode {
            data_type: JSONType::Bool,
            data: Some(vec![b as u8]),
        }),
        Value::Number(n) => graph.add_vertex(DataNode {
            data_type: JSONType::Number,
            data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
        }),
        Value::String(s) => graph.add_vertex(DataNode {
            data_type: JSONType::String,
            data: Some(s.as_bytes().to_vec()),
        }),
        Value::Array(values) => {
            //First define the array node itself
            let arr = graph.add_vertex(DataNode {
                data_type: JSONType::Array,
                data: None,
            })?;

            // now recurse

            let mut members: Vec<EntityId> = Vec::with_capacity(values.len());
            for (i, value) in values.into_iter().enumerate() {
                let member = walk_json(graph, value)?;

                // TODO1 - how does this work with symbolic types?
                graph.add_hyperedge(directed(JSONType::ArrayOffset(i), [arr.clone()], [member.clone()]))?;

                if i == 0 {
                    graph.add_hyperedge(directed(JSONType::ArrHead, [arr.clone()], [member.clone()]))?;
                } else {
                    let prev = members.last().unwrap();
                    graph.add_hyperedge(directed(JSONType::ArrNextMember, [prev.clone()], [member.clone()]))?;
                    graph.add_hyperedge(directed(JSONType::ArrPrevMember, [member.clone()], [prev.clone()]))?;
                };

                members.push(member);
            }
            if let Some(tail) = members.last() {
                graph.add_hyperedge(directed(JSONType::ArrTail, [arr.clone()], [tail.clone()]))?;
            }

            graph.add_hyperedge(directed(JSONType::ArrayMember, [arr.clone()], members))?;

            Ok(arr)
        }
        Value::Object(values) => {
            //First define the array node itself
            let obj = graph.add_vertex(DataNode {
                data_type: JSONType::Object,
                data: None,
            })?;
            let mut properties: Vec<EntityId> = Vec::with_capacity(values.len());
            let mut members: Vec<EntityId> = Vec::with_capacity(values.len());

            // now recurse
            for (key, value) in values {
                let member = walk_json(graph, value)?;

                // TODO1 - reconcile HyperedgeWeight with symbolic types
                let prop = graph.add_hyperedge(directed(JSONType::ObjectProperty(key), [obj.clone()], [member.clone()]))?;

                members.push(member);
                properties.push(prop);
            }
            graph.add_hyperedge(directed(JSONType::ObjectMembers, [obj.clone()], members.clone()))?;
            graph.add_hyperedge(directed(JSONType::ObjectProperties, [obj.clone()], members))?;

            Ok(obj)
        }
    }
}

// struct JSONRenderer<A, I>
// where
//     A: mindbase_hypergraph::traits::Weight,
//     I: mindbase_hypergraph::traits::Entity<A>,
// {
//     anticycle: Vec<I::ID>,
// }

// impl<A, I> JSONRenderer<A, I>
// where
//     A: mindbase_hypergraph::traits::Weight,
//     I: mindbase_hypergraph::traits::Entity<A>,
// {
//     pub fn render<W, S>(output: W, graph: &HyperGraph<S, A, I>, instance: &JNE) -> Result<(), std::io::Error>
//     where
//         W: std::io::Write,
//         S: mindbase_store::Store,
//     {
//         //
//         let mut renderer = JSONRenderer::<A, I> { anticycle: Vec::new() };
//         renderer.recurse(output, graph, instance);

//         Ok(())
//     }

//     fn anticycle_push(&mut self, instance_id: I::ID) -> bool {
//         match self.anticycle.binary_search(&instance_id) {
//             Ok(_) => false,
//             Err(i) => {
//                 self.anticycle.insert(i, instance_id.clone());
//                 true
//             }
//         }
//     }
//     fn anticycle_pop(&mut self, instance_id: I::ID) {
//         match self.anticycle.binary_search(&instance_id) {
//             Ok(i) => {
//                 self.anticycle.remove(i);
//             }
//             Err(i) => {}
//         };
//     }

//     fn recurse<W, S>(&mut self, output: W, graph: &HyperGraph<S, A, I>, instance: &JNE) -> Result<(), std::io::Error>
//     where
//         W: std::io::Write,
//         S: mindbase_store::Store,
//     {
//         let artifact = graph.get_weight(instance);
//     }
// }
