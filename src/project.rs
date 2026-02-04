mod note;
mod page;

pub use note::Note;
pub use page::Page;

use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use serde_json::json;

#[derive(Debug)]
pub struct Project {
    title: String,

    cover: String,

    pages: Vec<Page>,
}

impl Codec for Project {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        Ok(json!({
            "title": self.title,

            "cover": self.cover,

            "pages": self.pages.encode(manifest)?,
        }))
    }

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self> {
        let title = codec::field_as_str(value, "title")?.to_string();

        let cover = codec::field_as_str(value, "cover")?.to_string();

        let pages = codec::field_as_codec(manifest, value, "pages")?;

        Ok(Project {
            title,

            cover,

            pages,
        })
    }
}
