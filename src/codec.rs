mod asset;
mod error;
mod reader;
mod writer;

pub use asset::ArchiveSource;
pub use asset::AssetSource;
pub use error::Error;
pub use error::Result;
pub use reader::Reader;
pub use writer::Writer;

use crate::codec;
use serde_json::Value;
use std::marker::Sized;

pub trait Codec: Sized {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()>;
    fn decode(reader: &Reader) -> codec::Result<Self>;
}

impl Codec for String {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.value(self.to_string());

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(reader
            .value()
            .as_str()
            .ok_or(codec::Error::MismatchType {
                expected: "a string".to_string(),
                found: reader.value().to_string(),
            })?
            .to_string())
    }
}

impl Codec for f32 {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.value(*self as f64);

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(reader.value().as_f64().ok_or(codec::Error::MismatchType {
            expected: "f32".to_string(),
            found: reader.value().to_string(),
        })? as f32)
    }
}

impl Codec for Value {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.value(self.clone());

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(reader.value().clone())
    }
}

impl<T> Codec for Vec<T>
where
    T: Codec,
{
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.value(
            self.iter()
                .map(|item| {
                    let mut writer = writer.clone();

                    Codec::encode(item, &mut writer)?;

                    Ok(writer.into_value())
                })
                .collect::<codec::Result<Vec<Value>>>()?,
        );

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        reader
            .value()
            .as_array()
            .ok_or(codec::Error::MismatchType {
                expected: "an array".to_string(),
                found: reader.value().to_string(),
            })?
            .iter()
            .map(|item| T::decode(&reader.clone(item)))
            .collect()
    }
}
