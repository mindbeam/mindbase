use crate::mbql::error::Error as MBQLError;
#[derive(Debug)]
pub enum Error {
    Sled(sled::Error),
    SerdeJson(serde_json::Error),
    Bincode(bincode::Error),
    IoError(std::io::Error),
    AgentHandleNotFound,
    SignatureError,
    TryFromSlice,
    Base64Error,
    AllegationNotFound,
    MBQL(MBQLError),
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.kind {
//             MBQLErrorKind::IOError { error } => f.write_fmt(format_args!("MBQL IO Error: {}", error)),
//         }
//     }
// }

impl From<MBQLError> for Error {
    fn from(e: MBQLError) -> Self {
        Self::MBQL(e)
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Self::Sled(e)
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
