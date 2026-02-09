use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;

#[derive(Debug, Default)]
pub struct Text {
    content: String,
    comment: String,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content<C>(&mut self, content: C)
    where
        C: ToString,
    {
        self.content = content.to_string();
    }

    pub fn with_content<C>(mut self, content: C) -> Self
    where
        C: ToString,
    {
        self.content = content.to_string();

        self
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn set_comment<M>(&mut self, comment: M)
    where
        M: ToString,
    {
        self.comment = comment.to_string();
    }

    pub fn with_comment<M>(mut self, comment: M) -> Self
    where
        M: ToString,
    {
        self.comment = comment.to_string();

        self
    }
}

impl Codec for Text {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        match writer.manifest().version() {
            0 => {
                writer.field("content", &self.content)?;

                writer.field("comment", &self.comment)?;

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        match reader.manifest().version() {
            0 => Ok(Text {
                content: reader.field("content")?,

                comment: reader.field("comment")?,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Codec;
    use crate::Text;
    use crate::codec::Writer;
    use crate::file::Manifest;
    use serde_json::json;

    #[test]
    fn new() {
        let text = Text::new();

        assert!(text.content().is_empty());
        assert!(text.comment().is_empty());
    }

    #[test]
    fn with_content() {
        let text = Text::new().with_content("This is a content.");

        assert_eq!(text.content(), "This is a content.");
    }

    #[test]
    fn with_comment() {
        let text = Text::new().with_comment("This is a comment.");

        assert_eq!(text.comment(), "This is a comment.");
    }

    #[test]
    fn with_content_and_comment() {
        let text = Text::new()
            .with_content("This is a content.")
            .with_comment("This is a comment.");

        assert_eq!(text.content(), "This is a content.");
        assert_eq!(text.comment(), "This is a comment.");
    }

    #[test]
    fn set_content() {
        let mut text = Text::new();

        text.set_content("This is a content.");

        assert_eq!(text.content(), "This is a content.");
    }

    #[test]
    fn set_comment() {
        let mut text = Text::new();

        text.set_comment("This is a comment.");

        assert_eq!(text.comment(), "This is a comment.");
    }

    #[test]
    fn encode() {
        let text = Text::new()
            .with_content("This is a content.")
            .with_comment("This is a comment.");

        let manifest = Manifest::default();

        let mut writer = Writer::new(&manifest);

        assert!(Codec::encode(&text, &mut writer).is_ok());

        let value = writer.into_value();

        assert_eq!(
            value,
            json!({
                "content": "This is a content.",
                "comment": "This is a comment."
            })
        );
    }
}
