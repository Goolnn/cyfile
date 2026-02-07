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

impl Page {
    pub fn new(image: Asset) -> Self {
        Page {
            image,

            notes: Vec::new(),
        }
    }

    pub fn image(&self) -> &Asset {
        &self.image
    }

    pub fn set_image(&mut self, image: Asset) {
        self.image = image;
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn notes_mut(&mut self) -> &mut Vec<Note> {
        &mut self.notes
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes.push(note);

        self
    }
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
