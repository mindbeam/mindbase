pub mod test;
pub mod typemap;

use std::marker::PhantomData;

use mindbase_artifact::{
    body::{DataNode, Type},
    Artifact, ArtifactNodeType,
};
use mindbase_hypergraph::{
    entity::{directed, vertex},
    traits::{GraphInterface, Symbol, Weight},
    Entity, EntityId,
};
use serde_json::Value;

use crate::Error;

use self::typemap::JsonType;
pub use self::typemap::JsonTypeMap;

pub struct JsonAdapter<'a, G, T>
where
    T: Clone,
{
    graph: &'a G,
    typemap: JsonTypeMap<T>,
}

impl<'a, G, JsonTypeSymbol> JsonAdapter<'a, G, JsonTypeSymbol>
where
    G: GraphInterface<Artifact<JsonTypeSymbol>>,
    // W: Weight<Symbol = S> + From<DataNode<S>> + From<SubGraph<S>> + From<Type<S>>,
    JsonTypeSymbol: Symbol + ArtifactNodeType + Clone + std::fmt::Debug,
{
    pub fn new(graph: &'a G, typemap: JsonTypeMap<JsonTypeSymbol>) -> Self {
        Self {
            graph,
            // a: PhantomData,
            typemap,
        }
    }
    /// Load a json file into the hypergraph
    ///
    /// Note that this operation is NOT idempotent. We are not deduplicating json files or elements at this time.
    /// That operation would require a high degree of opinionation about precisely how it ought to be performed
    pub fn load<'b, R: std::io::Read>(&self, reader: R, filename: String) -> Result<EntityId, Error<Artifact<JsonTypeSymbol>>> {
        let jv: Value = serde_json::from_reader(reader)?;

        let root_element = self.input_recurse(jv)?;

        let json_document = self.graph.insert(vertex(DataNode {
            data_type: self.typemap.to_sym(JsonType::Document),
            data: Some(filename.into_bytes()),
        }))?;

        self.graph.insert(directed(
            Type(self.typemap.to_sym(JsonType::RootElement)),
            [json_document],
            [root_element],
        ))?;

        Ok(json_document)
    }
    pub fn get_filename_and_root<'b>(
        &self, entity_id: &'b EntityId,
    ) -> Result<(Option<String>, EntityId), Error<Artifact<JsonTypeSymbol>>> {
        // might be a document, or a root node
        let entity = self.graph.get(&entity_id)?;

        match &entity.weight {
            Artifact::Node(DataNode { ref data_type, data }) => {
                match self.typemap.from_sym(data_type, self.graph)? {
                    JsonType::Document => {
                        // This is dumb. it should not be a fulter function
                        let roots = self.graph.get_adjacencies_matching(entity_id, |a| {
                            if let Artifact::Type(Type(ty)) = a {
                                let score = ty.compare(&self.typemap.RootElement, self.graph)?;

                                if score > 0.7 {
                                    return Ok(true);
                                }
                            }
                            Ok(false)
                        })?;
                        if roots.len() == 0 {
                            return Err(Error::InvariantViolation("No RootElement found for Document"));
                        }
                        if roots.len() > 1 {
                            return Err(Error::InvariantViolation("Multiple RootElements found for Document"));
                        }
                        let root_id = roots[0];
                        Ok((data.as_ref().map(|b| String::from_utf8_lossy(&b).to_string()), root_id))
                    }
                    JsonType::Null | JsonType::Bool | JsonType::Number | JsonType::Array | JsonType::Object => {
                        // These are legal node types to start rendering
                        Ok((None, *entity_id))
                    }
                    _ => Err(Error::MaterializationDeclined {
                        entity,
                        reason: "Invalid root JSON entity",
                    }),
                }
            }
            Artifact::Agent(_) => Err(Error::MaterializationDeclined {
                entity,
                reason: "Entity should be a node, but is an Agent",
            }),
            Artifact::Url(_) => Err(Error::MaterializationDeclined {
                entity,
                reason: "Entity should be a node, but is a Url",
            }),
            Artifact::FlatText(_) => Err(Error::MaterializationDeclined {
                entity,
                reason: "Entity should be a node, but is a FlatText",
            }),
            Artifact::Type(v) => Err(Error::MaterializationDeclined {
                entity,
                reason: "Entity should be a node, but is a Type",
            }),
        }
    }
    pub fn write<'b, R: std::io::Write>(&self, writer: R, entity_id: EntityId) -> Result<(), Error<Artifact<JsonTypeSymbol>>> {
        let mut cycleguard = CycleGuard::default();

        // skip over the document vertex, if applicable
        let (_filename, root_id) = self.get_filename_and_root(&entity_id)?;

        // self.output_recurse(&mut cycleguard, &root_id, &writer)?;

        Ok(())
    }

    fn input_recurse<'b, W: Weight>(&self, v: Value) -> Result<EntityId, Error<W>> {
        let tm = &self.typemap;
        Ok(match v {
            Value::Null => self.graph.insert(vertex(DataNode {
                data_type: tm.to_sym(JsonType::Null),
                data: None,
            }))?,
            Value::Bool(b) => self.graph.insert(vertex(DataNode {
                data_type: tm.to_sym(JsonType::Bool),
                data: Some(vec![b as u8]),
            }))?,
            Value::Number(n) => self.graph.insert(vertex(DataNode {
                data_type: tm.to_sym(JsonType::Number),
                data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
            }))?,
            Value::String(s) => self.graph.insert(vertex(DataNode {
                data_type: tm.to_sym(JsonType::String),
                data: Some(s.as_bytes().to_vec()),
            }))?,
            Value::Array(values) => {
                //First define the array node itself
                let arr = self.graph.insert(vertex(Type(tm.to_sym(JsonType::Array))))?;

                // now recurse
                let mut members: Vec<EntityId> = Vec::with_capacity(values.len());
                for (i, value) in values.into_iter().enumerate() {
                    let member = self.input_recurse(value)?;

                    self.graph.insert(directed(
                        DataNode {
                            data_type: self.typemap.to_sym(JsonType::ArrayOffset),
                            data: Some(i.to_ne_bytes().to_vec()),
                        },
                        [arr],
                        [member],
                    ))?;

                    if i == 0 {
                        self.graph
                            .insert(directed(Type(tm.to_sym(JsonType::ArrHead)), [arr], [member]))?;
                    } else {
                        let prev = *members.last().unwrap();
                        self.graph
                            .insert(directed(Type(tm.to_sym(JsonType::ArrNextMember)), [prev], [member]))?;
                        self.graph
                            .insert(directed(Type(tm.to_sym(JsonType::ArrPrevMember)), [member], [prev]))?;
                    };

                    members.push(member);
                }
                if let Some(tail) = members.last() {
                    self.graph
                        .insert(directed(Type(tm.to_sym(JsonType::ArrTail)), [arr], [*tail]))?;
                }

                self.graph
                    .insert(directed(Type(tm.to_sym(JsonType::ArrayMember)), [arr], members))?;

                arr
            }
            Value::Object(values) => {
                //First define the array node itself
                let obj = self.graph.insert(vertex(DataNode {
                    data_type: tm.to_sym(JsonType::Object),
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
                            data_type: tm.to_sym(JsonType::ObjectProperty),
                            data: Some(key.bytes().collect()),
                        },
                        [obj],
                        [member],
                    ))?;

                    members.push(member);
                    properties.push(prop);
                }
                self.graph
                    .insert(directed(Type(tm.to_sym(JsonType::ObjectMembers)), [obj], members.clone()))?;
                self.graph
                    .insert(directed(Type(tm.to_sym(JsonType::ObjectProperties)), [obj], members))?;

                obj
            }
        })
    }

    //     fn output_recurse<R: std::io::Write>(
    //         &self, cycleguard: &mut CycleGuard, entity_id: &EntityId, writer: &R,
    //     ) -> Result<(), Error> {
    //         cycleguard.push(entity_id)?;

    //         // QUESTION: How might we potentially vectorize this kind of retrieval?

    //         let entity = self.graph.get(entity_id)?;
    //         let artifact = entity.weight;

    //         println!("ARTIFACT {:?}", artifact);
    //         // The distinction between edge and vertex feels wrong.
    //         // Maybe there should only be edges?

    //         match artifact {
    //             Artifact::Node(DataNode { data_type, data }) => match self.tm.from_sym(data_type, self.graph)? {
    //                 JsonType::Document => return Err(Error::MaterializationDeclined),
    //                 JsonType::Null => {}
    //                 JsonType::Bool => {}
    //                 JsonType::Number => {}
    //                 JsonType::String => {}
    //                 JsonType::Array => {}
    //                 JsonType::ArrayMember => {}
    //                 JsonType::ArrayOffset => {}
    //                 JsonType::ArrNextMember => {}
    //                 JsonType::ArrPrevMember => {}
    //                 JsonType::ArrHead => {}
    //                 JsonType::ArrTail => {}
    //                 JsonType::Object => {}
    //                 JsonType::ObjectProperty => {}
    //                 JsonType::ObjectProperties => {}
    //                 JsonType::ObjectMembers => {}
    //                 JsonType::RootElement => {
    //                     if let Some(entity_ids) = self.graph.get_adjacencies(entity_id)? {
    //                         for target_entity_id in entity_ids {
    //                             self.output_recurse(cycleguard, &target_entity_id, writer)?;
    //                         }
    //                     }
    //                 }
    //             },
    //             Artifact::Type(Type(s)) => match self.tm.from_sym(s, self.graph)? {
    //                 JsonType::Document => {}
    //                 JsonType::Null => {}
    //                 JsonType::Bool => {}
    //                 JsonType::Number => {}
    //                 JsonType::String => {}
    //                 JsonType::Array => {}
    //                 JsonType::ArrayMember => {}
    //                 JsonType::ArrayOffset => {}
    //                 JsonType::ArrNextMember => {}
    //                 JsonType::ArrPrevMember => {}
    //                 JsonType::ArrHead => {}
    //                 JsonType::ArrTail => {}
    //                 JsonType::Object => {}
    //                 JsonType::ObjectProperty => {}
    //                 JsonType::ObjectProperties => {}
    //                 JsonType::ObjectMembers => {}
    //                 JsonType::RootElement => {
    //                     if let Some(entity_ids) = self.graph.get_adjacencies(entity_id)? {
    //                         for target_entity_id in entity_ids {
    //                             self.output_recurse(cycleguard, &target_entity_id, writer)?;
    //                         }
    //                     }
    //                 }
    //             },
    //             _ => return Err(Error::InvariantViolation("Invalid artifact type for JSON")),
    //         }

    //         // Ok(match v {
    //         //     Value::Null => self.graph.insert(vertex(DataNode {
    //         //         data_type: self.tm.to_sym(JsonType::$),
    //         //         data: None,
    //         //     }))?,
    //         //     Value::Bool(b) => self.graph.insert(vertex(DataNode {
    //         //         data_type: self.tm.to_sym(JsonType::$),
    //         //         data: Some(vec![b as u8]),
    //         //     }))?,
    //         //     Value::Number(n) => self.graph.insert(vertex(DataNode {
    //         //         data_type: self.tm.to_sym(JsonType::$),
    //         //         data: Some(n.as_i64().unwrap().to_ne_bytes().to_vec()),
    //         //     }))?,
    //         //     Value::String(s) => self.graph.insert(vertex(DataNode {
    //         //         data_type: self.tm.to_sym(JsonType::$),
    //         //         data: Some(s.as_bytes().to_vec()),
    //         //     }))?

    //         cycleguard.pop(entity_id)?;

    //         Ok(())
    //     }
}

#[derive(Default)]
struct CycleGuard(Vec<EntityId>);

impl<'a> CycleGuard {
    fn push<W: Weight>(&mut self, entity: &EntityId) -> Result<(), Error<W>> {
        match self.0.binary_search(entity) {
            Ok(_) => Err(Error::CycleDetected),
            Err(i) => {
                self.0.insert(i, entity.clone());
                Ok(())
            }
        }
    }
    fn pop<W: Weight>(&mut self, entity_id: &EntityId) -> Result<(), Error<W>> {
        match self.0.binary_search(&entity_id) {
            Ok(i) => {
                self.0.remove(i);
                Ok(())
            }
            Err(i) => Err(Error::Sanity),
        }
    }
}
