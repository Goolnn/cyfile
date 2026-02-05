use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::file::Manifest;
use crate::project::Asset;
use crate::project::Note;
use serde_json::Value;
use serde_json::json;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct Page {
    image: Asset,

    notes: Vec<Note>,
}

impl Codec for Page {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        match manifest.version {
            0 => Ok(json!({
                "image": self.image.encode(manifest)?,

                "notes": self.notes.encode(manifest)?,
            })),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        match reader.manifest().version {
            0 => Ok(Page {
                image: reader.read("image")?,
                notes: reader.read("notes")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
