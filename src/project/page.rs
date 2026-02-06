use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::project::Asset;
use crate::project::Note;

#[derive(Debug)]
pub struct Page {
    image: Asset,

    notes: Vec<Note>,
}

impl Codec for Page {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.field("image", &self.image)?;

        writer.field("notes", &self.notes)?;

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Page {
            image: reader.field("image")?,

            notes: reader.field("notes")?,
        })
    }
}
