use std::collections::BTreeMap;

use artifact::{body::DataNode, body::DataNodeRelation, Artifact, ArtifactId, NodeType};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
enum JNT {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl NodeType for JNT {}
type BTM = BTreeMap<ArtifactId, Artifact<JNT>>;

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("colors.json")?;
    let v: Value = serde_json::from_str(&data)?;

    let mut map: BTM = BTreeMap::new();
    walk_json(&mut map, v);
    Ok(())
}

fn walk_json(map: &mut BTM, v: Value) -> ArtifactId {
    match v {
        Value::Null => {
            let a: Artifact<JNT> = DataNode {
                data_type: JNT::Null,
                data: None,
            }
            .into();
            let id = a.id();
            map.insert(id.clone(), a);
            id
        }
        Value::Bool(b) => {
            let a: Artifact<JNT> = DataNode {
                data_type: JNT::Bool,
                data: Some("1".as_bytes().to_vec()),
            }
            .into();
            let id = a.id();
            map.insert(id.clone(), a);
            id
        }
        Value::Number(n) => {
            let a: Artifact<JNT> = DataNode {
                data_type: JNT::Number,
                data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
            }
            .into();
            let id = a.id();
            map.insert(id.clone(), a);
            id
        }
        Value::String(s) => {
            let a: Artifact<JNT> = DataNode {
                data_type: JNT::String,
                data: Some(s.as_bytes().to_vec()),
            }
            .into();
            let id = a.id();
            map.insert(id.clone(), a);
            id
        }
        Value::Array(values) => {
            let a: Artifact<JNT> = DataNode {
                data_type: JNT::Array,
                data: None,
            }
            .into();

            // LEFT OFF HERE - This is where we run into problems with ID being based on the hash
            // TODO - determine a sensible structure for artifact graph representations

            // let id = a.id();
            // map.insert(id, a);

            // DataNodeRelation {
            //     to: (),
            //     relation_type: (),
            // }
            unimplemented!()
        }
        Value::Object(_) => {
            unimplemented!()
        }
    }
}
