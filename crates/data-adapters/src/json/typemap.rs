use std::cmp::Ordering;

use mindbase_artifact::{Artifact, ArtifactNodeType};
use mindbase_hypergraph::traits::{GraphInterface, Symbol};

use crate::Error;

#[allow(non_snake_case)]
pub struct JsonTypeMap<T>
where
    T: Clone,
{
    pub Document: T,
    pub Null: T,
    pub Bool: T,
    pub Number: T,
    pub String: T,
    pub Array: T,
    pub ArrayMember: T,
    pub ArrayOffset: T,
    pub ArrNextMember: T,
    pub ArrPrevMember: T,
    pub ArrHead: T,
    pub ArrTail: T,
    pub Object: T,
    pub ObjectProperty: T,
    pub ObjectProperties: T,
    pub ObjectMembers: T,
    pub RootElement: T,
}

#[derive(Clone, Copy)]
pub enum JsonType {
    Document,
    Null,
    Bool,
    Number,
    String,
    Array,
    ArrayMember,
    ArrayOffset,
    ArrNextMember,
    ArrPrevMember,
    ArrHead,
    ArrTail,
    Object,
    ObjectProperty,
    ObjectProperties,
    ObjectMembers,
    RootElement,
}

impl<T> JsonTypeMap<T>
where
    T: Symbol + ArtifactNodeType + Clone,
{
    pub fn to_sym(&self, jt: JsonType) -> T {
        match jt {
            JsonType::Document => self.Document.clone(),
            JsonType::Null => self.Null.clone(),
            JsonType::Bool => self.Bool.clone(),
            JsonType::Number => self.Number.clone(),
            JsonType::String => self.String.clone(),
            JsonType::Array => self.Array.clone(),
            JsonType::ArrayMember => self.ArrayMember.clone(),
            JsonType::ArrayOffset => self.ArrayOffset.clone(),
            JsonType::ArrNextMember => self.ArrNextMember.clone(),
            JsonType::ArrPrevMember => self.ArrPrevMember.clone(),
            JsonType::ArrHead => self.ArrHead.clone(),
            JsonType::ArrTail => self.ArrTail.clone(),
            JsonType::Object => self.Object.clone(),
            JsonType::ObjectProperty => self.ObjectProperty.clone(),
            JsonType::ObjectProperties => self.ObjectProperties.clone(),
            JsonType::ObjectMembers => self.ObjectMembers.clone(),
            JsonType::RootElement => self.RootElement.clone(),
        }
    }
    pub fn from_sym<G>(&self, symbol: T, graph: &G) -> Result<JsonType, Error>
    where
        G: GraphInterface<Artifact<T>>,
    {
        // Who should decide this?
        // How is it calculated for hyperedges?

        // TODO - cache the comparison per symbol
        let threshold = 0.7;

        struct Comp(JsonType, f64);
        let mut candidates = [
            Comp(JsonType::Document, symbol.compare(&self.Document, graph)?),
            Comp(JsonType::Null, symbol.compare(&self.Null, graph)?),
            Comp(JsonType::Bool, symbol.compare(&self.Bool, graph)?),
            Comp(JsonType::Number, symbol.compare(&self.Number, graph)?),
            Comp(JsonType::String, symbol.compare(&self.String, graph)?),
            Comp(JsonType::Array, symbol.compare(&self.Array, graph)?),
            Comp(JsonType::ArrayMember, symbol.compare(&self.ArrayMember, graph)?),
            Comp(JsonType::ArrayOffset, symbol.compare(&self.ArrayOffset, graph)?),
            Comp(JsonType::ArrNextMember, symbol.compare(&self.ArrNextMember, graph)?),
            Comp(JsonType::ArrPrevMember, symbol.compare(&self.ArrPrevMember, graph)?),
            Comp(JsonType::ArrHead, symbol.compare(&self.ArrHead, graph)?),
            Comp(JsonType::ArrTail, symbol.compare(&self.ArrTail, graph)?),
            Comp(JsonType::Object, symbol.compare(&self.Object, graph)?),
            Comp(JsonType::ObjectProperty, symbol.compare(&self.ObjectProperty, graph)?),
            Comp(JsonType::ObjectProperties, symbol.compare(&self.ObjectProperties, graph)?),
            Comp(JsonType::ObjectMembers, symbol.compare(&self.ObjectMembers, graph)?),
            Comp(JsonType::RootElement, symbol.compare(&self.RootElement, graph)?),
        ];
        candidates.sort_by(|Comp(_, a), Comp(_, b)| b.partial_cmp(a).unwrap_or(Ordering::Greater));

        if candidates[0].1 > threshold {
            return Ok(candidates[0].0.clone());
        } else {
            Err(Error::SymbolResolution)
        }
    }
}
