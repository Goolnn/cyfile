use crate::Note;
use crate::Text;

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
  source: Vec<u8>,
  mask: Vec<u8>,

  notes: Notes,
}

impl Page {
  pub fn new(source: Vec<u8>, mask: Vec<u8>) -> Self {
    Self {
      source,
      mask,

      ..Self::default()
    }
  }

  pub fn with_source(source: Vec<u8>) -> Self {
    Self {
      source,

      ..Self::default()
    }
  }

  pub fn set_source(&mut self, source: Vec<u8>) {
    self.source = source;
  }

  pub fn set_mask(&mut self, mask: Vec<u8>) {
    self.mask = mask;
  }

  pub fn source_mut(&mut self) -> &mut [u8] {
    &mut self.source
  }

  pub fn source(&self) -> &[u8] {
    &self.source
  }

  pub fn mask_mut(&mut self) -> &mut [u8] {
    &mut self.mask
  }

  pub fn mask(&self) -> &[u8] {
    &self.mask
  }

  pub fn notes_mut(&mut self) -> &mut [Note] {
    &mut self.notes
  }

  pub fn notes(&self) -> &[Note] {
    &self.notes
  }

  pub fn remove_note(&mut self, index: u32) {
    self.notes.remove(index as usize);
  }

  pub fn add_note(&mut self, note: Note) {
    self.notes.push(note);
  }

  pub(crate) fn size(&self) -> (usize, usize) {
    if let Ok(image) = image::load_from_memory(&self.source) {
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
        // 图像数据
        codec.write_data_with_len::<u32>(self.source())?;

        // 图像尺寸
        let (page_width, page_height) = self.size();

        // 标签数量
        codec.write_primitive(self.notes().len() as u8)?;

        for note in self.notes() {
          let note_x = (page_width as f64 * (note.x() + 1.0) / 2.0) as u16;
          let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

          codec.write_primitive(note_x)?;
          codec.write_primitive(note_y)?;

          // 合并文本
          let merged_text = note.merge_texts();

          codec.write_string_with_nil::<u16>(&merged_text)?;
        }

        Ok(())
      }

      (0, 2) => {
        codec.write_data_with_len::<u32>(&self.source)?;
        codec.write_data_with_len::<u32>(&self.mask)?;

        self.notes.encode(codec)?;

        Ok(())
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Encode for Notes {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    codec.write_primitive::<u32>(self.len() as u32)?;

    for note in self {
      note.encode(codec)?;
    }

    Ok(())
  }
}

impl Decode for Page {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    match codec.version() {
      (0, 0) => {
        let mut page = Page::with_source(codec.read_data_with_len::<u32>()?);

        let (page_width, page_height) = page.size();

        let note_count = codec.read_primitive::<u8>()?;

        for _ in 0..note_count {
          let note_x = codec.read_primitive::<u16>()? as f64;
          let note_y = codec.read_primitive::<u16>()? as f64;

          let content = codec.read_string_with_nil::<u16>()?;

          let mut note = Note::with_coordinate(
            note_x / page_width as f64 * 2.0 - 1.0,
            1.0 - note_y / page_height as f64 * 2.0,
          );

          note.add_text(Text::with_content(&content));

          page.add_note(note);
        }

        Ok(page)
      }

      (0, 2) => {
        let source = codec.read_data_with_len::<u32>()?;
        let mask = codec.read_data_with_len::<u32>()?;

        let notes = Notes::decode(codec)?;

        Ok(Self {
          source,
          mask,

          notes,
        })
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Decode for Notes {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    let note_count = codec.read_primitive::<u32>()?;

    let mut notes = Vec::with_capacity(note_count as usize);

    for _ in 0..note_count {
      notes.push(Note::decode(codec)?);
    }

    Ok(notes)
  }
}

impl Debug for Page {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Source Size: {:.4} MiB", self.source.len() as f64 / 1024.0 / 1024.0)?;
    writeln!(f, "Mask Size: {:.4} MiB", self.mask.len() as f64 / 1024.0 / 1024.0)?;

    writeln!(f)?;

    writeln!(f, "Notes[{}]:", self.notes.len())?;
    write!(f, "{}", &self.notes
      .iter()
      .enumerate()
      .map(|(index, note)| format!("* {}\n{:?}", index + 1, note)
        .lines()
        .map(|line| format!("  {}", line))
        .collect::<Vec<String>>()
        .join("\n"))
      .collect::<Vec<String>>()
      .join("\n\n")
    )?;

    Ok(())
  }
}