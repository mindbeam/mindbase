use crate::concept::Concept;

use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Analogy {
    pub concept:    Concept,
    pub confidence: f32,
    pub memberof:   Concept,
}

impl Analogy {
    pub fn declare<T>(concept: T, memberof: T) -> Self
        where T: Into<Concept>
    {
        Analogy { concept:    concept.into(),
                  confidence: 1.0,
                  memberof:   memberof.into(), }
    }

    pub fn declare_neg(concept: Concept, memberof: Concept) -> Self {
        Analogy { concept,
                  confidence: -1.0,
                  memberof }
    }

    pub fn to_string(&self) -> String {
        format!("{} is in the category of {} ({})",
                self.concept.to_string(),
                self.memberof.to_string(),
                self.confidence).to_string()
    }
}

impl Into<crate::allegation::Body> for Analogy {
    fn into(self) -> crate::allegation::Body {
        crate::allegation::Body::Analogy(self)
    }
}
