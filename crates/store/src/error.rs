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
