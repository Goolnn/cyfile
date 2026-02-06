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
use crate::codec::Writer;

#[derive(Debug)]
pub struct Project {
    title: String,

    cover: Asset,

    pages: Vec<Page>,
}

impl Codec for Project {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.field("title", &self.title)?;

        writer.field("cover", &self.cover)?;

        writer.field("pages", &self.pages)?;

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Project {
            title: reader.field("title")?,

            cover: reader.field("cover")?,

            pages: reader.field("pages")?,
        })
    }
}
