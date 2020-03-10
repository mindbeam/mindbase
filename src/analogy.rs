use crate::{
    allegation::Alledgable,
    concept::Concept,
    Agent,
    Allegation,
    Error,
    MindBase,
};
use std::fmt;

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

impl fmt::Display for Analogy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{} is in the category of {} ({})",
               self.concept, self.memberof, self.confidence)
    }
}

impl Alledgable for Analogy {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, Error> {
        let allegation = Allegation::new(agent, crate::allegation::Body::Analogy(self))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
