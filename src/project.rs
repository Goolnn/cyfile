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
    cover: Option<Asset>,

    title: String,

    overview: String,

    pages: Vec<Page>,
}

impl Project {
    pub fn cover(&self) -> Option<&Asset> {
        self.cover.as_ref()
    }

    pub fn set_cover(&mut self, cover: Option<Asset>) {
        self.cover = cover;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title<T: ToString>(&mut self, title: T) {
        self.title = title.to_string();
    }

    pub fn overview(&self) -> &str {
        &self.overview
    }

    pub fn set_overview<T: ToString>(&mut self, overview: T) {
        self.overview = overview.to_string();
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn pages_mut(&mut self) -> &mut [Page] {
        &mut self.pages
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }
}

impl Codec for Project {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.field("cover", &self.cover)?;

        writer.field("title", &self.title)?;

        writer.field("overview", &self.overview)?;

        writer.field("pages", &self.pages)?;

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Project {
            cover: reader.field("cover")?,

            title: reader.field("title")?,

            overview: reader.field("overview")?,

            pages: reader.field("pages")?,
        })
    }
}
