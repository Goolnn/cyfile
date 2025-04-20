use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Result;
use crate::codec::Writer;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

/// Text is a basic component of project files, used to store text content and comments.
/// The content field is used to save the content of the translated text, and the comment
/// field is used to save the comments of the translated text.
///
/// To create a text object, you can use the default constructor `Text::new()`, and then
/// use the `with_content()` and `with_comment()` methods to set the text content and
/// comments.
///
/// ```
/// use cyfile::Text;
///
/// let text = Text::new()
///     .with_content("Content")
///     .with_comment("Comment");
///
/// assert_eq!(text.content(), "Content");
/// assert_eq!(text.comment(), "Comment");
/// ```
#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Text {
    content: String,
    comment: String,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();

        self
    }

    pub fn with_comment(mut self, comment: impl ToString) -> Self {
        self.comment = comment.to_string();

        self
    }

    pub fn set_content(&mut self, content: impl ToString) {
        self.content = content.to_string();
    }

    pub fn set_comment(&mut self, comment: impl ToString) {
        self.comment = comment.to_string();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

impl Codec for Text {
    fn decode<S>(reader: &mut Reader<S>) -> Result<Self>
    where
        S: Read + Seek,
    {
        Ok(Self {
            content: reader.read_string_with_len::<u32>()?,
            comment: reader.read_string_with_len::<u32>()?,
        })
    }

    fn encode<S>(&self, writer: &mut Writer<S>) -> Result<()>
    where
        S: Write + Seek,
    {
        writer.write_string_with_len::<u32>(&self.content)?;
        writer.write_string_with_len::<u32>(&self.comment)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Text;
    use std::io::Cursor;
    use std::io::Seek;

    #[test]
    fn new() {
        let text = Text::new();

        assert!(text.content().is_empty());
        assert!(text.comment().is_empty());
    }

    #[test]
    fn with_content() {
        let text = Text::new().with_content("Content");

        assert_eq!(text.content(), "Content");
        assert!(text.comment().is_empty());
    }

    #[test]
    fn with_comment() {
        let text = Text::new().with_comment("Comment");

        assert!(text.content().is_empty());
        assert_eq!(text.comment(), "Comment");
    }

    #[test]
    fn set_content() {
        let mut text = Text::new().with_content("Content").with_comment("Comment");

        text.set_content("New content");

        assert_eq!(text.content(), "New content");
        assert_eq!(text.comment(), "Comment");
    }

    #[test]
    fn set_comment() {
        let mut text = Text::new().with_content("Content").with_comment("Comment");

        text.set_comment("New comment");

        assert_eq!(text.content(), "Content");
        assert_eq!(text.comment(), "New comment");
    }

    #[test]
    fn codec() {
        let text = Text::new().with_content("Content").with_comment("Comment");

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&text).unwrap();
        writer.rewind().unwrap();

        let mut reader = Reader::new(writer.into_inner());

        assert_eq!(reader.read_object::<Text>().unwrap(), text);
    }
}
