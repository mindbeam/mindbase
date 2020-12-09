use std::collections::{btree_map::Entry, BTreeMap};

use mindbase_artifact::{body::DataNode, body::DataRelation, body::SubGraph, Artifact, ArtifactId, NodeInstance, NodeType};
use mindbase_graph::Graph;
use mindbase_store::MemoryStore;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INCREMENT: Arc<Mutex<u32>> = Arc::new(Mutex::new(0u32));
}

#[derive(Serialize, Debug)]
enum JNT {
    Document,
    Null,
    Bool,
    Number,
    String,
    Array,
    ArrayMember,
    ArrNextMember,
    ArrPrevMember,
    ArrHead,
    ArrTail,
    Object,
    ObjectProperty,
    Contains,
    Value,
}
#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct JNI {
    id: u32,
    artifact_id: ArtifactId,
}

impl NodeType for JNT {}
impl NodeInstance for JNI {}

impl mindbase_graph::traits::ArtifactInstance<Artifact<JNT, JNI>> for JNI {
    type ID = [u8; 4];

    fn id(&self) -> Self::ID {
        self.id.to_ne_bytes()
    }

    fn instantiate(artifact_id: &ArtifactId) -> Self {
        Self::new(artifact_id.clone())
    }

    fn get_id_and_bytes(&self) -> (Self::ID, Vec<u8>) {
        (self.id(), bincode::serialize(self).unwrap())
    }

    fn from_id_and_bytes<B: AsRef<u8>>(id: Self::ID, bytes: B) {
        todo!()
    }
}

impl JNI {
    pub fn new(artifact_id: ArtifactId) -> Self {
        let mut inc = INCREMENT.lock().unwrap();
        let id = *inc;
        *inc += 1;

        JNI { id, artifact_id }
    }
}
impl std::fmt::Display for JNI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.id, self.artifact_id)
    }
}

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let data = include_str!("./colors.json");
    let v: Value = serde_json::from_str(&data)?;

    let mut graph: Graph<MemoryStore, Artifact<JNT, JNI>, JNI> = Graph::new(MemoryStore::new()).unwrap();
    let root = walk_json(&mut graph, v)?;

    let document = graph.put_artifact(SubGraph {
        graph_type: JNT::Document,
        nodes: vec![root],
    })?;

    // NEXT render it back out (and consider how to genericize walk/rander)
    // render(graph, document);
    Ok(())
}

fn walk_json(store: &mut Graph<MemoryStore, Artifact<JNT, JNI>, JNI>, v: Value) -> Result<JNI, mindbase_graph::error::Error> {
    match v {
        Value::Null => store.put_artifact(DataNode {
            data_type: JNT::Null,
            data: None,
        }),
        Value::Bool(b) => store.put_artifact(DataNode {
            data_type: JNT::Bool,
            data: Some(vec![b as u8]),
        }),
        Value::Number(n) => store.put_artifact(DataNode {
            data_type: JNT::Number,
            data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
        }),
        Value::String(s) => store.put_artifact(DataNode {
            data_type: JNT::String,
            data: Some(s.as_bytes().to_vec()),
        }),
        Value::Array(values) => {
            //First define the array node itself
            let arr = store.put_artifact(DataNode {
                data_type: JNT::Array,
                data: None,
            })?;

            // now recurse

            let mut last: Option<JNI> = None;
            for (i, value) in values.into_iter().enumerate() {
                let member = store.put_artifact(DataNode {
                    data_type: JNT::ArrayMember,
                    data: Some((i as u64).to_ne_bytes().to_vec()),
                })?;

                store.put_artifact(DataRelation {
                    relation_type: JNT::Contains,
                    from: arr.clone(),
                    to: member.clone(),
                })?;

                if i == 0 {
                    store.put_artifact(DataRelation {
                        relation_type: JNT::ArrHead,
                        from: arr.clone(),
                        to: member.clone(),
                    })?;
                }
                if let Some(prev) = last {
                    store.put_artifact(DataRelation {
                        relation_type: JNT::ArrNextMember,
                        from: prev.clone(),
                        to: member.clone(),
                    })?;
                    store.put_artifact(DataRelation {
                        relation_type: JNT::ArrPrevMember,
                        from: member.clone(),
                        to: prev,
                    })?;
                };
                last = Some(member.clone());

                let value = walk_json(store, value)?;

                store.put_artifact(DataRelation {
                    relation_type: JNT::Value,
                    from: member,
                    to: value,
                })?;
            }

            if let Some(prev) = last {
                store.put_artifact(DataRelation {
                    relation_type: JNT::ArrTail,
                    from: arr.clone(),
                    to: prev,
                })?;
            }

            Ok(arr)
        }
        Value::Object(values) => {
            //First define the array node itself
            let obj = store.put_artifact(DataNode {
                data_type: JNT::Object,
                data: None,
            })?;

            // now recurse
            for (key, value) in values {
                let prop = store.put_artifact(DataNode {
                    data_type: JNT::ObjectProperty,
                    data: Some(key.as_bytes().to_owned()),
                })?;

                store.put_artifact(DataRelation {
                    relation_type: JNT::Contains,
                    from: obj.clone(),
                    to: prop.clone(),
                })?;

                let value = walk_json(store, value)?;

                store.put_artifact(DataRelation {
                    relation_type: JNT::Value,
                    from: prop,
                    to: value,
                })?;
            }

            Ok(obj)
        }
    }
}
