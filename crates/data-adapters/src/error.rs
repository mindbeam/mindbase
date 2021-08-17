use mindbase_hypergraph::{traits::Weight, Entity};

#[derive(Debug)]
pub enum Error<W: Weight> {
    Hypergraph(mindbase_hypergraph::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
    NotFound,
    CycleDetected,
    Sanity,
    InvariantViolation(&'static str),
    SymbolResolution,
    MaterializationDeclined { entity: Entity<W>, reason: &'static str },
}

impl<'a, W: Weight> std::convert::From<Error<W>> for std::io::Error {
    fn from(error: Error<W>) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
impl<'a, W: Weight> From<mindbase_hypergraph::Error> for Error<W> {
    fn from(e: mindbase_hypergraph::Error) -> Self {
        Error::Hypergraph(e)
    }
}
impl<'a, W: Weight> From<serde_json::Error> for Error<W> {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl<'a, W: Weight> From<std::io::Error> for Error<W> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
