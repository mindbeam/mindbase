use crate::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn try_64_from_slice<'a, T>(slice: &[T]) -> Result<&[T; 64], Error> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 64];
        unsafe { Ok(&*ptr) }
    } else {
        Err(Error::TryFromSlice)
    }
}

pub fn ser_as_base64<S>(v: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use base64::STANDARD_NO_PAD;
    serializer.serialize_str(&base64::encode_config(&v[..], STANDARD_NO_PAD))
}

pub fn de_from_base64<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    match String::deserialize(deserializer) {
        Ok(string) => {
            let bytes = base64::decode(&string).map_err(|_| D::Error::custom("failed to decode base64"))?;
            let array: &[u8; 64] = try_64_from_slice::<u8>(&bytes[..]).map_err(|_| D::Error::custom("invalid length"))?;
            Ok(array.clone())
        }
        Err(_e) => Err(D::Error::custom("failed to deserialize")),
    }
    // .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
    //                                  .map(|bytes| try_64_from_slice::<u8>(&bytes[..]).map(|v|v.clone())
    //                                  .and_then(|opt| opt.map_err(|_| ))
}
