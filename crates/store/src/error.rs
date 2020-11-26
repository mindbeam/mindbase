#[derive(Debug)]
pub enum Error {
    #[cfg(not(target_arch = "wasm32"))]
    Sled(sled::Error),
}

#[cfg(not(target_arch = "wasm32"))]
impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Self::Sled(e)
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
