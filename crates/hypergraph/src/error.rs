#[derive(Debug)]
pub enum Error {
    Store(mindbase_store::Error),
    ArtifactNotFound,
}

impl From<mindbase_store::Error> for Error {
    fn from(e: mindbase_store::Error) -> Self {
        Self::Store(e)
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
