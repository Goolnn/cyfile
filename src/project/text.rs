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
        writer.field("content", &self.content)?;

        writer.field("comment", &self.comment)?;

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Text {
            content: reader.field("content")?,
            comment: reader.field("comment")?,
        })
    }
}
