#[derive(Debug)]
pub enum Error {
    Mac(crypto_mac::MacError),
    Signature(ed25519_dalek::SignatureError),
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

// NFI why this impl can't seem to be found from mindbase_app
#[cfg(target_arch = "wasm32")]
impl std::convert::Into<wasm_bindgen::JsValue> for Error {
    fn into(self) -> wasm_bindgen::JsValue {
        format!("{:?}", self).into()
    }
}
