use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::project::Text;

#[derive(Debug, Default)]
pub struct Note {
    x: f32,
    y: f32,

    texts: Vec<Text>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;

        self
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;

        self
    }

    pub fn texts(&self) -> &Vec<Text> {
        &self.texts
    }

    pub fn texts_mut(&mut self) -> &mut Vec<Text> {
        &mut self.texts
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.texts.push(text);

        self
    }
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
