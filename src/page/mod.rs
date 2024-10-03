mod image;
mod pages;

pub use image::Image;
pub use pages::Pages;

use crate::error::{FileError, FileResult};
use crate::{Codec, Decode, Encode, Note, Notes, Text};
use ::image::GenericImageView;

#[derive(Default)]
pub struct Page {
    source: Image,
    mask: Image,

    notes: Notes,
}

impl Page {
    pub fn new(source: Vec<u8>, mask: Vec<u8>) -> Self {
        Self {
            source: Image::from(source),
            mask: Image::from(mask),

            ..Self::default()
        }
    }

    pub fn with_source(source: Vec<u8>) -> Self {
        Self {
            source: Image::from(source),

            ..Self::default()
        }
    }

    pub fn set_source(&mut self, source: Vec<u8>) {
        self.source = Image::from(source);
    }

    pub fn set_mask(&mut self, mask: Vec<u8>) {
        self.mask = Image::from(mask);
    }

    pub fn source_mut(&mut self) -> &mut Image {
        &mut self.source
    }

    pub fn source(&self) -> &Image {
        &self.source
    }

    pub fn mask_mut(&mut self) -> &mut Image {
        &mut self.mask
    }

    pub fn mask(&self) -> &Image {
        &self.mask
    }

    pub fn notes_mut(&mut self) -> &mut Notes {
        &mut self.notes
    }

    pub fn notes(&self) -> &Notes {
        &self.notes
    }

    pub(crate) fn size(&self) -> (usize, usize) {
        if let Ok(image) = ::image::load_from_memory(self.source.inner()) {
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
                codec.write_object(self.source())?;

                // 图像尺寸
                let (page_width, page_height) = self.size();

                // 标签数量
                codec.write_primitive(self.notes().len() as u8)?;

                for note in self.notes().inner() {
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
                codec.write_object(self.source())?;
                codec.write_object(self.mask())?;

                codec.write_object(self.notes())?;

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

                    note.texts_mut().push_back(Text::with_content(&content));

                    page.notes_mut().push_back(note);
                }

                Ok(page)
            }

            (0, 2) => {
                let source = codec.read_data_with_len::<u32>()?;
                let mask = codec.read_data_with_len::<u32>()?;

                let notes = codec.read_object()?;

                Ok(Self {
                    source: Image::from(source),
                    mask: Image::from(mask),

                    notes,
                })
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}
