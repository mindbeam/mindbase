#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Artifact {
    Agent(AgentId),
    Url(Url),
    FlatText(Text),
    DataGraph(DataGraph),
    DataNode(DataNode),
}

impl Artifact {
    pub fn id(&self) -> ArtifactId {
        let mut hasher = Sha512Trunc256::new();

        // TODO 5 switch to CapnProto or similar. Artifact storage and wire representation should be identical
        // Therefore we should hash that
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        hasher.update(&encoded);

        let result = hasher.finalize();

        ArtifactId(result.into())
    }

    /// Might as well Serialize and hash in one go. Remove this when switching to CapnProto
    pub fn get_id_and_bytes(&self) -> (ArtifactId, Vec<u8>) {
        let mut hasher = Sha512Trunc256::new();

        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        hasher.update(&encoded);

        let result = hasher.finalize();

        (ArtifactId(result.into()), encoded)
    }
}

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent(_a) => unimplemented!(),
            Self::Url(_u) => unimplemented!(),
            Self::FlatText(t) => write!(f, "Artifact({})", t),
            Self::DataGraph(_d) => unimplemented!(),
            Self::DataNode(_n) => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Url {
    pub url: String,
}

impl Into<Artifact> for Url {
    fn into(self) -> Artifact {
        Artifact::Url(self)
    }
}

pub fn text(text: &str) -> Text {
    Text::new(text)
}

/// Text of nonspecific structure, origin, and language
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Text { text: text.to_string() }
    }

    pub fn string(text: String) -> Self {
        Text { text }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Into<Artifact> for Text {
    fn into(self) -> Artifact {
        Artifact::FlatText(self)
    }
}

impl Into<Artifact> for &str {
    fn into(self) -> Artifact {
        Artifact::FlatText(Text::new(self))
    }
}

// Allow the Agent to store arbitrary Graph of data, of an arbitrarily defined type.
// This can be used to store XML or JSON documents, or other application specific formats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataGraph {
    pub graph_type: Symbol,
    pub bytes: u32, // Optional
    /// Must contain all unreachable nodes. Optionally reachable nodes may be present
    pub nodes: Vec<ClaimId>,
}

impl Into<Artifact> for DataGraph {
    fn into(self) -> Artifact {
        Artifact::DataGraph(self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataNode {
    pub data_type: Symbol,
    pub data: Option<Vec<u8>>,
}

impl Into<Artifact> for DataNode {
    fn into(self) -> Artifact {
        Artifact::DataNode(self)
    }
}

impl Into<Artifact> for AgentId {
    fn into(self) -> Artifact {
        Artifact::Agent(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataNodeRelation {
    pub to: ClaimId,
    pub relation_type: Symbol,
}
