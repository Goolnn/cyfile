use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::Text;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Note {
    x: f64,
    y: f64,

    choice: u32,

    texts: Vec<Text>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_coordinate(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    pub fn with_choice(mut self, choice: u32) -> Self {
        self.choice = choice;

        self
    }

    pub fn with_texts(mut self, texts: Vec<Text>) -> Self {
        self.texts = texts;

        self
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.texts.push(text);

        self
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_choice(&mut self, choice: u32) {
        self.choice = choice;
    }

    pub fn set_texts(&mut self, texts: Vec<Text>) {
        self.texts = texts;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn choice(&self) -> u32 {
        self.choice
    }

    pub fn texts_mut(&mut self) -> &mut Vec<Text> {
        &mut self.texts
    }

    pub fn texts(&self) -> &Vec<Text> {
        &self.texts
    }

    pub(crate) fn merge_texts(&self) -> (String, String) {
        let content = self
            .texts
            .iter()
            .filter_map(|text| {
                if !text.content().is_empty() {
                    Some(text.content())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join("\n\n");

        let comment = self
            .texts
            .iter()
            .filter_map(|text| {
                if !text.comment().is_empty() {
                    Some(text.comment())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join("\n\n");

        (content, comment)
    }
}

impl Decode for Note {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let x = reader.read_primitive()?;
        let y = reader.read_primitive()?;

        let choice = reader.read_primitive()?;

        let texts = reader.read_objects::<u32, Text>()?;

        Ok(Self {
            x,
            y,

            choice,

            texts,
        })
    }
}

impl Encode for Note {
    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.x)?;
        writer.write_primitive(self.y)?;

        writer.write_primitive(self.choice)?;

        writer.write_objects::<u32, Text>(self.texts())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Note;
    use crate::Text;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn new() {
        let note = Note::new();

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 0);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_coordinate() {
        let note = Note::new().with_coordinate(0.5, 0.5);

        assert_eq!(note.x(), 0.5);
        assert_eq!(note.y(), 0.5);

        assert_eq!(note.choice(), 0);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_choice() {
        let note = Note::new().with_choice(1);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 1);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_texts() {
        let note = Note::new().with_texts(vec![
            Text::new()
                .with_content("content_1")
                .with_comment("comment_1"),
            Text::new()
                .with_content("content_2")
                .with_comment("comment_2"),
            Text::new()
                .with_content("content_3")
                .with_comment("comment_3"),
        ]);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 0);

        assert_eq!(note.texts().len(), 3);

        assert_eq!(note.texts()[0].content(), "content_1");
        assert_eq!(note.texts()[0].comment(), "comment_1");

        assert_eq!(note.texts()[1].content(), "content_2");
        assert_eq!(note.texts()[1].comment(), "comment_2");

        assert_eq!(note.texts()[2].content(), "content_3");
        assert_eq!(note.texts()[2].comment(), "comment_3");
    }

    #[test]
    fn with_text() {
        let note = Note::new()
            .with_text(
                Text::new()
                    .with_content("content_1")
                    .with_comment("comment_1"),
            )
            .with_text(
                Text::new()
                    .with_content("content_2")
                    .with_comment("comment_2"),
            )
            .with_text(
                Text::new()
                    .with_content("content_3")
                    .with_comment("comment_3"),
            );

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 0);

        assert_eq!(note.texts().len(), 3);

        assert_eq!(note.texts()[0].content(), "content_1");
        assert_eq!(note.texts()[0].comment(), "comment_1");

        assert_eq!(note.texts()[1].content(), "content_2");
        assert_eq!(note.texts()[1].comment(), "comment_2");

        assert_eq!(note.texts()[2].content(), "content_3");
        assert_eq!(note.texts()[2].comment(), "comment_3");
    }

    #[test]
    fn set_x() {
        let mut note = Note::new();

        note.set_x(0.5);

        assert_eq!(note.x(), 0.5);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 0);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn set_y() {
        let mut note = Note::new();

        note.set_y(0.5);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.5);

        assert_eq!(note.choice(), 0);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn set_choice() {
        let mut note = Note::new();

        note.set_choice(1);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 1);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn set_texts() {
        let mut note = Note::new();

        note.set_texts(vec![
            Text::new()
                .with_content("content_1")
                .with_comment("comment_1"),
            Text::new()
                .with_content("content_2")
                .with_comment("comment_2"),
            Text::new()
                .with_content("content_3")
                .with_comment("comment_3"),
        ]);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.choice(), 0);

        assert_eq!(note.texts().len(), 3);

        assert_eq!(note.texts()[0].content(), "content_1");
        assert_eq!(note.texts()[0].comment(), "comment_1");

        assert_eq!(note.texts()[1].content(), "content_2");
        assert_eq!(note.texts()[1].comment(), "comment_2");

        assert_eq!(note.texts()[2].content(), "content_3");
        assert_eq!(note.texts()[2].comment(), "comment_3");
    }

    #[test]
    fn codec() {
        let note = Note::new()
            .with_coordinate(0.5, 0.5)
            .with_choice(1)
            .with_text(
                Text::new()
                    .with_content("content_1")
                    .with_comment("comment_1"),
            )
            .with_text(
                Text::new()
                    .with_content("content_2")
                    .with_comment("comment_2"),
            )
            .with_text(
                Text::new()
                    .with_content("content_3")
                    .with_comment("comment_3"),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&note).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_object::<Note>().unwrap(), note);
    }
}
