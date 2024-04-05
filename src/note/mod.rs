use crate::error::FileResult;

use crate::text::Text;

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

use std::fmt::{
  Formatter,
  Debug,
};

type Texts = Vec<Text>;

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

  pub fn texts_mut(&mut self) -> &mut [Text] {
    &mut self.texts
  }

  pub fn texts(&self) -> &[Text] {
    &self.texts
  }

  pub fn remove_text(&mut self, index: u32) {
    self.texts.remove(index as usize);
  }

  pub fn add_text(&mut self, text: Text) {
    self.texts.push(text);
  }

  pub(crate) fn merge_texts(&self) -> String {
    self.texts.iter().map(|text| {
      let mut result = String::new();

      if text.content().is_empty() && !text.comment().is_empty() {
        result.push_str(text.comment());
      } else if text.comment().is_empty() && !text.content().is_empty() {
        result.push_str(text.content());
      } else {
        result.push_str(format!("{}\n\n{}", text.content(), text.comment()).as_ref());
      }

      result
    }).collect::<Vec<String>>().join("\n\n")
  }
}

impl Encode for Note {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    codec.write_primitive(self.x)?;
    codec.write_primitive(self.y)?;

    codec.write_primitive(self.choice)?;

    codec.write_primitive(self.texts.len() as u32)?;

    self.texts.encode(codec)?;

    Ok(())
  }
}

impl Encode for Texts {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    for text in self {
      text.encode(codec)?;
    }

    Ok(())
  }
}

impl Decode for Note {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    let x = codec.read_primitive::<f64>()?;
    let y = codec.read_primitive::<f64>()?;

    let choice = codec.read_primitive::<u32>()?;

    let texts = Texts::decode(codec)?;

    Ok(Self {
      x,
      y,

      choice,

      texts,
    })
  }
}

impl Decode for Texts {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    let len = codec.read_primitive::<u32>()?;

    let mut texts = Vec::with_capacity(len as usize);

    for _ in 0..len {
      texts.push(Text::decode(codec)?);
    }

    Ok(texts)
  }
}

impl Debug for Note {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "X: {:.2}", self.x)?;
    writeln!(f, "Y: {:.2}", self.y)?;

    writeln!(f)?;

    writeln!(f, "Choice: {}", self.choice)?;

    writeln!(f)?;

    writeln!(f, "Texts[{}]:", self.texts.len())?;
    write!(f, "{}", &self.texts
      .iter()
      .enumerate()
      .map(|(index, text)| format!("* {}\n{:?}", index + 1, text)
        .lines()
        .map(|line| format!("  {}", line))
        .collect::<Vec<String>>()
        .join("\n")
      ).collect::<Vec<String>>()
      .join("\n\n")
    )?;

    Ok(())
  }
}