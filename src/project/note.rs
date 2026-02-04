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
    fn encode(&self, _: &Manifest) -> codec::Result<Value> {
        Ok(json!({
            "x": self.x,
            "y": self.y,
        }))
    }

    fn decode(_: &Manifest, value: &Value) -> codec::Result<Self> {
        let x = codec::field_as_f32(value, "x")?;
        let y = codec::field_as_f32(value, "y")?;

        Ok(Note { x, y })
    }
}
