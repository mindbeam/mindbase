#[derive(Debug)]
pub enum Error {
    Mac(crypto_mac::MacError),
    Signature(ed25519_dalek::SignatureError),
    Bincode(bincode::Error),
    Store(mindbase_store::Error),
}

impl From<mindbase_store::Error> for Error {
    fn from(e: mindbase_store::Error) -> Self {
        Error::Store(e)
    }
}

impl From<crypto_mac::MacError> for Error {
    fn from(e: crypto_mac::MacError) -> Self {
        Error::Mac(e)
    }
}

impl From<ed25519_dalek::SignatureError> for Error {
    fn from(e: ed25519_dalek::SignatureError) -> Self {
        Error::Signature(e)
    }
}
impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::Bincode(e)
    }
}

// NFI why this impl can't seem to be found from mindbase_app
#[cfg(target_arch = "wasm32")]
impl std::convert::Into<wasm_bindgen::JsValue> for Error {
    fn into(self) -> wasm_bindgen::JsValue {
        format!("{:?}", self).into()
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{:?}", error))
    }
}
