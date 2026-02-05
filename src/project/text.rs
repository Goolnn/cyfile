use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::file::Manifest;
use serde_json::Value;
use serde_json::json;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct Text {
    content: String,
    comment: String,
}

impl Codec for Text {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        match manifest.version {
            0 => Ok(json!({
                "content": self.content,
                "comment": self.comment,
            })),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        match reader.manifest().version {
            0 => Ok(Text {
                content: reader.read("content")?,
                comment: reader.read("comment")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
