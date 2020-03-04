#[derive(Debug)]
pub enum Error {
    Sled(sled::Error),
    SerdeJson(serde_json::Error),
    Bincode(bincode::Error),
    IoError(std::io::Error),
    AgentHandleNotFound,
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

impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        use std::io::ErrorKind;

        std::io::Error::new(ErrorKind::Other, format!("{:?}", self))
    }
}
