use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::file::Manifest;
use crate::project::Text;
use serde_json::Value;
use serde_json::json;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct Note {
    x: f32,
    y: f32,

    texts: Vec<Text>,
}

impl Codec for Note {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        match manifest.version {
            0 => Ok(json!({
                "x": self.x,
                "y": self.y,

                "texts": self.texts.encode(manifest)?,
            })),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        match reader.manifest().version {
            0 => Ok(Note {
                x: reader.read("x")?,
                y: reader.read("y")?,

                texts: reader.read("texts")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
