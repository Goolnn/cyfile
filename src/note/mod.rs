use crate::text::Text;

use crate::error::{
  FileResult,
  FileError,
};

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

pub struct Note {
  x: f64,
  y: f64,

  choice: u32,

  texts: Vec<Text>,
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

  pub fn texts_mut(&mut self) -> &mut Vec<Text> {
    &mut self.texts
  }

  pub fn texts(&self) -> &Vec<Text> {
    &self.texts
  }
}

impl Default for Note {
  fn default() -> Self {
    Self {
      x: 0.0,
      y: 0.0,

      choice: 0,

      texts: Vec::new(),
    }
  }
}

impl Encode for Note {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    match codec.version() {
      (0, 2) => {
        codec.write_primitive(self.x)?;
        codec.write_primitive(self.y)?;

        codec.write_primitive(self.choice)?;

        codec.write_primitive(self.texts.len() as u32)?;

        for text in &self.texts {
          text.encode(codec)?;
        }

        Ok(())
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Decode for Note {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    match codec.version() {
      (0, 2) => {
        let x = codec.read_primitive::<f64>()?;
        let y = codec.read_primitive::<f64>()?;

        let choice = codec.read_primitive::<u32>()?;

        let len = codec.read_primitive::<u32>()?;

        let mut texts = Vec::with_capacity(len as usize);

        for _ in 0..len {
          texts.push(Text::decode(codec)?);
        }

        Ok(Self {
          x,
          y,

          choice,

          texts,
        })
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}