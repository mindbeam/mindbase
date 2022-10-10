use mindbase_hypergraph::traits::TSymbol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TestJSONSymbol {
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
    Value,
    RootElement,
}

// impl Into<TestWeight<TestJSONType>> for TestJSONType {
//     fn into(self) -> TestWeight<TestJSONType> {
//         TestWeight::Type(self)
//     }
// }
// impl From<TestJSONType> for TestWeight<TestJSONType> {
//     fn from(t: TestJSONType) -> Self {
//         TestWeight::Type(t)
//     }
// }

impl TestJSONSymbol {
    pub fn typemap() -> crate::json::JsonTypeMap<TestJSONSymbol> {
        crate::json::JsonTypeMap {
            Document: TestJSONSymbol::Document,
            Null: TestJSONSymbol::Null,
            Bool: TestJSONSymbol::Bool,
            Number: TestJSONSymbol::Number,
            String: TestJSONSymbol::String,
            Array: TestJSONSymbol::Array,
            ArrayMember: TestJSONSymbol::ArrayMember,
            ArrayOffset: TestJSONSymbol::ArrayOffset,
            ArrNextMember: TestJSONSymbol::ArrNextMember,
            ArrPrevMember: TestJSONSymbol::ArrPrevMember,
            ArrHead: TestJSONSymbol::ArrHead,
            ArrTail: TestJSONSymbol::ArrTail,
            Object: TestJSONSymbol::Object,
            ObjectProperty: TestJSONSymbol::ObjectProperty,
            ObjectProperties: TestJSONSymbol::ObjectProperties,
            ObjectMembers: TestJSONSymbol::ObjectMembers,
            RootElement: TestJSONSymbol::RootElement,
        }
    }
}

impl TSymbol for TestJSONSymbol {
    // fn compare<G, W>(&self, other: &Self, graph: &G) -> Result<f64, mindbase_hypergraph::Error>
    // where
    //     G: mindbase_hypergraph::traits::GraphInterface<W>,
    //     W: mindbase_hypergraph::traits::TValue<Symbol = Self>,
    // {
    //     if *self == *other {
    //         Ok(1.0)
    //     } else {
    //         Ok(0.0)
    //     }
    // }
}
