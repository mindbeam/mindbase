// TODO 1 - LEFT OFF HERE - is it desirable to directly return a symbol and not Claim?
pub trait Symbolize: std::fmt::Debug {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError>;
}

impl Symbolize for &ArtifactId {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let allegation = Claim::new(agent, crate::claim::Body::Artifact(self.clone()))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
impl Symbolize for ArtifactId {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let allegation = Claim::new(agent, crate::claim::Body::Artifact(self))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}

impl Symbolize for &AgentId {
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let artifact_id = mb.put_artifact(self.id())?;
        let allegation = Claim::new(agent, crate::claim::Body::Artifact(artifact_id))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
impl<T> Symbolize for T
where
    T: Into<Artifact> + std::fmt::Debug,
{
    fn alledge(self, mb: &MindBase, agent: &Agent) -> Result<Claim, MBError> {
        let artifact_id = mb.put_artifact(self)?;
        let allegation = Claim::new(agent, crate::claim::Body::Artifact(artifact_id))?;
        mb.put_allegation(&allegation)?;
        Ok(allegation)
    }
}
