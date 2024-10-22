use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::Note;
use crate::Text;
use image::ImageReader;
use std::fmt::Debug;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

#[derive(Clone, PartialEq)]
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
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        match writer.version().into() {
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

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        }
    }
}

impl Decode for Page {
    fn decode<S: Read>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        match reader.version().into() {
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

                let note_count = reader.read_primitive::<u32>()?;

                for _ in 0..note_count {
                    let note = reader.read_object()?;

                    page.notes_mut().push(note);
                }

                Ok(page)
            }

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        }
    }
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page").field("notes", &self.notes).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Note;
    use crate::Page;
    use crate::Text;
    use std::fs;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn codec_for_version_0_0() {
        let image = fs::read(r"tests\images\0.png").unwrap();
        let page = Page::new(image)
            .with_note(
                Note::new()
                    .with_coordinate(0.5, 0.5)
                    .with_text(Text::new().with_content("content_1_1"))
                    .with_text(Text::new().with_content("content_1_2"))
                    .with_text(Text::new().with_content("content_1_3")),
            )
            .with_note(
                Note::new()
                    .with_coordinate(-0.5, -0.5)
                    .with_text(Text::new().with_content("content_2_1"))
                    .with_text(Text::new().with_content("content_2_2"))
                    .with_text(Text::new().with_content("content_2_3")),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 0));

        writer.write_object(&page).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor).with_version((0, 0));

        let read_page = reader.read_object::<Page>().unwrap();

        assert_eq!(read_page.data(), page.data());

        for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
            assert!(read_note.x() - note.x() <= 0.1);
            assert!(read_note.y() - note.y() <= 0.1);

            assert_eq!(read_note.texts()[0].content(), note.merge_texts());
        }
    }

    #[test]
    fn codec_for_version_0_2() {
        let image = fs::read(r"tests\images\0.png").unwrap();
        let page = Page::new(image)
            .with_note(
                Note::new()
                    .with_coordinate(0.5, 0.5)
                    .with_text(Text::new().with_content("content_1_1"))
                    .with_text(Text::new().with_content("content_1_2"))
                    .with_text(Text::new().with_content("content_1_3")),
            )
            .with_note(
                Note::new()
                    .with_coordinate(-0.5, -0.5)
                    .with_text(Text::new().with_content("content_2_1"))
                    .with_text(Text::new().with_content("content_2_2"))
                    .with_text(Text::new().with_content("content_2_3")),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 2));

        writer.write_object(&page).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor).with_version((0, 2));

        assert_eq!(reader.read_object::<Page>().unwrap(), page);
    }
}
