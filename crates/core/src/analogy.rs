use crate::{claim::Alledgable, symbol::Symbol, Agent, Claim, MBError, MindBase};
use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Analogy {
    pub left: Symbol,
    pub right: Symbol,
    pub confidence: f32,
}

impl Analogy {
    pub fn declarative<T>(left: T, right: T) -> Self
    where
        T: Into<Symbol>,
    {
        Analogy {
            left: left.into(),
            right: right.into(),
            confidence: 1.0,
        }
    }

    pub fn negative(left: Symbol, right: Symbol) -> Self {
        Analogy {
            left,
            right,
            confidence: -1.0,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} is in the category of {} ({})",
            self.left.to_string(),
            self.right.to_string(),
            self.confidence
        )
        .to_string()
    }
}

impl Into<crate::claim::Body> for Analogy {
    fn into(self) -> crate::claim::Body {
        crate::claim::Body::Analogy(self)
    }
}

impl fmt::Display for Analogy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is in the category of {} ({})", self.left, self.right, self.confidence)
    }
}

impl Alledgable for Analogy {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let allegation = Claim::new(agent, crate::claim::Body::Analogy(self))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
