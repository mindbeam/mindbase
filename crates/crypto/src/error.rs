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
