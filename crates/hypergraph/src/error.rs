#[derive(Debug)]
pub enum Error {
    Store(toboggan_kv::Error),
    Bincode(bincode::Error),
    Io(std::io::Error),
    NotFound,
    InvalidSlice,
}

impl From<toboggan_kv::Error> for Error {
    fn from(e: toboggan_kv::Error) -> Self {
        Self::Store(e)
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
