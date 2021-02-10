use std::marker::PhantomData;

use mindbase_artifact::{
    body::{DataNode, SubGraph},
    Artifact, NodeType,
};
use mindbase_hypergraph::{
    entity::{directed, vertex},
    traits::{GraphInterface, Weight},
    EntityId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Error;

pub struct JsonAdapter<'a, G, W> {
    graph: &'a G,
    _w: PhantomData<W>,
}

impl<'a, G, W> JsonAdapter<'a, G, W>
where
    G: GraphInterface<W>,
    W: Weight + From<JSONType> + From<DataNode<JSONType>> + From<SubGraph<JSONType>>,
{
    pub fn new(graph: &'a G) -> Self {
        Self { graph, _w: PhantomData }
    }
    pub fn load<R: std::io::Read>(&self, reader: R) -> Result<EntityId, Error> {
        let jv: Value = serde_json::from_reader(reader)?;

        let root_element = self.input_recurse(jv)?;
        let json_document = self.graph.insert(vertex(SubGraph {
            graph_type: JSONType::Document,
        }))?;

        self.graph
            .insert(directed(JSONType::RootElement, [json_document], [root_element]))?;

        Ok(json_document)
    }
    pub fn write<R: std::io::Write>(&self, writer: R, entity_id: EntityId) -> Result<(), Error> {
        let mut cycleguard = CycleGuard::default();
        self.output_recurse(&mut cycleguard, &entity_id, writer);
        Ok(())
    }

    fn input_recurse(&self, v: Value) -> Result<EntityId, Error> {
        Ok(match v {
            Value::Null => self.graph.insert(vertex(DataNode {
                data_type: JSONType::Null,
                data: None,
            }))?,
            Value::Bool(b) => self.graph.insert(vertex(DataNode {
                data_type: JSONType::Bool,
                data: Some(vec![b as u8]),
            }))?,
            Value::Number(n) => self.graph.insert(vertex(DataNode {
                data_type: JSONType::Number,
                data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
            }))?,
            Value::String(s) => self.graph.insert(vertex(DataNode {
                data_type: JSONType::String,
                data: Some(s.as_bytes().to_vec()),
            }))?,
            Value::Array(values) => {
                //First define the array node itself
                let arr = self.graph.insert(vertex(DataNode {
                    data_type: JSONType::Array,
                    data: None,
                }))?;

                // now recurse

                let mut members: Vec<EntityId> = Vec::with_capacity(values.len());
                for (i, value) in values.into_iter().enumerate() {
                    let member = self.input_recurse(value)?;

                    // TODO1 - how does this work with symbolic types?
                    self.graph.insert(directed(JSONType::ArrayOffset(i), [arr], [member]))?;

                    if i == 0 {
                        self.graph.insert(directed(JSONType::ArrHead, [arr], [member]))?;
                    } else {
                        let prev = *members.last().unwrap();
                        self.graph.insert(directed(JSONType::ArrNextMember, [prev], [member]))?;
                        self.graph.insert(directed(JSONType::ArrPrevMember, [member], [prev]))?;
                    };

                    members.push(member);
                }
                if let Some(tail) = members.last() {
                    self.graph.insert(directed(JSONType::ArrTail, [arr], [*tail]))?;
                }

                self.graph.insert(directed(JSONType::ArrayMember, [arr], members))?;

                arr
            }
            Value::Object(values) => {
                //First define the array node itself
                let obj = self.graph.insert(vertex(DataNode {
                    data_type: JSONType::Object,
                    data: None,
                }))?;
                let mut properties: Vec<EntityId> = Vec::with_capacity(values.len());
                let mut members: Vec<EntityId> = Vec::with_capacity(values.len());

                // now recurse
                for (key, value) in values {
                    let member = self.input_recurse(value)?;

                    // TODO1 - reconcile HyperedgeWeight with symbolic types
                    let prop = self.graph.insert(directed(JSONType::ObjectProperty(key), [obj], [member]))?;

                    members.push(member);
                    properties.push(prop);
                }
                self.graph.insert(directed(JSONType::ObjectMembers, [obj], members.clone()))?;
                self.graph.insert(directed(JSONType::ObjectProperties, [obj], members))?;

                obj
            }
        })
    }

    fn output_recurse<R: std::io::Write>(
        &self, cycleguard: &mut CycleGuard, entity_id: &EntityId, writer: R,
    ) -> Result<(), std::io::Error> {
        cycleguard.push(entity_id)?;
        let entity = self.graph.get(entity_id)?;

        // LEFT OFF HERE - TODO
        // We need to be able to access the artifact here, while still preserving the ability to test without using fuzzy symbols
        // match entity.weight {
        // }
        cycleguard.pop(entity_id);
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JSONType {
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

impl Weight for JSONType {}
impl NodeType for JSONType {}

#[derive(Default)]
struct CycleGuard(Vec<EntityId>);

impl CycleGuard {
    fn push(&mut self, entity: &EntityId) -> Result<(), Error> {
        match self.0.binary_search(entity) {
            Ok(_) => Err(Error::CycleDetected),
            Err(i) => {
                self.0.insert(i, entity.clone());
                Ok(())
            }
        }
    }
    fn pop(&mut self, entity_id: &EntityId) -> Result<(), Error> {
        match self.0.binary_search(&entity_id) {
            Ok(i) => {
                self.0.remove(i);
                Ok(())
            }
            Err(i) => Err(Error::Sanity),
        }
    }
}
