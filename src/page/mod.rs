use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::file;
use crate::Note;
use crate::Text;
use image::ImageReader;
use std::fmt::Debug;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
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

    pub fn with_notes(mut self, notes: Vec<Note>) -> Self {
        self.notes.extend(notes);

        self
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes.push(note);

        self
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn set_notes(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn notes_mut(&mut self) -> &mut Vec<Note> {
        &mut self.notes
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn size(&self) -> file::Result<(usize, usize)> {
        let cursor = Cursor::new(self.data());
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let dimensions = reader.into_dimensions()?;

        Ok((dimensions.0 as usize, dimensions.1 as usize))
    }
}

impl Codec for Page {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> codec::Result<Self> {
        match reader.version().into() {
            (0, 0) => {
                let data = reader.read_bytes_with_len::<u32>()?;

                let mut page = Page::new(data);

                let (page_width, page_height) =
                    page.size().map_err(|_| codec::Error::InvalidImage)?;

                let note_count = reader.read_primitive::<u8>()?;

                for _ in 0..note_count {
                    let note_x = reader.read_primitive::<u16>()? as f64;
                    let note_y = reader.read_primitive::<u16>()? as f64;

                    reader.read_primitive::<u16>()?;

                    let content = reader.read_string_with_nil()?;

                    let mut note = Note::new().with_position(
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
                let notes = reader.read_objects::<u32, Note>()?;

                let page = Page::new(data).with_notes(notes);

                Ok(page)
            }

            version => Err(codec::Error::UnsupportedVersion {
                version: version.into(),
            }),
        }
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> codec::Result<()> {
        match writer.version().into() {
            (0, 0) => {
                // 图像数据
                writer.write_bytes_with_len::<u32>(self.data())?;

                // 图像尺寸
                let (page_width, page_height) =
                    self.size().map_err(|_| codec::Error::InvalidImage)?;

                // 标签数量
                writer.write_primitive(self.notes().len() as u8)?;

                for note in self.notes() {
                    let note_x = (page_width as f64 * (note.x() + 1.0) / 2.0) as u16;
                    let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

                    writer.write_primitive(note_x)?;
                    writer.write_primitive(note_y)?;

                    // 合并文本
                    let (content, comment) = note.merge_texts();

                    let merged_text = [content, comment]
                        .into_iter()
                        .filter(|text| !text.is_empty())
                        .collect::<Vec<String>>()
                        .join("\n\n");

                    writer.write_primitive(merged_text.len() as u16 + 1)?;
                    writer.write_string_with_nil(&merged_text)?;
                }

                Ok(())
            }

            (0, 2) => {
                writer.write_bytes_with_len::<u32>(self.data())?;
                writer.write_objects::<u32, Note>(self.notes())?;

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion {
                version: version.into(),
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
    fn new() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let page = Page::new(image.clone());

        assert_eq!(page.data(), image.as_slice());
        assert_eq!(page.notes().len(), 0);

        Ok(())
    }

    #[test]
    fn with_notes() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let page = Page::new(image.clone()).with_notes(vec![
            Note::new().with_position(0.5, 0.5),
            Note::new().with_position(-0.5, -0.5),
        ]);

        assert_eq!(page.data(), image.as_slice());

        assert_eq!(page.notes().len(), 2);

        assert_eq!(page.notes()[0].x(), 0.5);
        assert_eq!(page.notes()[0].y(), 0.5);

        assert_eq!(page.notes()[1].x(), -0.5);
        assert_eq!(page.notes()[1].y(), -0.5);

        Ok(())
    }

    #[test]
    fn with_note() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let page = Page::new(image.clone())
            .with_note(Note::new().with_position(0.5, 0.5))
            .with_note(Note::new().with_position(-0.5, -0.5));

        assert_eq!(page.data(), image.as_slice());

        assert_eq!(page.notes().len(), 2);

        assert_eq!(page.notes()[0].x(), 0.5);
        assert_eq!(page.notes()[0].y(), 0.5);

        assert_eq!(page.notes()[1].x(), -0.5);
        assert_eq!(page.notes()[1].y(), -0.5);

        Ok(())
    }

    #[test]
    fn set_data() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let mut page = Page::new(image.clone());

        let new_image = fs::read(r"tests/images/1.png")?;
        page.set_data(new_image.clone());

        assert_eq!(page.data(), new_image.as_slice());

        Ok(())
    }

    #[test]
    fn set_notes() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let mut page = Page::new(image.clone());

        let notes = vec![
            Note::new().with_position(0.5, 0.5),
            Note::new().with_position(-0.5, -0.5),
        ];

        page.set_notes(notes.clone());

        assert_eq!(page.notes(), &notes);

        Ok(())
    }

    #[test]
    fn codec_for_version_0_0() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let page = Page::new(image)
            .with_note(
                Note::new()
                    .with_position(0.5, 0.5)
                    .with_text(Text::new().with_content("content_1_1"))
                    .with_text(Text::new().with_content("content_1_2"))
                    .with_text(Text::new().with_content("content_1_3")),
            )
            .with_note(
                Note::new()
                    .with_position(-0.5, -0.5)
                    .with_text(Text::new().with_content("content_2_1"))
                    .with_text(Text::new().with_content("content_2_2"))
                    .with_text(Text::new().with_content("content_2_3")),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 0));

        writer.write_object(&page)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor).with_version((0, 0));

        let read_page = reader.read_object::<Page>()?;

        assert_eq!(read_page.data(), page.data());

        for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
            assert!(read_note.x() - note.x() <= 0.1);
            assert!(read_note.y() - note.y() <= 0.1);

            let (content, comment) = note.merge_texts();

            let merged_text = [content, comment]
                .into_iter()
                .filter(|text| !text.is_empty())
                .collect::<Vec<String>>()
                .join("\n\n");

            assert_eq!(read_note.texts()[0].content(), merged_text);
        }

        Ok(())
    }

    #[test]
    fn codec_for_version_0_2() -> anyhow::Result<()> {
        let image = fs::read(r"tests/images/0.png")?;
        let page = Page::new(image)
            .with_note(
                Note::new()
                    .with_position(0.5, 0.5)
                    .with_text(Text::new().with_content("content_1_1"))
                    .with_text(Text::new().with_content("content_1_2"))
                    .with_text(Text::new().with_content("content_1_3")),
            )
            .with_note(
                Note::new()
                    .with_position(-0.5, -0.5)
                    .with_text(Text::new().with_content("content_2_1"))
                    .with_text(Text::new().with_content("content_2_2"))
                    .with_text(Text::new().with_content("content_2_3")),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 2));

        writer.write_object(&page)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor).with_version((0, 2));

        assert_eq!(reader.read_object::<Page>()?, page);

        Ok(())
    }
}
