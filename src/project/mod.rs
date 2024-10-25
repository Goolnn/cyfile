use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::Credit;
use crate::Date;
use crate::Note;
use crate::Page;
use crate::Text;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Default, PartialEq, Clone)]
pub struct Project {
    cover: Vec<u8>,

    category: String,
    title: String,

    number: (u32, u32),

    created_date: Date,
    saved_date: Date,

    credits: HashMap<Credit, HashSet<String>>,

    pages: Vec<Page>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            cover: Vec::new(),

            category: String::new(),
            title: String::new(),

            number: (0, 0),

            created_date: Date::now(),
            saved_date: Date::now(),

            credits: HashMap::new(),

            pages: Vec::new(),
        }
    }

    pub fn with_cover(mut self, cover: Vec<u8>) -> Self {
        self.cover = cover;

        self
    }

    pub fn with_category(mut self, category: impl ToString) -> Self {
        self.category = category.to_string();

        self
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_number(mut self, number: (u32, u32)) -> Self {
        self.number = number;

        self
    }

    pub fn with_credit(mut self, credit: Credit, name: impl ToString) -> Self {
        self.credits
            .entry(credit)
            .or_default()
            .insert(name.to_string());

        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);

        self
    }

    pub fn cover(&self) -> &[u8] {
        &self.cover
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn number(&self) -> (u32, u32) {
        self.number
    }

    pub fn created_date(&self) -> Date {
        self.created_date
    }

    pub fn saved_date(&self) -> Date {
        self.saved_date
    }

    pub fn credits_mut(&mut self) -> &mut HashMap<Credit, HashSet<String>> {
        &mut self.credits
    }

    pub fn credits(&self) -> &HashMap<Credit, HashSet<String>> {
        &self.credits
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

impl Decode for Project {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        match reader.version().into() {
            (0, 0) => {
                let pages = reader.read_object::<Vec<Page>>()?;

                let cover = if !pages.is_empty() {
                    pages[0].data().to_vec()
                } else {
                    Vec::new()
                };

                Ok(Self {
                    cover,
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

                        // 初稿数据
                        let draft_len = reader.read_primitive::<u16>()? as usize;

                        let mut draft_bytes = reader.read_bytes(draft_len)?;

                        draft_bytes.pop();

                        let draft = String::from_utf8(draft_bytes).unwrap();

                        // 校对数据
                        let revision_len = reader.read_primitive::<u16>()? as usize;

                        let mut revision_bytes = reader.read_bytes(revision_len)?;

                        revision_bytes.pop();

                        let revision = String::from_utf8(revision_bytes).unwrap();

                        if draft.contains("DOCTYPE HTML PUBLIC")
                            || revision.contains("DOCTYPE HTML PUBLIC")
                        {
                            // 解析 HTML 文本
                            static SPAN: Lazy<Regex> =
                                Lazy::new(|| Regex::new(r"<span.*?>|</span>").unwrap());

                            let draft = SPAN.replace_all(&draft, "").to_string();
                            let revision = SPAN.replace_all(&revision, "").to_string();

                            static PARA: Lazy<Regex> =
                                Lazy::new(|| Regex::new(r"<p.*?>(.*)</p>").unwrap());

                            let extract = |text| {
                                PARA.captures_iter(text)
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

                let cover = if !pages.is_empty() {
                    pages[0].data().to_vec()
                } else {
                    Vec::new()
                };

                Ok(Self {
                    cover,

                    created_date: date,
                    saved_date: date,

                    pages,

                    ..Self::default()
                })
            }

            (0, 2) => Ok(Self {
                cover: reader.read_bytes_with_len::<u32>()?,

                category: reader.read_string_with_len::<u32>()?,
                title: reader.read_string_with_len::<u32>()?,

                number: reader.read_object()?,

                created_date: reader.read_object()?,
                saved_date: reader.read_object()?,

                credits: reader.read_object()?,

                pages: reader.read_object()?,
            }),

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        }
    }
}

impl Decode for (u32, u32) {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let begin_number = reader.read_primitive()?;
        let ent_number = reader.read_primitive()?;

        Ok((begin_number, ent_number))
    }
}

impl Decode for HashMap<Credit, HashSet<String>> {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let mut credits = HashMap::new();

        let credit_count = reader.read_primitive::<u8>()?;

        for _ in 0..credit_count {
            let credit = reader.read_object()?;
            let names = reader.read_object()?;

            credits.insert(credit, names);
        }

        Ok(credits)
    }
}

impl Decode for HashSet<String> {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let mut names = HashSet::new();

        let name_count = reader.read_primitive::<u8>()?;

        for _ in 0..name_count {
            let name = reader.read_string_with_len::<u32>()?;

            names.insert(name);
        }

        Ok(names)
    }
}

impl Decode for Vec<Page> {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let page_count = match reader.version().into() {
            (0, 0) => reader.read_primitive::<u8>()? as usize,
            (0, 2) => reader.read_primitive::<u32>()? as usize,

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        };

        let mut pages = Vec::with_capacity(page_count);

        for _ in 0..page_count {
            pages.push(reader.read_object()?);
        }

        Ok(pages)
    }
}

impl Encode for Project {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        match writer.version().into() {
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
                writer.write_bytes_with_len::<u32>(&self.cover)?;

                writer.write_string_with_len::<u32>(&self.category)?;
                writer.write_string_with_len::<u32>(&self.title)?;

                writer.write_object(&self.number)?;

                writer.write_object(&self.created_date)?;
                writer.write_object(&self.saved_date)?;

                writer.write_object(&self.credits)?;

                writer.write_object(&self.pages)?;

                Ok(())
            }

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        }
    }
}

impl Encode for (u32, u32) {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.0)?;
        writer.write_primitive(self.1)?;

        Ok(())
    }
}

impl Encode for HashMap<Credit, HashSet<String>> {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.len() as u8)?;

        for (credit, names) in self.iter() {
            writer.write_object(credit)?;
            writer.write_object(names)?;
        }

        Ok(())
    }
}

impl Encode for HashSet<String> {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.len() as u8)?;

        for name in self.iter() {
            writer.write_string_with_len::<u32>(name)?;
        }

        Ok(())
    }
}

impl Encode for Vec<Page> {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        match writer.version().into() {
            (0, 0) => writer.write_primitive(self.len() as u8)?,
            (0, 2) => writer.write_primitive(self.len() as u32)?,

            version => anyhow::bail!(FileError::UnsupportedVersion {
                version: version.into()
            }),
        }

        for page in self.iter() {
            writer.write_object(page)?;
        }

        Ok(())
    }
}

impl Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field(
                "cover",
                if self.cover.is_empty() {
                    &"Empty"
                } else {
                    &"Exists"
                },
            )
            .field("category", &self.category)
            .field("title", &self.title)
            .field("number", &self.number)
            .field("created_date", &self.created_date)
            .field("saved_date", &self.saved_date)
            .field("credits", &self.credits)
            .field("pages", &self.pages)
            .finish()
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
        let image = fs::read(r"tests/images/0.png").unwrap();

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
        let image = fs::read(r"tests/images/0.png").unwrap();

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
        let image = fs::read(r"tests/images/0.png").unwrap();

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
