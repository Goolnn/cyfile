mod error;

pub use error::Error;
pub use error::Result;

use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use std::marker::Sized;

pub trait Codec: Sized {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value>;

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self>;
}

impl<T: Codec> Codec for Vec<T> {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        let mut vec = Vec::with_capacity(self.len());

        for item in self {
            vec.push(item.encode(manifest)?);
        }

        Ok(Value::Array(vec))
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        let array = match value.as_array() {
            Some(val) => val,

            None => return Err(Error::ParseFailure),
        };

        let mut vec = Vec::with_capacity(array.len());

        for item in array {
            vec.push(T::decode(manifest, item)?);
        }

        Ok(vec)
    }
}
