use crate::text::Text;

pub struct Note {
  x: f64,
  y: f64,

  choice: usize,

  texts: Vec<Text>,
}

impl Note {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_coordinate_and_choice(x: f64, y: f64, choice: usize) -> Self {
    Self {
      x,
      y,

      choice,

      texts: Vec::new(),
    }
  }

  pub fn with_coordinate(x: f64, y: f64) -> Self {
    Self {
      x,
      y,

      choice: 0,

      texts: Vec::new(),
    }
  }

  pub fn set_x(&mut self, x: f64) {
    self.x = x;
  }

  pub fn set_y(&mut self, y: f64) {
    self.y = y;
  }

  pub fn set_choice(&mut self, choice: usize) {
    self.choice = choice;
  }

  pub fn x(&self) -> f64 {
    self.x
  }

  pub fn y(&self) -> f64 {
    self.y
  }

  pub fn choice(&self) -> usize {
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