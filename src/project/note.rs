use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use crate::project::Text;
use serde_json::Value;
use serde_json::json;

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

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        match manifest.version {
            0 => {
                let x = codec::field_as_f32(value, "x")?;
                let y = codec::field_as_f32(value, "y")?;

                let texts = codec::field_as_codec(manifest, value, "texts")?;

                Ok(Note { x, y, texts })
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
