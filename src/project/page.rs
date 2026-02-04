use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use crate::project::Note;
use serde_json::Value;
use serde_json::json;

#[derive(Debug)]
pub struct Page {
    image: String,

    notes: Vec<Note>,
}

impl Codec for Page {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        Ok(json!({
            "image": self.image,

            "notes": self.notes.encode(manifest)?,
        }))
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        let image = codec::field_as_str(value, "image")?.to_string();

        let notes = codec::field_as_codec(manifest, value, "notes")?;

        Ok(Page { image, notes })
    }
}
