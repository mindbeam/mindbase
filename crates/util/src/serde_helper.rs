use serde::{Deserialize, Deserializer, Serializer};
pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    use base64::STANDARD_NO_PAD;
    serializer.serialize_str(&base64::encode_config(v.as_ref(), STANDARD_NO_PAD))
}

pub fn from_base64_16<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use std::convert::TryInto;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| bytes[..].try_into())
        .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize")))
}
pub fn from_base64_32<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use std::convert::TryInto;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .map(|bytes| bytes[..].try_into())
        .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize")))
}
