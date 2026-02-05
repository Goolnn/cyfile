mod asset;
mod error;
mod reader;

pub use asset::AssetSource;
pub use error::Error;
pub use error::Result;
pub use reader::Reader;

use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use std::io::Read;
use std::io::Seek;
use std::marker::Sized;

pub trait Codec: Sized {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value>;

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek;
}

impl Codec for String {
    fn encode(&self, _manifest: &Manifest) -> codec::Result<Value> {
        todo!()
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        reader
            .value()
            .as_str()
            .map(|s| s.to_string())
            .ok_or(codec::Error::MismatchType {
                expected: "a string".to_string(),
                found: reader.value().to_string(),
            })
    }
}

impl Codec for f32 {
    fn encode(&self, _manifest: &Manifest) -> codec::Result<Value> {
        todo!()
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        reader
            .value()
            .as_f64()
            .map(|v| v as f32)
            .ok_or(codec::Error::MismatchType {
                expected: "a number".to_string(),
                found: reader.value().to_string(),
            })
    }
}

impl Codec for f64 {
    fn encode(&self, _manifest: &Manifest) -> codec::Result<Value> {
        todo!()
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        reader.value().as_f64().ok_or(codec::Error::MismatchType {
            expected: "a number".to_string(),
            found: reader.value().to_string(),
        })
    }
}

impl<T> Codec for Vec<T>
where
    T: Codec,
{
    fn encode(&self, _manifest: &Manifest) -> codec::Result<Value> {
        todo!()
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        let array = reader
            .value()
            .as_array()
            .ok_or(codec::Error::MismatchType {
                expected: "an array".to_string(),
                found: reader.value().to_string(),
            })?;

        let mut vec = Vec::with_capacity(array.len());

        for item in array {
            vec.push(T::decode(reader.at(item))?);
        }

        Ok(vec)
    }
}
