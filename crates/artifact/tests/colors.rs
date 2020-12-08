use std::collections::{btree_map::Entry, BTreeMap};

use mindbase_artifact::{
    artifact::DataNode, artifact::DataRelation, artifact::SubGraph, Artifact, ArtifactId, NodeInstance, NodeType,
};
use serde::Serialize;
use serde_json::Value;

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INCREMENT: Arc<Mutex<usize>> = Arc::new(Mutex::new(0usize));
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
#[derive(Clone, Serialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct JNI {
    id: usize,
    artifact_id: ArtifactId,
}

impl NodeType for JNT {}
impl NodeInstance for JNI {}

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

    let mut graph = Graph::default();
    let root = walk_json(&mut graph, v);

    let document = graph.put_artifact(SubGraph {
        graph_type: JNT::Document,
        nodes: vec![root],
    });

    // println!("{:?}", graph);

    // NEXT render it back out (and consider how to genericize walk/rander)
    // render(graph, document);
    Ok(())
}

#[derive(Default, Debug)]
struct Graph {
    artifacts: BTreeMap<ArtifactId, Artifact<JNT, JNI>>,
    instances: BTreeMap<usize, JNI>,
}

impl Graph {
    fn instantiate<T: Into<Artifact<JNT, JNI>>>(&mut self, artifact: T) -> JNI {
        let artifact: Artifact<JNT, JNI> = artifact.into();

        let artifact_id = artifact.id();
        // Only store it if we haven't seen this one before
        if let Entry::Vacant(v) = self.artifacts.entry(artifact_id.clone()) {
            v.insert(artifact);
        }

        // either way we want to create an instance

        let instance = JNI::new(artifact_id);
        self.instances.insert(instance.id, instance.clone());

        instance
    }
}

fn walk_json(store: &mut Graph, v: Value) -> JNI {
    match v {
        Value::Null => store.instantiate(DataNode {
            data_type: JNT::Null,
            data: None,
        }),
        Value::Bool(b) => store.instantiate(DataNode {
            data_type: JNT::Bool,
            data: Some(vec![b as u8]),
        }),
        Value::Number(n) => store.instantiate(DataNode {
            data_type: JNT::Number,
            data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
        }),
        Value::String(s) => store.instantiate(DataNode {
            data_type: JNT::String,
            data: Some(s.as_bytes().to_vec()),
        }),
        Value::Array(values) => {
            //First define the array node itself
            let arr = store.instantiate(DataNode {
                data_type: JNT::Array,
                data: None,
            });

            // now recurse

            let mut last: Option<JNI> = None;
            for (i, value) in values.into_iter().enumerate() {
                let member = store.instantiate(DataNode {
                    data_type: JNT::ArrayMember,
                    data: Some((i as u64).to_ne_bytes().to_vec()),
                });

                store.instantiate(DataRelation {
                    relation_type: JNT::Contains,
                    from: arr.clone(),
                    to: member.clone(),
                });

                if i == 0 {
                    store.instantiate(DataRelation {
                        relation_type: JNT::ArrHead,
                        from: arr.clone(),
                        to: member.clone(),
                    });
                }
                if let Some(prev) = last {
                    store.instantiate(DataRelation {
                        relation_type: JNT::ArrNextMember,
                        from: prev.clone(),
                        to: member.clone(),
                    });
                    store.instantiate(DataRelation {
                        relation_type: JNT::ArrPrevMember,
                        from: member.clone(),
                        to: prev,
                    });
                };
                last = Some(member.clone());

                let value = walk_json(store, value);

                store.instantiate(DataRelation {
                    relation_type: JNT::Value,
                    from: member,
                    to: value,
                });
            }

            if let Some(prev) = last {
                store.instantiate(DataRelation {
                    relation_type: JNT::ArrTail,
                    from: arr.clone(),
                    to: prev,
                });
            }

            arr
        }
        Value::Object(values) => {
            //First define the array node itself
            let obj = store.instantiate(DataNode {
                data_type: JNT::Object,
                data: None,
            });

            // now recurse
            for (key, value) in values {
                let prop = store.instantiate(DataNode {
                    data_type: JNT::ObjectProperty,
                    data: Some(key.as_bytes().to_owned()),
                });

                store.instantiate(DataRelation {
                    relation_type: JNT::Contains,
                    from: obj.clone(),
                    to: prop.clone(),
                });

                let value = walk_json(store, value);

                store.instantiate(DataRelation {
                    relation_type: JNT::Value,
                    from: prop,
                    to: value,
                });
            }

            obj
        }
    }
}
