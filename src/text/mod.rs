mod texts;

pub use texts::Texts;

use crate::error::{FileError, FileResult};
use crate::{Codec, Decode, Encode};

/// Be used to store the content and the comment of a text.
///
/// The `Text` type generally does not exist independently but rather as a child of a
/// [`Text`](Text) type.
///
/// # Examples
///
/// You can create a `Text` with content and comment use [`Text::with_content_and_comment`], and to
/// get the content and the comment of a `Text`, you can use [`content`](Text::content) and
/// [`content`](Text::content):
///
/// ```
/// use cyfile::Text;
///
/// let text = Text::with_content_and_comment("Content of the text", "Comment of the text");
///
/// assert_eq!(text.content(), "Content of the text");
/// assert_eq!(text.comment(), "Comment of the text");
/// ```
///
/// You can clear a `Text` with [`clear_content`](Text::clear_content) and [`clear_comment`](Text::clear_comment):
///
/// ```
/// use cyfile::Text;
///
/// let mut text = Text::with_content_and_comment("Content of the text", "Comment of the text");
///
/// text.clear_content();
/// text.clear_comment();
///
/// assert!(text.content().is_empty());
/// assert!(text.comment().is_empty());
/// ```
///
/// # Encode
///
/// Support version: `0.2`
///
/// # Decode
///
/// Support version: `0.2`
#[derive(Default)]
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

    /// Creates a new `Text` with content and comment.
    ///
    /// # Exampls
    ///
    /// ```
    /// use cyfile::Text;
    ///
    /// let text = Text::with_content_and_comment("Content of the text", "Comment of the text");
    ///
    /// assert_eq!(text.content(), "Content of the text");
    /// assert_eq!(text.comment(), "Comment of the text");
    /// ```
    pub fn with_content_and_comment(content: &str, comment: &str) -> Self {
        Self {
            content: content.to_string(),
            comment: comment.to_string(),
        }
    }

    /// Creates a new `Text` with content. The comment is still empty before setting.
    ///
    /// # Exampls
    ///
    /// ```
    /// use cyfile::Text;
    ///
    /// let text = Text::with_content("Content of the text");
    ///
    /// assert_eq!(text.content(), "Content of the text");
    /// assert!(text.comment().is_empty());
    /// ```
    pub fn with_content(content: &str) -> Self {
        Self {
            content: content.to_string(),

            ..Self::default()
        }
    }

    /// Creates a new `Text` with comment. The content is still empty before setting.
    ///
    /// # Exampls
    ///
    /// ```
    /// use cyfile::Text;
    ///
    /// let text = Text::with_comment("Comment of the text");
    ///
    /// assert!(text.content().is_empty());
    /// assert_eq!(text.comment(), "Comment of the text");
    /// ```
    pub fn with_comment(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),

            ..Self::default()
        }
    }

    /// Clear content of the `Text`.
    ///
    /// This method will clear all data of content.
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    /// Clear comment of the `Text`.
    ///
    /// This method will clear all data of comment.
    pub fn clear_comment(&mut self) {
        self.comment.clear();
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
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        match codec.version() {
            (0, 2) => {
                codec.write_string::<u32>(&self.content)?;
                codec.write_string::<u32>(&self.comment)?;

                Ok(())
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

impl Decode for Text {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        match codec.version() {
            (0, 2) => Ok(Self {
                content: codec.read_string::<u32>()?,
                comment: codec.read_string::<u32>()?,
            }),

            _ => Err(FileError::InvalidVersion),
        }
    }
}
