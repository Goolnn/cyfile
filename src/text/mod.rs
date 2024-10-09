use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::error::FileResult;
use std::io::Read;
use std::io::Write;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Text {
    content: String,
    comment: String,
}

impl Text {
    /// Creates a new empty `Text`.
    ///
    /// Given that the `Text` is empty, the content and the comment will not allocate any initial
    /// buffer. If you have an idea of what the content and the comment is, consider the
    /// [`Text::with_content_and_comment`], [`Text::with_content`] and [`Text::with_comment`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cyfile::Text;
    ///
    /// let text = Text::new();
    /// ```
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

    /// Set content of the `Text`.
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    /// Set comment of the `Text`.
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    /// Return content of the `Text`.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Return comment of the `Text`.
    pub fn comment(&self) -> &str {
        &self.comment
    }
}

impl Encode for Text {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> FileResult<()> {
        match writer.version() {
            (0, 2) => {
                writer.write_string_with_len::<u32>(&self.content)?;
                writer.write_string_with_len::<u32>(&self.comment)?;

                Ok(())
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

impl Decode for Text {
    fn decode<S: Read>(reader: &mut Reader<S>) -> FileResult<Self> {
        match reader.version() {
            (0, 2) => Ok(Self {
                content: reader.read_string_with_len::<u32>()?,
                comment: reader.read_string_with_len::<u32>()?,
            }),

            _ => Err(FileError::InvalidVersion),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Text;

    #[test]
    fn test_new() {
        let text = Text::new();

        assert!(text.content().is_empty());
        assert!(text.comment().is_empty());
    }

    #[test]
    fn test_with_content_and_comment() {
        let text = Text::with_content_and_comment("Content of the text", "Comment of the text");

        assert_eq!(text.content(), "Content of the text");
        assert_eq!(text.comment(), "Comment of the text");
    }

    #[test]
    fn test_with_content() {
        let text = Text::with_content("Content of the text");

        assert_eq!(text.content(), "Content of the text");
        assert!(text.comment().is_empty());
    }

    #[test]
    fn test_with_comment() {
        let text = Text::with_comment("Comment of the text");

        assert!(text.content().is_empty());
        assert_eq!(text.comment(), "Comment of the text");
    }

    #[test]
    fn test_set_content() {
        let mut text = Text::new();

        text.set_content("content");

        assert_eq!(text.content(), "content");
        assert!(text.comment().is_empty());
    }

    #[test]
    fn test_set_comment() {
        let mut text = Text::new();

        text.set_comment("comment");

        assert!(text.content().is_empty());
        assert_eq!(text.comment(), "comment");
    }

    #[test]
    fn test_set_content_and_comment() {
        let mut text = Text::new();

        text.set_content("content");
        text.set_comment("comment");

        assert_eq!(text.content(), "content");
        assert_eq!(text.comment(), "comment");
    }
}
