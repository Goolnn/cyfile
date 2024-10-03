mod notes;

pub use notes::Notes;

use crate::error::FileResult;
use crate::Texts;
use crate::{Codec, Decode, Encode};

#[derive(Default)]
pub struct Note {
    x: f64,
    y: f64,

    choice: u32,

    texts: Texts,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_coordinate_and_choice(x: f64, y: f64, choice: u32) -> Self {
        Self {
            x,
            y,

            choice,

            ..Self::default()
        }
    }

    pub fn with_coordinate(x: f64, y: f64) -> Self {
        Self {
            x,
            y,

            ..Self::default()
        }
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_choice(&mut self, choice: u32) {
        self.choice = choice;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn choice(&self) -> u32 {
        self.choice
    }

    pub fn texts_mut(&mut self) -> &mut Texts {
        &mut self.texts
    }

    pub fn texts(&self) -> &Texts {
        &self.texts
    }

    pub(crate) fn merge_texts(&self) -> String {
        self.texts
            .inner()
            .iter()
            .map(|text| {
                let mut result = String::new();

                if text.content().is_empty() && !text.comment().is_empty() {
                    result.push_str(text.comment());
                } else if text.comment().is_empty() && !text.content().is_empty() {
                    result.push_str(text.content());
                } else {
                    result.push_str(format!("{}\n\n{}", text.content(), text.comment()).as_ref());
                }

                result
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Encode for Note {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        codec.write_primitive(self.x)?;
        codec.write_primitive(self.y)?;

        codec.write_primitive(self.choice)?;

        codec.write_primitive(self.texts.len() as u32)?;

        codec.write_object(self.texts())?;

        Ok(())
    }
}

impl Decode for Note {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        let x = codec.read_primitive()?;
        let y = codec.read_primitive()?;

        let choice = codec.read_primitive()?;

        let texts = codec.read_object()?;

        Ok(Self {
            x,
            y,

            choice,

            texts,
        })
    }
}
