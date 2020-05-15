pub mod iter;

use crate::MBError;

pub(crate) trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub(crate) trait TryFromBytes
    where Self: Sized
{
    fn from_bytes(bytes: &[u8]) -> Result<Self, MBError>;
}

pub(crate) mod array64 {
    use crate::error::MBError;
    use serde::{
        Deserialize,
        Deserializer,
        Serializer,
    };

    pub(crate) fn try_64_from_slice<'a, T>(slice: &[T]) -> Result<&[T; 64], MBError> {
        if slice.len() == 64 {
            let ptr = slice.as_ptr() as *const [T; 64];
            unsafe { Ok(&*ptr) }
        } else {
            Err(MBError::TryFromSlice)
        }
    }

    pub(crate) fn ser_as_base64<S>(v: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use base64::STANDARD_NO_PAD;
        serializer.serialize_str(&base64::encode_config(&v[..], STANDARD_NO_PAD))
    }

    pub(crate) fn de_from_base64<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        match String::deserialize(deserializer) {
            Ok(string) => {
                let bytes = base64::decode(&string).map_err(|_| D::Error::custom("failed to decode base64"))?;
                let array: &[u8; 64] = try_64_from_slice::<u8>(&bytes[..]).map_err(|_| D::Error::custom("invalid length"))?;
                Ok(array.clone())
            },
            Err(_e) => Err(D::Error::custom("failed to deserialize")),
        }
        // .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        //                                  .map(|bytes| try_64_from_slice::<u8>(&bytes[..]).map(|v|v.clone())
        //                                  .and_then(|opt| opt.map_err(|_| ))
    }
}

pub(crate) mod serde_helper {
    use serde::{
        Deserialize,
        Deserializer,
        Serializer,
    };
    pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: AsRef<[u8]>,
              S: Serializer
    {
        use base64::STANDARD_NO_PAD;
        serializer.serialize_str(&base64::encode_config(v.as_ref(), STANDARD_NO_PAD))
    }

    pub fn from_base64_16<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use std::convert::TryInto;
        String::deserialize(deserializer).and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
                                         .map(|bytes| bytes[..].try_into())
                                         .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize")))
    }
    pub fn from_base64_32<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use std::convert::TryInto;
        String::deserialize(deserializer).and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
                                         .map(|bytes| bytes[..].try_into())
                                         .and_then(|opt| opt.map_err(|_| Error::custom("failed to deserialize")))
    }
}
