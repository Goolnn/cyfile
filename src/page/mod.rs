use crate::note::Note;

use image::GenericImageView;

use crate::error::{
  FileResult,
  FileError,
};

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

use std::fmt::{
  Formatter,
  Debug,
};

type Notes = Vec<Note>;

#[derive(Default)]
pub struct Page {
  raw: Vec<u8>,
  mask: Vec<u8>,

  notes: Notes,
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

  pub fn notes_mut(&mut self) -> &mut Notes {
    &mut self.notes
  }

  pub fn notes(&self) -> &Notes {
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

impl Encode for Page {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    match codec.version() {
      (0, 0) => {
        codec.write_data_with_len::<u32>(&self.raw)?;

        let (image_width, image_height) = {
          let (image_width, image_height) = self.size();

          (image_width as f64, image_height as f64)
        };

        let note_count = self.notes.len() as u8;

        codec.write_primitive(note_count)?;

        for note in &self.notes {
          let note_x = (image_width * (note.x() + 1.0) / 2.0) as u16;
          let note_y = (image_height * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

          codec.write_primitive(note_x)?;
          codec.write_primitive(note_y)?;

          let merged_text = note.merge_texts();

          codec.write_string_with_nil::<u16>(&merged_text)?;
        }

        Ok(())
      }

      (0, 2) => {
        codec.write_data_with_len::<u32>(&self.raw)?;
        codec.write_data_with_len::<u32>(&self.mask)?;

        codec.write_primitive(self.notes.len() as u32)?;

        for note in &self.notes {
          note.encode(codec)?;
        }

        Ok(())
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Decode for Page {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    match codec.version() {
      (0, 0) => {
        todo!()
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Debug for Page {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Raw Size: {:.4} MiB", self.raw.len() as f64 / 1024.0 / 1024.0)?;
    writeln!(f, "Mask Size: {:.4} MiB", self.mask.len() as f64 / 1024.0 / 1024.0)?;

    writeln!(f)?;

    writeln!(f, "Notes[{}]:", self.notes.len())?;
    writeln!(f, "{}", &self.notes.iter().enumerate().map(|(index, note)| format!("* {}\n{:?}", index + 1, note).lines().map(|line| format!("  {}", line)).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>().join("\n\n"))?;

    Ok(())
  }
}