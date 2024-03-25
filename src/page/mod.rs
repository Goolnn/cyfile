use crate::note::Note;

use image::GenericImageView;

#[derive(Default)]
pub struct Page {
  raw: Vec<u8>,
  mask: Vec<u8>,

  notes: Vec<Note>,
}

impl Page {
  pub fn new(raw: Vec<u8>) -> Self {
    Self {
      raw,

      ..Self::default()
    }
  }

  pub fn with_mask(raw: Vec<u8>, mask: Vec<u8>) -> Self {
    Self {
      raw,
      mask,

      ..Self::default()
    }
  }

  pub fn set_raw(&mut self, raw: Vec<u8>) {
    self.raw = raw;
  }

  pub fn set_mask(&mut self, mask: Vec<u8>) {
    self.mask = mask;
  }

  pub fn raw_mut(&mut self) -> &mut Vec<u8> {
    &mut self.raw
  }

  pub fn raw(&self) -> &Vec<u8> {
    &self.raw
  }

  pub fn mask_mut(&mut self) -> &mut Vec<u8> {
    &mut self.mask
  }

  pub fn mask(&self) -> &Vec<u8> {
    &self.mask
  }

  pub fn notes_mut(&mut self) -> &mut Vec<Note> {
    &mut self.notes
  }

  pub fn notes(&self) -> &Vec<Note> {
    &self.notes
  }

  pub fn size(&self) -> (usize, usize) {
    if let Ok(image) = image::load_from_memory(&self.raw) {
      let (width, height) = image.dimensions();

      (width as usize, height as usize)
    } else {
      (0, 0)
    }
  }
}