#[derive(Debug)]
pub enum Error {
    Hypergraph(mindbase_hypergraph::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
    NotFound,
    CycleDetected,
    Sanity,
    InvariantViolation(&'static str),
    SymbolResolution,
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
impl From<mindbase_hypergraph::Error> for Error {
    fn from(e: mindbase_hypergraph::Error) -> Self {
        Error::Hypergraph(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
