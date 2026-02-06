use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::project::Text;

#[derive(Debug)]
pub struct Note {
    x: f32,
    y: f32,

    texts: Vec<Text>,
}

impl Codec for Note {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.field("x", &self.x)?;
        writer.field("y", &self.y)?;

        writer.field("texts", &self.texts)?;

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Note {
            x: reader.field("x")?,
            y: reader.field("y")?,

            texts: reader.field("texts")?,
        })
    }
}
