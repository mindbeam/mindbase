use crate::{
    allegation::AllegationId,
    error::Error,
    MindBase,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;

/// Pointer to a region within Semantic/Knowledge-Space
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Concept {
    // # how would the agent know which entities they are referring to?
    // # I suppose the UI could remember a list of entities which are being
    // # converged in the rendering. Not really in love with the fact that
    // # the Agent has to pick specific entities as a representative sample
    // # Of the cluster they're actually referring to, but it will suffice
    // # for now I think.
    /// A list of entities which serve as a representative sample of the K-Space cluster
    pub members:       Vec<AllegationId>,
    pub spread_factor: f32,
    /* # Here's a slightly different way, but still not great
     * # median_entity: Entity,
     * # radius: Float */
}

impl fmt::Display for Concept {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.members.iter().map(|e| format!("{}", e)).collect();
        write!(f, "[{}]", parts.join(","))
    }
}

impl Concept {
    pub fn is_subjective(&self, mb: &MindBase) -> Result<bool, Error> {
        unimplemented!()
    }

    pub fn is_intersubjective(&self, mb: &MindBase) -> Result<bool, Error> {
        unimplemented!()
    }
}
