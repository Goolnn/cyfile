mod asset;
mod error;

pub use asset::AssetSource;
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
        let array = value.as_array().ok_or(codec::Error::MismatchType {
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

pub fn field_as_str<S: AsRef<str>>(value: &Value, key: S) -> codec::Result<&str> {
    let field = field(value, key)?;

    field.as_str().ok_or(codec::Error::MismatchType {
        expected: "a string".to_string(),
        found: field.to_string(),
    })
}

pub fn field_as_f32<S: AsRef<str>>(value: &Value, key: S) -> codec::Result<f32> {
    let field = field(value, key)?;

    field
        .as_f64()
        .map(|v| v as f32)
        .ok_or(codec::Error::MismatchType {
            expected: "a number".to_string(),
            found: field.to_string(),
        })
}

pub fn field_as_f64<S: AsRef<str>>(value: &Value, key: S) -> codec::Result<f64> {
    let field = field(value, key)?;

    field.as_f64().ok_or(codec::Error::MismatchType {
        expected: "a number".to_string(),
        found: field.to_string(),
    })
}

pub fn field_as_codec<T: Codec, S: AsRef<str>>(
    manifest: &Manifest,
    value: &Value,
    key: S,
) -> codec::Result<T> {
    let field = field(value, key)?;

    T::decode(manifest, field)
}

fn field<S: AsRef<str>>(value: &Value, key: S) -> codec::Result<&Value> {
    let key = key.as_ref();

    match value.get(key) {
        Option::Some(val) => Ok(val),

        None => Err(codec::Error::MissingField {
            field: key.to_string(),
        }),
    }
}
