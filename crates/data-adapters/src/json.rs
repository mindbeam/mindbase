pub mod test;

use std::marker::PhantomData;

use mindbase_artifact::body::{DataNode, SubGraph};
use mindbase_hypergraph::{
    entity::{directed, vertex},
    traits::{GraphInterface, Symbol, Weight},
    EntityId,
};
use serde_json::Value;

use crate::Error;

pub struct JsonAdapter<'a, G, W, T>
where
    T: Clone,
{
    graph: &'a G,
    tm: TypeMap<T>,
    _w: PhantomData<W>,
}

impl<'a, G, W, S> JsonAdapter<'a, G, W, S>
where
    G: GraphInterface<W>,
    W: Weight<Symbol = S> + From<S> + From<DataNode<S>> + From<SubGraph<S>>,
    S: Symbol + Clone,
{
    pub fn new(graph: &'a G, typemap: TypeMap<S>) -> Self {
        Self {
            graph,
            _w: PhantomData,
            tm: typemap,
        }
    }
    pub fn load<R: std::io::Read>(&self, reader: R) -> Result<EntityId, Error> {
        let jv: Value = serde_json::from_reader(reader)?;

        let root_element = self.input_recurse(jv)?;
        let json_document = self.graph.insert(vertex(SubGraph {
            graph_type: self.tm.Document.clone(),
        }))?;

        self.graph
            .insert(directed(self.tm.RootElement.clone(), [json_document], [root_element]))?;

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
                data_type: self.tm.Null.clone(),
                data: None,
            }))?,
            Value::Bool(b) => self.graph.insert(vertex(DataNode {
                data_type: self.tm.Bool.clone(),
                data: Some(vec![b as u8]),
            }))?,
            Value::Number(n) => self.graph.insert(vertex(DataNode {
                data_type: self.tm.Number.clone(),
                data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
            }))?,
            Value::String(s) => self.graph.insert(vertex(DataNode {
                data_type: self.tm.String.clone(),
                data: Some(s.as_bytes().to_vec()),
            }))?,
            Value::Array(values) => {
                //First define the array node itself
                let arr = self.graph.insert(vertex(DataNode {
                    data_type: self.tm.Array.clone(),
                    data: None,
                }))?;

                // now recurse

                let mut members: Vec<EntityId> = Vec::with_capacity(values.len());
                for (i, value) in values.into_iter().enumerate() {
                    let member = self.input_recurse(value)?;

                    // TODO1 - how does this work with symbolic types?
                    self.graph.insert(directed(
                        DataNode {
                            data_type: self.tm.ArrayOffset.clone(),
                            data: Some(i.to_ne_bytes().to_vec()),
                        },
                        [arr],
                        [member],
                    ))?;

                    if i == 0 {
                        self.graph.insert(directed(self.tm.ArrHead.clone(), [arr], [member]))?;
                    } else {
                        let prev = *members.last().unwrap();
                        self.graph.insert(directed(self.tm.ArrNextMember.clone(), [prev], [member]))?;
                        self.graph.insert(directed(self.tm.ArrPrevMember.clone(), [member], [prev]))?;
                    };

                    members.push(member);
                }
                if let Some(tail) = members.last() {
                    self.graph.insert(directed(self.tm.ArrTail.clone(), [arr], [*tail]))?;
                }

                self.graph.insert(directed(self.tm.ArrayMember.clone(), [arr], members))?;

                arr
            }
            Value::Object(values) => {
                //First define the array node itself
                let obj = self.graph.insert(vertex(DataNode {
                    data_type: self.tm.Object.clone(),
                    data: None,
                }))?;
                let mut properties: Vec<EntityId> = Vec::with_capacity(values.len());
                let mut members: Vec<EntityId> = Vec::with_capacity(values.len());

                // now recurse
                for (key, value) in values {
                    let member = self.input_recurse(value)?;

                    // TODO1 - reconcile HyperedgeWeight with symbolic types
                    let prop = self.graph.insert(directed(
                        DataNode {
                            data_type: self.tm.ObjectProperty.clone(),
                            data: Some(key.bytes().collect()),
                        },
                        [obj],
                        [member],
                    ))?;

                    members.push(member);
                    properties.push(prop);
                }
                self.graph
                    .insert(directed(self.tm.ObjectMembers.clone(), [obj], members.clone()))?;
                self.graph
                    .insert(directed(self.tm.ObjectProperties.clone(), [obj], members))?;

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

#[allow(non_snake_case)]
pub struct TypeMap<T>
where
    T: Clone,
{
    Document: T,
    Null: T,
    Bool: T,
    Number: T,
    String: T,
    Array: T,
    ArrayMember: T,
    ArrayOffset: T,
    ArrNextMember: T,
    ArrPrevMember: T,
    ArrHead: T,
    ArrTail: T,
    Object: T,
    ObjectProperty: T,
    ObjectProperties: T,
    ObjectMembers: T,
    RootElement: T,
}

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
