use crate::{
    allegation::Alledgable,
    concept::Concept,
    Agent,
    Allegation,
    MBError,
    MindBase,
};
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Analogy {
    pub left:       Concept,
    pub right:      Concept,
    pub confidence: f32,
}

impl Analogy {
    pub fn declarative<T>(left: T, right: T) -> Self
        where T: Into<Concept>
    {
        Analogy { left:       left.into(),
                  right:      right.into(),
                  confidence: 1.0, }
    }

    pub fn negative(left: Concept, right: Concept) -> Self {
        Analogy { left,
                  right,
                  confidence: -1.0 }
    }

    pub fn to_string(&self) -> String {
        format!("{} is in the category of {} ({})",
                self.left.to_string(),
                self.right.to_string(),
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
        write!(f, "{} is in the category of {} ({})", self.left, self.right, self.confidence)
    }
}

impl Alledgable for Analogy {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Allegation, MBError> {
        let allegation = Allegation::new(agent, crate::allegation::Body::Analogy(self))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
