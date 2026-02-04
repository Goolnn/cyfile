use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use serde_json::json;

#[derive(Debug)]
pub struct Text {
    context: String,
    comment: String,
}

impl Codec for Text {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        match manifest.version {
            0 => Ok(json!({
                "context": self.context,
                "comment": self.comment,
            })),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        match manifest.version {
            0 => {
                let context = codec::field_as_str(value, "context")?.to_string();
                let comment = codec::field_as_str(value, "comment")?.to_string();

                Ok(Text { context, comment })
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
