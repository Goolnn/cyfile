use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::error::FileResult;
use crate::Note;
use crate::Text;
use image::ImageReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

pub struct Page {
    data: Vec<u8>,

    notes: Vec<Note>,
}

impl Page {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,

            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes.push(note);

        self
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> (usize, usize) {
        let cursor = Cursor::new(self.data());
        let reader = ImageReader::new(cursor).with_guessed_format().unwrap();
        let dimensions = reader.into_dimensions().unwrap();

        (dimensions.0 as usize, dimensions.1 as usize)
    }

    pub fn notes_mut(&mut self) -> &mut Vec<Note> {
        &mut self.notes
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

impl Encode for Page {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> FileResult<()> {
        match writer.version() {
            (0, 0) => {
                // 图像数据
                writer.write_bytes_with_len::<u32>(self.data())?;

                // 图像尺寸
                let (page_width, page_height) = self.size();

                // 标签数量
                writer.write_primitive(self.notes().len() as u8)?;

                for note in self.notes() {
                    let note_x = (page_width as f64 * (note.x() + 1.0) / 2.0) as u16;
                    let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

                    writer.write_primitive(note_x)?;
                    writer.write_primitive(note_y)?;

                    // 合并文本
                    let merged_text = note.merge_texts();

                    writer.write_primitive(merged_text.len() as u16 + 1)?;
                    writer.write_string_with_nil(&merged_text)?;
                }

                Ok(())
            }

            (0, 2) => {
                writer.write_bytes_with_len::<u32>(self.data())?;

                writer.write_primitive(self.notes().len() as u32)?;

                for note in self.notes() {
                    writer.write_object(note)?;
                }

                Ok(())
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

impl Decode for Page {
    fn decode<S: Read>(reader: &mut Reader<S>) -> FileResult<Self> {
        match reader.version() {
            (0, 0) => {
                let data = reader.read_bytes_with_len::<u32>()?;

                let mut page = Page::new(data);

                let (page_width, page_height) = page.size();

                let note_count = reader.read_primitive::<u8>()?;

                for _ in 0..note_count {
                    let note_x = reader.read_primitive::<u16>()? as f64;
                    let note_y = reader.read_primitive::<u16>()? as f64;

                    reader.read_primitive::<u16>()?;

                    let content = reader.read_string_with_nil()?;

                    let mut note = Note::new().with_coordinate(
                        note_x / page_width as f64 * 2.0 - 1.0,
                        1.0 - note_y / page_height as f64 * 2.0,
                    );

                    note.texts_mut().push(Text::new().with_content(&content));

                    page.notes_mut().push(note);
                }

                Ok(page)
            }

            (0, 2) => {
                let data = reader.read_bytes_with_len::<u32>()?;

                let mut page = Page::new(data);

                let note_count = reader.read_primitive::<u8>()?;

                for _ in 0..note_count {
                    let note = reader.read_object()?;

                    page.notes_mut().push(note);
                }

                Ok(page)
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}
