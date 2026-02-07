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

#[derive(Debug, Default)]
pub struct Project {
    cover: Option<Asset>,

    title: String,

    overview: String,

    pages: Vec<Page>,
}

impl Project {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cover(&self) -> Option<&Asset> {
        self.cover.as_ref()
    }

    pub fn set_cover(&mut self, cover: Option<Asset>) {
        self.cover = cover;
    }

    pub fn with_cover(mut self, cover: Asset) -> Self {
        self.cover = Some(cover);

        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title<T>(&mut self, title: T)
    where
        T: ToString,
    {
        self.title = title.to_string();
    }

    pub fn with_title<T>(mut self, title: T) -> Self
    where
        T: ToString,
    {
        self.title = title.to_string();

        self
    }

    pub fn overview(&self) -> &str {
        &self.overview
    }

    pub fn set_overview<T>(&mut self, overview: T)
    where
        T: ToString,
    {
        self.overview = overview.to_string();
    }

    pub fn with_overview<T>(mut self, overview: T) -> Self
    where
        T: ToString,
    {
        self.overview = overview.to_string();

        self
    }

    pub fn pages(&self) -> &Vec<Page> {
        &self.pages
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);

        self
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
