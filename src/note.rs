mod area;
mod text;

pub use area::Area;
pub use text::Text;

use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

/// Note is used to represent a marked position on an image with text information,
/// and can also specify a confirmation text.
///
/// You can create a new `Note` instance using `Note::new()`, and set the position
/// with the `with_position` method, set the confirmation text with the `with_comfirm`
/// method, set the text list with the `with_texts` method, or add a text with the
/// `with_text` method.
///
/// ```
/// use cyfile::Note;
/// use cyfile::Text;
///
/// let confirm = Text::new()
///     .with_content("Confirm Content")
///     .with_comment("Confirm Comment");
///
/// let text_1 = Text::new()
///     .with_content("Content 1")
///     .with_comment("Comment 1");
///
/// let text_2 = Text::new()
///     .with_content("Content 2")
///     .with_comment("Comment 2");
///
/// let note = Note::new()
///     .with_position(0.5, 0.5)
///     .with_comfirm(confirm.clone())
///     // Or use `with_texts` to set multiple texts at once
///     // .with_texts(vec![text_1.clone(), text_2.clone()]);
///     .with_text(text_1.clone())
///     .with_text(text_2.clone());
///
/// assert_eq!(note.x(), 0.5);
/// assert_eq!(note.y(), 0.5);
///
/// assert_eq!(note.comfirm(), Some(&confirm));
///
/// assert_eq!(note.texts(), &vec![text_1, text_2]);
/// ```
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Note {
    x: f64,
    y: f64,

    comfirm: Option<Text>,

    texts: Vec<Text>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    pub fn with_comfirm(mut self, comfirm: Text) -> Self {
        self.comfirm = Some(comfirm);

        self
    }

    pub fn with_texts(mut self, texts: Vec<Text>) -> Self {
        self.texts.extend(texts);

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

    pub fn set_comfirm(&mut self, comfirm: Option<Text>) {
        self.comfirm = comfirm;
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

    pub fn comfirm(&self) -> Option<&Text> {
        self.comfirm.as_ref()
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

impl Codec for Note {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> codec::Result<Self> {
        let x = reader.read_primitive()?;
        let y = reader.read_primitive()?;

        let comfirm = reader.read_object()?;

        let texts = reader.read_objects::<u32, Text>()?;

        Ok(Self {
            x,
            y,

            comfirm,

            texts,
        })
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> codec::Result<()> {
        writer.write_primitive(self.x)?;
        writer.write_primitive(self.y)?;

        writer.write_object(&self.comfirm)?;

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

        assert_eq!(note.comfirm(), None);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_position() {
        let note = Note::new().with_position(0.5, 0.5);

        assert_eq!(note.x(), 0.5);
        assert_eq!(note.y(), 0.5);

        assert_eq!(note.comfirm(), None);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn with_comfirm() {
        let comfirm = Text::new();
        let note = Note::new().with_comfirm(comfirm.clone());

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.comfirm(), Some(&comfirm));

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

        assert_eq!(note.comfirm(), None);

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

        assert_eq!(note.comfirm(), None);

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

        assert_eq!(note.comfirm(), None);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn set_y() {
        let mut note = Note::new();

        note.set_y(0.5);

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.5);

        assert_eq!(note.comfirm(), None);

        assert!(note.texts().is_empty());
    }

    #[test]
    fn set_comfirm() {
        let mut note = Note::new();
        let comfirm = Text::new();

        note.set_comfirm(Some(comfirm.clone()));

        assert_eq!(note.x(), 0.0);
        assert_eq!(note.y(), 0.0);

        assert_eq!(note.comfirm(), Some(&comfirm));

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

        assert_eq!(note.comfirm(), None);

        assert_eq!(note.texts().len(), 3);

        assert_eq!(note.texts()[0].content(), "content_1");
        assert_eq!(note.texts()[0].comment(), "comment_1");

        assert_eq!(note.texts()[1].content(), "content_2");
        assert_eq!(note.texts()[1].comment(), "comment_2");

        assert_eq!(note.texts()[2].content(), "content_3");
        assert_eq!(note.texts()[2].comment(), "comment_3");
    }

    #[test]
    fn codec() -> anyhow::Result<()> {
        let note = Note::new()
            .with_position(0.5, 0.5)
            .with_comfirm(
                Text::new()
                    .with_content("comfirm_content")
                    .with_comment("comfirm_comment"),
            )
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

        writer.write_object(&note)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_object::<Note>()?, note);

        Ok(())
    }
}
