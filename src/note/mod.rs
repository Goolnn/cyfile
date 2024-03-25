use crate::text::Text;
use crate::page::Page;

pub struct Note<'a> {
  parent: Option<&'a Page>,
  
  x: f64,
  y: f64,

  choice: usize,

  texts: Vec<Text>,
}

impl<'a> Note<'a> {
  pub fn new(parent: &'a Page) -> Self {
    Self {
      parent: Some(parent),
      
      ..Self::default()
    }
  }

  pub fn with_coordinate_and_choice(parent: &'a Page, x: f64, y: f64, choice: usize) -> Self {
    Self {
      parent: Some(parent),
      
      x,
      y,

      choice,

      ..Self::default()
    }
  }

  pub fn with_coordinate(parent: &'a Page, x: f64, y: f64) -> Self {
    Self {
      parent: Some(parent),
      
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

impl<'a> Default for Note<'a> {
  fn default() -> Self {
    Self {
      parent: None,
      
      x: 0.0,
      y: 0.0,

      choice: 0,

      texts: Vec::new(),
    }
  }
}