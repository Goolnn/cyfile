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
        match writer.manifest().version() {
            0 => {
                writer.field("cover", &self.cover)?;

                writer.field("title", &self.title)?;

                writer.field("overview", &self.overview)?;

                writer.field("pages", &self.pages)?;

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        match reader.manifest().version() {
            0 => Ok(Project {
                cover: reader.field("cover")?,

                title: reader.field("title")?,

                overview: reader.field("overview")?,

                pages: reader.field("pages")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Asset;
    use crate::Codec;
    use crate::Page;
    use crate::Project;
    use crate::codec::Writer;
    use crate::file::Manifest;

    #[test]
    fn new() {
        let project = Project::new();

        assert!(project.cover().is_none());

        assert!(project.title().is_empty());

        assert!(project.overview().is_empty());

        assert!(project.pages().is_empty());
    }

    #[test]
    fn with_cover() {
        let data = vec![0, 1, 2, 3];

        let asset = Asset::new("cover.png", data);

        let project = Project::new().with_cover(asset);

        match project.cover() {
            Some(cover) => assert_eq!(cover.path(), "cover.png"),

            None => panic!("Cover should be set."),
        }
    }

    #[test]
    fn with_title() {
        let project = Project::new().with_title("Project Title");

        assert_eq!(project.title(), "Project Title");
    }

    #[test]
    fn with_overview() {
        let project = Project::new().with_overview("This is an overview.");

        assert_eq!(project.overview(), "This is an overview.");
    }

    #[test]
    fn with_pages() {
        let page1 = Page::new().with_image(Asset::new("image1.png", Vec::new()));
        let page2 = Page::new().with_image(Asset::new("image2.png", Vec::new()));
        let page3 = Page::new().with_image(Asset::new("image3.png", Vec::new()));

        let project = Project::new()
            .with_page(page1)
            .with_page(page2)
            .with_page(page3);

        assert_eq!(project.pages().len(), 3);

        match project.pages()[0].image() {
            Some(image) => assert_eq!(image.path(), "image1.png"),
            None => panic!("Expected image, found None"),
        }

        match project.pages()[1].image() {
            Some(image) => assert_eq!(image.path(), "image2.png"),
            None => panic!("Expected image, found None"),
        }

        match project.pages()[2].image() {
            Some(image) => assert_eq!(image.path(), "image3.png"),
            None => panic!("Expected image, found None"),
        }
    }

    #[test]
    fn encode() {
        let page1 = Page::new().with_image(Asset::new("image1.png", Vec::new()));
        let page2 = Page::new().with_image(Asset::new("image2.png", Vec::new()));
        let page3 = Page::new().with_image(Asset::new("image3.png", Vec::new()));

        let project = Project::new()
            .with_title("Project Title")
            .with_overview("This is an overview.")
            .with_page(page1)
            .with_page(page2)
            .with_page(page3);

        let manifest = Manifest::default();

        let mut writer = Writer::new(&manifest);

        assert!(Codec::encode(&project, &mut writer).is_ok());

        let (_, value) = writer.end();

        assert_eq!(
            value,
            serde_json::json!({
                "cover": null,
                "title": "Project Title",
                "overview": "This is an overview.",
                "pages": [
                    {
                        "image": "image1.png",
                        "notes": []
                    },
                    {
                        "image": "image2.png",
                        "notes": []
                    },
                    {
                        "image": "image3.png",
                        "notes": []
                    },
                ],
            })
        );
    }
}
