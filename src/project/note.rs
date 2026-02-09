use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::project::Text;

#[derive(Debug, Default)]
pub struct Note {
    x: f32,
    y: f32,

    texts: Vec<Text>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;

        self
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;

        self
    }

    pub fn texts(&self) -> &Vec<Text> {
        &self.texts
    }

    pub fn texts_mut(&mut self) -> &mut Vec<Text> {
        &mut self.texts
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.texts.push(text);

        self
    }
}

impl Codec for Note {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        match writer.manifest().version() {
            0 => {
                writer.field("x", &self.x)?;
                writer.field("y", &self.y)?;

                writer.field("texts", &self.texts)?;

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        match reader.manifest().version() {
            0 => Ok(Note {
                x: reader.field("x")?,
                y: reader.field("y")?,

                texts: reader.field("texts")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Codec;
    use crate::Note;
    use crate::Text;
    use crate::codec::Writer;
    use crate::file::Manifest;
    use serde_json::json;

    #[test]
    fn new() {
        let note = Note::new();

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_position() {
        let note = Note::new().with_position(1.0, 2.0);

        assert_eq!(note.x(), 1.0);
        assert_eq!(note.y(), 2.0);
    }

    #[test]
    fn with_x() {
        let note = Note::new().with_x(1.0);

        assert_eq!(note.x(), 1.0);
        assert_eq!(note.y(), 0.0);
    }

    #[test]
    fn with_y() {
        let note = Note::new().with_y(2.0);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 2.0);
    }

    #[test]
    fn with_texts() {
        let text1 = Text::new().with_content("This is a content 1.");
        let text2 = Text::new().with_content("This is a content 2.");
        let text3 = Text::new().with_content("This is a content 3.");

        let note = Note::new()
            .with_text(text1.clone())
            .with_text(text2.clone())
            .with_text(text3.clone());

        assert_eq!(note.texts().len(), 3);

        assert_eq!(note.texts()[0].content(), "This is a content 1.");
        assert_eq!(note.texts()[1].content(), "This is a content 2.");
        assert_eq!(note.texts()[2].content(), "This is a content 3.");
    }

    #[test]
    fn set_position() {
        let mut note = Note::new();

        note.set_position(1.0, 2.0);

        assert_eq!(note.x(), 1.0);
        assert_eq!(note.y(), 2.0);
    }

    #[test]
    fn set_x() {
        let mut note = Note::new();

        note.set_x(1.0);

        assert_eq!(note.x(), 1.0);
        assert_eq!(note.y(), 0.0);
    }

    #[test]
    fn set_y() {
        let mut note = Note::new();

        note.set_y(2.0);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 2.0);
    }

    #[test]
    fn set_x_and_y() {
        let mut note = Note::new();

        note.set_x(1.0);
        note.set_y(2.0);

        assert_eq!(note.x(), 1.0);
        assert_eq!(note.y(), 2.0);
    }

    #[test]
    fn encode() {
        let text1 = Text::new().with_content("This is a content 1.");
        let text2 = Text::new().with_content("This is a content 2.");
        let text3 = Text::new().with_content("This is a content 3.");

        let note = Note::new()
            .with_position(1.0, 2.0)
            .with_text(text1.clone())
            .with_text(text2.clone())
            .with_text(text3.clone());

        let manifest = Manifest::default();

        let mut writer = Writer::new(&manifest);

        assert!(Codec::encode(&note, &mut writer).is_ok());

        let value = writer.into_value();

        assert_eq!(
            value,
            json!({
                "x": 1.0,
                "y": 2.0,

                "texts": [
                    {
                        "content": "This is a content 1.",
                        "comment": ""
                    },
                    {
                        "content": "This is a content 2.",
                        "comment": ""
                    },
                    {
                        "content": "This is a content 3.",
                        "comment": ""
                    }
                ]
            })
        );
    }
}
