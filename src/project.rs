mod asset;
mod note;
mod page;
mod text;

pub use asset::Asset;
pub use note::Note;
pub use page::Page;
pub use text::Text;

use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::file::Manifest;
use serde_json::Value;
use serde_json::json;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct Project {
    title: String,

    cover: Asset,

    pages: Vec<Page>,
}

impl Project {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn cover(&self) -> &Asset {
        &self.cover
    }

    pub fn cover_mut(&mut self) -> &mut Asset {
        &mut self.cover
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn pages_mut(&mut self) -> &mut [Page] {
        &mut self.pages
    }
}

impl Codec for Project {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value> {
        Ok(json!({
            "title": self.title,

            "cover": self.cover.encode(manifest)?,

            "pages": self.pages.encode(manifest)?,
        }))
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        match reader.manifest().version {
            0 => Ok(Project {
                title: reader.read("title")?,

                cover: reader.read("cover")?,

                pages: reader.read("pages")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}
