use mindbase_artifact::{test::TestWeight, ArtifactNodeType};
use mindbase_hypergraph::traits::Symbol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TestJSONType {
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
impl From<TestJSONType> for TestWeight<TestJSONType> {
    fn from(t: TestJSONType) -> Self {
        TestWeight::Type(t)
    }
}

impl TestJSONType {
    pub fn typemap() -> crate::json::JsonTypeMap<TestJSONType> {
        crate::json::JsonTypeMap {
            Document: TestJSONType::Document,
            Null: TestJSONType::Null,
            Bool: TestJSONType::Bool,
            Number: TestJSONType::Number,
            String: TestJSONType::String,
            Array: TestJSONType::Array,
            ArrayMember: TestJSONType::ArrayMember,
            ArrayOffset: TestJSONType::ArrayOffset,
            ArrNextMember: TestJSONType::ArrNextMember,
            ArrPrevMember: TestJSONType::ArrPrevMember,
            ArrHead: TestJSONType::ArrHead,
            ArrTail: TestJSONType::ArrTail,
            Object: TestJSONType::Object,
            ObjectProperty: TestJSONType::ObjectProperty,
            ObjectProperties: TestJSONType::ObjectProperties,
            ObjectMembers: TestJSONType::ObjectMembers,
            RootElement: TestJSONType::RootElement,
        }
    }
}

impl ArtifactNodeType for TestJSONType {}
impl Symbol for TestJSONType {}
