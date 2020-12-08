pub enum Error {
    Store(mindbase_store::Error),
}

impl From<mindbase_store::Error> for Error {
    fn from(e: mindbase_store::Error) -> Self {
        Self::Store(e)
    }
}
