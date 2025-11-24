use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::Date;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

/// Text is a basic component of project files, used to store text content and comments.
/// The content field is used to save the content of the translated text, and the comment
/// field is used to save the comments of the translated text.
///
/// To create a text, you can use the default constructor `Text::new()`, and then
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

    create_at: Date,
    update_at: Date,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.update_at = Date::now();

        self.content = content.to_string();

        self
    }

    pub fn with_comment(mut self, comment: impl ToString) -> Self {
        self.update_at = Date::now();

        self.comment = comment.to_string();

        self
    }

    pub fn set_content(&mut self, content: impl ToString) {
        self.update_at = Date::now();

        self.content = content.to_string();
    }

    pub fn set_comment(&mut self, comment: impl ToString) {
        self.update_at = Date::now();

        self.comment = comment.to_string();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn create_at(&self) -> &Date {
        &self.create_at
    }

    pub fn update_at(&self) -> &Date {
        &self.update_at
    }
}

impl Codec for Text {
    fn decode<S>(reader: &mut Reader<S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        Ok(Self {
            content: reader.read_string_with_len::<u32>()?,
            comment: reader.read_string_with_len::<u32>()?,

            create_at: reader.read_object()?,
            update_at: reader.read_object()?,
        })
    }

    fn encode<S>(&self, writer: &mut Writer<S>) -> codec::Result<()>
    where
        S: Write + Seek,
    {
        writer.write_string_with_len::<u32>(&self.content)?;
        writer.write_string_with_len::<u32>(&self.comment)?;

        writer.write_object(&self.create_at)?;
        writer.write_object(&self.update_at)?;

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
    fn codec() -> anyhow::Result<()> {
        let text = Text::new().with_content("Content").with_comment("Comment");

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&text)?;
        writer.rewind()?;

        let mut reader = Reader::new(writer.into_inner());

        assert_eq!(reader.read_object::<Text>()?, text);

        Ok(())
    }
}
