use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;

#[derive(Debug)]
pub struct Text {
    content: String,
    comment: String,
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
