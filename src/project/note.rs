use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use serde_json::json;

#[derive(Debug)]
pub struct Note {
    x: f32,
    y: f32,
}

impl Codec for Note {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        match manifest.version {
            0 => Ok(json!({
                "x": self.x,
                "y": self.y,
            })),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        match manifest.version {
            0 => {
                let x = codec::field_as_f32(value, "x")?;
                let y = codec::field_as_f32(value, "y")?;

                Ok(Note { x, y })
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
