use crate::{
    agent::AgentId,
    error::MBError,
};

struct PolicyId {}

pub struct Policy {
    _id:   PolicyId,
    _body: PolicyBody,
}

impl Policy {
    pub fn new(_body: PolicyBody) -> Result<Policy, MBError> {
        unimplemented!()
    }
}

pub enum PolicyBody {
    // I want to use all your symbols during my ground symbol lookup process
    GroundSymbolAgent(AgentId),
    // I trust you thisss much? (About what?)
    TrustRelationship(AgentId, f32),
    // do I want to specify some sort of pattern?
    // Or do I want to attach the policy to my allegation context?
    DisclosureRelationship {
        // Symbol referring to an abstract group (the identity of the group itself. Not its members)
        // Allegations of Agent membership to this group can be changed by other Agents, but who?
        group:          Symbol,
        administrators: Symbol, //

        what: Symbol,
    },
}

use crate::{
    symbol::Symbol,
    util::AsBytes,
};

impl AsBytes for PolicyId {
    fn as_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::{
        Policy,
        PolicyBody,
    };
    use crate::prelude::{
        MindBase,
        Text,
    };

    fn _disclosure_relationship() -> Result<(), std::io::Error> {
        let tmpdir = tempfile::tempdir()?;
        let tmpdirpath = tmpdir.path();
        let mb = MindBase::open(&tmpdirpath)?;

        let my_agent = &mb.default_agent;

        let group = mb.alledge(Text::new("Authorized Members of Project Falcor"))?.subjective();
        let administrators = mb.alledge(my_agent)?.subjective();

        let what = mb.alledge(Text::new("Project Falcor Assets"))?.subjective();

        mb.add_policy(Policy::new(PolicyBody::DisclosureRelationship { group,
                                                                       administrators,
                                                                       what })?)?;
        Ok(())
    }
}
