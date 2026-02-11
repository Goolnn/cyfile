use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::project::Asset;
use crate::project::Note;

#[derive(Debug, Default)]
pub struct Page {
    image: Option<Asset>,

    notes: Vec<Note>,
}

impl Page {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image(&self) -> Option<&Asset> {
        self.image.as_ref()
    }

    pub fn set_image(&mut self, image: Option<Asset>) {
        self.image = image;
    }

    pub fn with_image(mut self, image: Asset) -> Self {
        self.image = Some(image);

        self
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn notes_mut(&mut self) -> &mut Vec<Note> {
        &mut self.notes
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes.push(note);

        self
    }
}

impl Codec for Page {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        match writer.manifest().version() {
            0 => {
                writer.field("image", &self.image)?;

                writer.field("notes", &self.notes)?;

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        match reader.manifest().version() {
            0 => Ok(Page {
                image: reader.field("image")?,

                notes: reader.field("notes")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Asset;
    use crate::Codec;
    use crate::Note;
    use crate::Page;
    use crate::Text;
    use crate::codec::Writer;
    use crate::file::Manifest;
    use serde_json::json;

    #[test]
    fn new() {
        let asset = Asset::new("path/to/image.png", Vec::new());

        let page = Page::new().with_image(asset);

        match page.image() {
            Some(image) => assert_eq!(image.path(), "path/to/image.png"),
            None => panic!("Expected image, found None"),
        }

        assert!(page.notes().is_empty());
    }

    #[test]
    fn with_notes() {
        let asset = Asset::new("path/to/image.png", Vec::new());

        let note1 = Note::new().with_text(Text::new().with_content("This is a note1."));
        let note2 = Note::new().with_text(Text::new().with_content("This is a note2."));
        let note3 = Note::new().with_text(Text::new().with_content("This is a note3."));

        let page = Page::new()
            .with_image(asset)
            .with_note(note1.clone())
            .with_note(note2.clone())
            .with_note(note3.clone());

        assert_eq!(page.notes().len(), 3);

        assert_eq!(page.notes()[0].texts()[0].content(), "This is a note1.");
        assert_eq!(page.notes()[1].texts()[0].content(), "This is a note2.");
        assert_eq!(page.notes()[2].texts()[0].content(), "This is a note3.");
    }

    #[test]
    fn set_image() {
        let asset1 = Asset::new("path/to/image1.png", Vec::new());
        let asset2 = Asset::new("path/to/image2.png", Vec::new());

        let mut page = Page::new().with_image(asset1);

        match page.image() {
            Some(image) => assert_eq!(image.path(), "path/to/image1.png"),
            None => panic!("Expected image, found None"),
        }

        page.set_image(Some(asset2));

        match page.image() {
            Some(image) => assert_eq!(image.path(), "path/to/image2.png"),
            None => panic!("Expected image, found None"),
        }
    }

    #[test]
    fn encode() {
        let asset = Asset::new("path/to/image.png", Vec::new());

        let note1 = Note::new().with_text(Text::new().with_content("This is a note1."));
        let note2 = Note::new().with_text(Text::new().with_content("This is a note2."));
        let note3 = Note::new().with_text(Text::new().with_content("This is a note3."));

        let page = Page::new()
            .with_image(asset)
            .with_note(note1.clone())
            .with_note(note2.clone())
            .with_note(note3.clone());

        let manifest = Manifest::default();

        let mut writer = Writer::new(&manifest);

        assert!(Codec::encode(&page, &mut writer).is_ok());

        let (_, value) = writer.end();

        assert_eq!(
            value,
            json!({
                "image": "path/to/image.png",
                "notes": [
                    {
                        "x": 0.0,
                        "y": 0.0,
                        "texts": [
                            {
                                "content": "This is a note1.",
                                "comment": "",
                            }
                        ],
                    },
                    {
                        "x": 0.0,
                        "y": 0.0,
                        "texts": [
                            {
                                "content": "This is a note2.",
                                "comment": "",
                            }
                        ],
                    },
                    {
                        "x": 0.0,
                        "y": 0.0,
                        "texts": [
                            {
                                "content": "This is a note3.",
                                "comment": "",
                            }
                        ],
                    },
                ],
            }),
        );
    }
}
