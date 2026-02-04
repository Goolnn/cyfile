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
        let mut array = Vec::with_capacity(self.len());

        for item in self {
            array.push(item.encode(manifest)?);
        }

        Ok(Value::Array(array))
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        let array = value.as_array().ok_or(codec::Error::TypeMismatch {
            expected: "an array".to_string(),
            found: value.to_string(),
        })?;

        let mut vec = Vec::with_capacity(array.len());

        for item in array {
            vec.push(T::decode(manifest, item)?);
        }

        Ok(vec)
    }
}
