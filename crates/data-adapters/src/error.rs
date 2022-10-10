use mindbase_hypergraph::{
    traits::{TSymbol, TValue},
    Entity,
};

#[derive(Debug)]
pub enum Error<Sym: TSymbol, Val: TValue> {
    Hypergraph(mindbase_hypergraph::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
    NotFound,
    CycleDetected,
    Sanity,
    InvariantViolation(&'static str),
    SymbolResolution,
    MaterializationDeclined { entity: Entity<Sym, Val>, reason: &'static str },
}

impl<'a, Sym: TSymbol, Val: TValue> std::convert::From<Error<Sym, Val>> for std::io::Error {
    fn from(error: Error<Sym, Val>) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
impl<'a, Sym: TSymbol, Val: TValue> From<mindbase_hypergraph::Error> for Error<Sym, Val> {
    fn from(e: mindbase_hypergraph::Error) -> Self {
        Error::Hypergraph(e)
    }
}
impl<'a, Sym: TSymbol, Val: TValue> From<serde_json::Error> for Error<Sym, Val> {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl<'a, Sym: TSymbol, Val: TValue> From<std::io::Error> for Error<Sym, Val> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
