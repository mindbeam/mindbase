#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    Bincode(bincode::Error),
    IoError(std::io::Error),
    AgentHandleNotFound,
    SignatureError,
    Base64Error,
    ClaimNotFound,
    // MBQL(Box<mindbase_mbql::error::MBQLError>),
    TraversalFailed,
    UnboundSymbol,
    SymbolVarNotFound,
    SymbolVarAlreadyBound,
    NullSymbol,
    Other,
    Store(mindbase_store::Error),
    Util(mindbase_util::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<MBQLError> for Error {
//     fn from(e: MBQLError) -> Self {
//         Self::MBQL(Box::new(e))
//     }
// }

impl From<mindbase_store::Error> for Error {
    fn from(e: mindbase_store::Error) -> Self {
        Self::Store(e)
    }
}

impl From<mindbase_util::Error> for Error {
    fn from(e: mindbase_util::Error) -> Self {
        Self::Util(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}
impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::Bincode(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
// impl Into<std::io::Error> for Error {
//     fn into(self) -> std::io::Error {
//         use std::io::ErrorKind;

//         std::io::Error::new(ErrorKind::Other, format!("{:?}", self))
//     }
// }
