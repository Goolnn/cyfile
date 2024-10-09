use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::error::FileResult;
use crate::Date;
use crate::Note;
use crate::Page;
use crate::Text;
use regex::Regex;
use std::io::Read;
use std::io::Write;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Project {
    title: String,

    created_date: Date,
    saved_date: Date,

    pages: Vec<Page>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            title: String::new(),

            created_date: Date::now(),
            saved_date: Date::now(),

            pages: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);

        self
    }

    pub fn set_title(&mut self, title: impl ToString) {
        self.title = title.to_string();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn created_date(&self) -> Date {
        self.created_date
    }

    pub fn saved_date(&self) -> Date {
        self.saved_date
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

impl Encode for Project {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> FileResult<()> {
        match writer.version() {
            (0, 0) => {
                writer.write_primitive(self.pages().len() as u8)?;

                for page in self.pages() {
                    writer.write_object(page)?;
                }

                Ok(())
            }

            (0, 1) => {
                // 图像数量
                writer.write_primitive(self.pages.len() as u8)?;

                // 保存次数
                writer.write_primitive(1_u8)?;

                // 保存时间
                Date::now().encode(writer)?;

                // 图像数据
                for page in self.pages() {
                    writer.write_bytes_with_len::<u32>(page.data())?;
                }

                // 标记数据
                for page in self.pages() {
                    // 图像尺寸
                    let (page_width, page_height) = page.size();

                    // 标记数量
                    writer.write_primitive(page.notes().len() as u8)?;

                    for note in page.notes() {
                        let note_x = (page_width as f64 * ((note.x() + 1.0) / 2.0)) as u16;
                        let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

                        writer.write_primitive(note_x)?;
                        writer.write_primitive(note_y)?;

                        // 合并文本
                        let merged_text = note.merge_texts();

                        // 初译数据
                        writer.write_primitive(merged_text.len() as u16 + 1)?;
                        writer.write_string_with_nil(&merged_text)?;
                        // 校对数据
                        writer.write_primitive(1u16)?;
                        writer.write_string_with_nil("")?;
                    }
                }

                Ok(())
            }

            (0, 2) => {
                writer.write_string_with_len::<u32>(self.title())?;

                writer.write_object(&self.created_date())?;
                writer.write_object(&self.saved_date())?;

                writer.write_primitive(self.pages().len() as u32)?;

                for page in self.pages() {
                    writer.write_object(page)?;
                }

                Ok(())
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

impl Decode for Project {
    fn decode<S: Read>(reader: &mut Reader<S>) -> FileResult<Self> {
        match reader.version() {
            (0, 0) => {
                let page_count = reader.read_primitive::<u8>()?;

                let mut pages = Vec::with_capacity(page_count as usize);

                for _ in 0..page_count {
                    pages.push(reader.read_object()?);
                }

                Ok(Self {
                    title: String::new(),

                    pages,

                    ..Self::default()
                })
            }

            (0, 1) => {
                // 图像数量
                let page_count = reader.read_primitive::<u8>()?;

                // 保存次数
                reader.read_primitive::<u8>()?;

                // 保存时间
                let date = reader.read_object()?;

                // 读取图像
                let mut pages = Vec::with_capacity(page_count as usize);

                for _ in 0..page_count {
                    let image_data = reader.read_bytes_with_len::<u32>()?;

                    pages.push(Page::new(image_data));
                }

                // 读取标记
                for i in 0..page_count {
                    // 标记数量
                    let note_count = reader.read_primitive::<u8>()?;

                    let page = &mut pages[i as usize];

                    let (page_width, page_height) = page.size();

                    for _ in 0..note_count {
                        let note_x = reader.read_primitive::<u16>()? as f64;
                        let note_y = reader.read_primitive::<u16>()? as f64;

                        let mut note = Note::new().with_coordinate(
                            note_x / page_width as f64 * 2.0 - 1.0,
                            1.0 - note_y / page_height as f64 * 2.0,
                        );

                        reader.read_primitive::<u16>()?;
                        let draft = reader.read_string_with_nil()?;

                        reader.read_primitive::<u16>()?;
                        let revision = reader.read_string_with_nil()?;

                        if draft.contains("DOCTYPE HTML PUBLIC")
                            || revision.contains("DOCTYPE HTML PUBLIC")
                        {
                            // 解析 HTML 文本
                            let regex = match Regex::new(r"<span.*?>|</span>") {
                                Ok(regex) => regex,
                                Err(_) => return Err(FileError::Undefined),
                            };

                            let draft = regex.replace_all(&draft, "").to_string();
                            let revision = regex.replace_all(&revision, "").to_string();

                            let regex = match Regex::new(r"<p.*?>(.*)</p>") {
                                Ok(regex) => regex,
                                Err(_) => return Err(FileError::Undefined),
                            };

                            let extract = |text| {
                                regex
                                    .captures_iter(text)
                                    .map(|capture| {
                                        let (_, [text]) = capture.extract();

                                        if text == "<br />" {
                                            String::new()
                                        } else {
                                            text.replace("<br />", "\n")
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            };

                            let draft = extract(&draft);
                            let revision = extract(&revision);

                            // 添加文本
                            if !draft.is_empty() {
                                note.texts_mut().push(Text::new().with_content(&draft));
                            }

                            if !revision.is_empty() {
                                note.texts_mut().push(Text::new().with_content(&revision));
                            }
                        } else {
                            // 添加文本
                            if !draft.is_empty() {
                                note.texts_mut().push(Text::new().with_content(&draft));
                            }

                            if !revision.is_empty() {
                                note.texts_mut().push(Text::new().with_content(&revision));
                            }
                        }

                        page.notes_mut().push(note);
                    }
                }

                Ok(Self {
                    title: String::new(),

                    created_date: date,
                    saved_date: date,

                    pages,
                })
            }

            (0, 2) => {
                let title = reader.read_string_with_len::<u32>()?;

                let created_date = reader.read_object()?;
                let saved_date = reader.read_object()?;

                let page_count = reader.read_primitive::<u32>()?;

                let mut pages = Vec::with_capacity(page_count as usize);

                for _ in 0..page_count {
                    pages.push(reader.read_object()?);
                }

                Ok(Self {
                    title,
                    created_date,
                    saved_date,
                    pages,
                })
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Note;
    use crate::Page;
    use crate::Project;
    use crate::Text;
    use std::fs;
    use std::io::Cursor;
    use std::io::Seek;

    #[test]
    fn codec_for_version_0_0() {
        let image = fs::read(r"tests\images\0.png").unwrap();

        let project = Project::new()
            .with_title("工程")
            .with_page(
                Page::new(image.clone())
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_1_1"))
                            .with_text(Text::new().with_content("content_1_1_2")),
                    )
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_2_1"))
                            .with_text(Text::new().with_content("content_1_2_2"))
                            .with_text(Text::new().with_content("content_1_2_3")),
                    ),
            )
            .with_page(
                Page::new(image).with_note(
                    Note::new()
                        .with_text(Text::new().with_content("content_2_1_1"))
                        .with_text(Text::new().with_content("content_2_1_2")),
                ),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 0));

        writer.write_object(&project).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor).with_version((0, 0));

        let read_project = reader.read_object::<Project>().unwrap();

        assert!(read_project.title().is_empty());

        assert_eq!(read_project.created_date(), project.created_date());
        assert_eq!(read_project.saved_date(), project.saved_date());

        for (read_page, page) in read_project.pages().iter().zip(project.pages()) {
            assert_eq!(read_page.data(), page.data());

            for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
                assert!(read_note.x() - note.x() <= 0.1);
                assert!(read_note.y() - note.y() <= 0.1);

                assert_eq!(read_note.texts()[0].content(), note.merge_texts());
            }
        }
    }

    #[test]
    fn codec_for_version_0_1() {
        let image = fs::read(r"tests\images\0.png").unwrap();

        let project = Project::new()
            .with_title("工程")
            .with_page(
                Page::new(image.clone())
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_1_1"))
                            .with_text(Text::new().with_content("content_1_1_2")),
                    )
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_2_1"))
                            .with_text(Text::new().with_content("content_1_2_2"))
                            .with_text(Text::new().with_content("content_1_2_3")),
                    ),
            )
            .with_page(
                Page::new(image).with_note(
                    Note::new()
                        .with_text(Text::new().with_content("content_2_1_1"))
                        .with_text(Text::new().with_content("content_2_1_2")),
                ),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 1));

        writer.write_object(&project).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor).with_version((0, 1));

        let read_project = reader.read_object::<Project>().unwrap();

        assert!(read_project.title().is_empty());

        assert_eq!(read_project.created_date(), project.created_date());
        assert_eq!(read_project.saved_date(), project.saved_date());

        for (read_page, page) in read_project.pages().iter().zip(project.pages()) {
            assert_eq!(read_page.data(), page.data());

            for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
                assert!(read_note.x() - note.x() <= 0.1);
                assert!(read_note.y() - note.y() <= 0.1);

                assert_eq!(read_note.texts()[0].content(), note.merge_texts());
            }
        }
    }

    #[test]
    fn codec_for_version_0_2() {
        let image = fs::read(r"tests\images\0.png").unwrap();

        let project = Project::new()
            .with_title("工程")
            .with_page(
                Page::new(image.clone())
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_1_1"))
                            .with_text(Text::new().with_content("content_1_1_2")),
                    )
                    .with_note(
                        Note::new()
                            .with_text(Text::new().with_content("content_1_2_1"))
                            .with_text(Text::new().with_content("content_1_2_2"))
                            .with_text(Text::new().with_content("content_1_2_3")),
                    ),
            )
            .with_page(
                Page::new(image).with_note(
                    Note::new()
                        .with_text(Text::new().with_content("content_2_1_1"))
                        .with_text(Text::new().with_content("content_2_1_2")),
                ),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor).with_version((0, 2));

        writer.write_object(&project).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor).with_version((0, 2));

        assert_eq!(reader.read_object::<Project>().unwrap(), project);
    }
}
