use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::file;
use crate::Date;
use crate::Note;
use crate::Page;
use crate::Text;
use scraper::Html;
use scraper::Selector;
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

    comment: String,

    created_date: Date,
    updated_date: Date,

    pages: Vec<Page>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            cover: Vec::new(),

            category: String::new(),
            title: String::new(),

            number: (0, 0),

            comment: String::new(),

            created_date: Date::now(),
            updated_date: Date::now(),

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

    pub fn with_comment(mut self, comment: impl ToString) -> Self {
        self.comment = comment.to_string();

        self
    }

    pub fn with_pages(mut self, pages: Vec<Page>) -> Self {
        self.pages = pages;

        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);

        self
    }

    pub fn set_cover(&mut self, cover: Vec<u8>) {
        self.cover = cover;
    }

    pub fn set_category(&mut self, category: impl ToString) {
        self.category = category.to_string();
    }

    pub fn set_title(&mut self, title: impl ToString) {
        self.title = title.to_string();
    }

    pub fn set_number(&mut self, number: (u32, u32)) {
        self.number = number;
    }

    pub fn set_comment(&mut self, comment: impl ToString) {
        self.comment = comment.to_string();
    }

    pub fn set_pages(&mut self, pages: Vec<Page>) {
        self.pages = pages;
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

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn created_date(&self) -> Date {
        self.created_date
    }

    pub fn updated_date(&self) -> Date {
        self.updated_date
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

impl Codec for Project {
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

                    let (page_width, page_height) = page.size()?;

                    for _ in 0..note_count {
                        let note_x = reader.read_primitive::<u16>()? as f64;
                        let note_y = reader.read_primitive::<u16>()? as f64;

                        let note = Note::new().with_coordinate(
                            note_x / page_width as f64 * 2.0 - 1.0,
                            1.0 - note_y / page_height as f64 * 2.0,
                        );

                        // 初稿数据
                        let draft_len = reader.read_primitive::<u16>()? as usize;

                        let mut draft_bytes = reader.read_bytes(draft_len)?;

                        draft_bytes.pop();

                        let mut draft = String::from_utf8(draft_bytes).unwrap();

                        // 校对数据
                        let revision_len = reader.read_primitive::<u16>()? as usize;

                        let mut revision_bytes = reader.read_bytes(revision_len)?;

                        revision_bytes.pop();

                        let mut revision = String::from_utf8(revision_bytes).unwrap();

                        // 判断是否为 HTML
                        let draft_is_html = draft.contains("DOCTYPE HTML PUBLIC");
                        let revision_is_html = revision.contains("DOCTYPE HTML PUBLIC");

                        if draft_is_html || revision_is_html {
                            let draft_parser = Html::parse_document(&draft);
                            let revision_parser = Html::parse_document(&revision);

                            let selector = Selector::parse("p").unwrap();

                            draft = draft_parser
                                .select(&selector)
                                .map(|paragraph| paragraph.text().collect::<String>())
                                .collect::<Vec<String>>()
                                .join("\n");

                            revision = revision_parser
                                .select(&selector)
                                .map(|paragraph| paragraph.text().collect::<String>())
                                .collect::<Vec<String>>()
                                .join("\n");
                        }

                        let mut text = Text::new();

                        if !draft.is_empty() {
                            text.set_content(draft);
                        }

                        if !revision.is_empty() {
                            text.set_comment(revision);
                        }

                        if text.content().is_empty() && text.comment().is_empty() {
                            continue;
                        }

                        page.notes_mut().push(note.with_text(text));
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
                    updated_date: date,

                    pages,

                    ..Self::default()
                })
            }

            (0, 2) => Ok(Self {
                cover: reader.read_bytes_with_len::<u32>()?,

                category: reader.read_string_with_len::<u32>()?,
                title: reader.read_string_with_len::<u32>()?,

                number: reader.read_object()?,

                comment: String::new(),

                created_date: reader.read_object()?,
                updated_date: reader.read_object()?,

                pages: reader.read_object()?,
            }),

            version => anyhow::bail!(file::Error::UnsupportedVersion {
                version: version.into()
            }),
        }
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        match writer.version().into() {
            (0, 0) => {
                writer.write_objects::<u8, Page>(&self.pages)?;

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
                    let (page_width, page_height) = page.size()?;

                    // 标记数量
                    writer.write_primitive(page.notes().len() as u8)?;

                    for note in page.notes() {
                        let note_x = (page_width as f64 * ((note.x() + 1.0) / 2.0)) as u16;
                        let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

                        writer.write_primitive(note_x)?;
                        writer.write_primitive(note_y)?;

                        // 合并文本
                        let (content, comment) = note.merge_texts();

                        // 初译数据
                        writer.write_primitive(content.len() as u16 + 1)?;
                        writer.write_string_with_nil(&content)?;
                        // 校对数据
                        writer.write_primitive(comment.len() as u16 + 1)?;
                        writer.write_string_with_nil(&comment)?;
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
                writer.write_object(&self.updated_date)?;

                writer.write_object(&self.pages)?;

                Ok(())
            }

            version => anyhow::bail!(file::Error::UnsupportedVersion {
                version: version.into()
            }),
        }
    }
}

impl Codec for (u32, u32) {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let begin_number = reader.read_primitive()?;
        let ent_number = reader.read_primitive()?;

        Ok((begin_number, ent_number))
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.0)?;
        writer.write_primitive(self.1)?;

        Ok(())
    }
}

impl Codec for HashSet<String> {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let mut names = HashSet::new();

        let name_count = reader.read_primitive::<u8>()?;

        for _ in 0..name_count {
            let name = reader.read_string_with_len::<u32>()?;

            names.insert(name);
        }

        Ok(names)
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.len() as u8)?;

        for name in self.iter() {
            writer.write_string_with_len::<u32>(name)?;
        }

        Ok(())
    }
}

impl Codec for Vec<Page> {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let pages = match reader.version().into() {
            (0, 0) => reader.read_objects::<u8, Page>(),
            (0, 2) => reader.read_objects::<u32, Page>(),

            version => anyhow::bail!(file::Error::UnsupportedVersion {
                version: version.into()
            }),
        }?;

        Ok(pages)
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        match writer.version().into() {
            (0, 0) => writer.write_objects::<u8, Page>(self)?,
            (0, 2) => writer.write_objects::<u32, Page>(self)?,

            version => anyhow::bail!(file::Error::UnsupportedVersion {
                version: version.into()
            }),
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
            .field("updated_date", &self.updated_date)
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
    fn new() {
        let project = Project::new();

        assert!(project.cover().is_empty());

        assert!(project.category().is_empty());
        assert!(project.title().is_empty());

        assert_eq!(project.number(), (0, 0));

        assert!(project.comment().is_empty());

        assert!(project.pages().is_empty());
    }

    #[test]
    fn with_cover() {
        let cover = fs::read(r"tests/images/0.png").unwrap();
        let project = Project::new().with_cover(cover.clone());

        assert_eq!(project.cover(), cover.as_slice());
    }

    #[test]
    fn with_category() {
        let project = Project::new().with_category("类别");

        assert_eq!(project.category(), "类别");
    }

    #[test]
    fn with_title() {
        let project = Project::new().with_title("标题");

        assert_eq!(project.title(), "标题");
    }

    #[test]
    fn with_number() {
        let project = Project::new().with_number((1, 2));

        assert_eq!(project.number(), (1, 2));
    }

    #[test]
    fn with_comment() {
        let project = Project::new().with_comment("备注");

        assert_eq!(project.comment(), "备注");
    }

    #[test]
    fn with_pages() {
        let image = fs::read(r"tests/images/0.png").unwrap();

        let project = Project::new().with_pages(vec![
            Page::new(image.clone()).with_note(
                Note::new()
                    .with_text(Text::new().with_content("content_1_1_1"))
                    .with_text(Text::new().with_content("content_1_1_2")),
            ),
            Page::new(image.clone()).with_note(
                Note::new()
                    .with_text(Text::new().with_content("content_2_1_1"))
                    .with_text(Text::new().with_content("content_2_1_2")),
            ),
        ]);

        assert_eq!(project.pages().len(), 2);

        assert_eq!(project.pages()[0].data(), image.as_slice());
        assert_eq!(project.pages()[1].data(), image.as_slice());

        assert_eq!(project.pages()[0].notes().len(), 1);
        assert_eq!(project.pages()[1].notes().len(), 1);

        assert_eq!(project.pages()[0].notes()[0].texts().len(), 2);
        assert_eq!(project.pages()[1].notes()[0].texts().len(), 2);

        assert_eq!(
            project.pages()[0].notes()[0].texts()[0].content(),
            "content_1_1_1"
        );
        assert_eq!(
            project.pages()[0].notes()[0].texts()[1].content(),
            "content_1_1_2"
        );

        assert_eq!(
            project.pages()[1].notes()[0].texts()[0].content(),
            "content_2_1_1"
        );
        assert_eq!(
            project.pages()[1].notes()[0].texts()[1].content(),
            "content_2_1_2"
        );
    }

    #[test]
    fn with_page() {
        let image = fs::read(r"tests/images/0.png").unwrap();

        let project = Project::new()
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
                Page::new(image.clone()).with_note(
                    Note::new()
                        .with_text(Text::new().with_content("content_2_1_1"))
                        .with_text(Text::new().with_content("content_2_1_2")),
                ),
            );

        assert_eq!(project.pages().len(), 2);

        assert_eq!(project.pages()[0].data(), image.as_slice());
        assert_eq!(project.pages()[1].data(), image.as_slice());

        assert_eq!(project.pages()[0].notes().len(), 2);
        assert_eq!(project.pages()[1].notes().len(), 1);

        assert_eq!(project.pages()[0].notes()[0].texts().len(), 2);
        assert_eq!(project.pages()[0].notes()[1].texts().len(), 3);

        assert_eq!(project.pages()[1].notes()[0].texts().len(), 2);

        assert_eq!(
            project.pages()[0].notes()[0].texts()[0].content(),
            "content_1_1_1"
        );
        assert_eq!(
            project.pages()[0].notes()[0].texts()[1].content(),
            "content_1_1_2"
        );

        assert_eq!(
            project.pages()[0].notes()[1].texts()[0].content(),
            "content_1_2_1"
        );
        assert_eq!(
            project.pages()[0].notes()[1].texts()[1].content(),
            "content_1_2_2"
        );
        assert_eq!(
            project.pages()[0].notes()[1].texts()[2].content(),
            "content_1_2_3"
        );

        assert_eq!(
            project.pages()[1].notes()[0].texts()[0].content(),
            "content_2_1_1"
        );
        assert_eq!(
            project.pages()[1].notes()[0].texts()[1].content(),
            "content_2_1_2"
        );
    }

    #[test]
    fn set_cover() {
        let cover = fs::read(r"tests/images/0.png").unwrap();
        let mut project = Project::new();

        project.set_cover(cover.clone());

        assert_eq!(project.cover(), cover.as_slice());
    }

    #[test]
    fn set_category() {
        let mut project = Project::new();

        project.set_category("类别");

        assert_eq!(project.category(), "类别");
    }

    #[test]
    fn set_title() {
        let mut project = Project::new();

        project.set_title("标题");

        assert_eq!(project.title(), "标题");
    }

    #[test]
    fn set_number() {
        let mut project = Project::new();

        project.set_number((1, 2));

        assert_eq!(project.number(), (1, 2));
    }

    #[test]
    fn set_comment() {
        let mut project = Project::new();

        project.set_comment("备注");

        assert_eq!(project.comment(), "备注");
    }

    #[test]
    fn set_pages() {
        let image = fs::read(r"tests/images/0.png").unwrap();

        let mut project = Project::new();

        project.set_pages(vec![
            Page::new(image.clone()).with_note(
                Note::new()
                    .with_text(Text::new().with_content("content_1_1_1"))
                    .with_text(Text::new().with_content("content_1_1_2")),
            ),
            Page::new(image.clone()).with_note(
                Note::new()
                    .with_text(Text::new().with_content("content_2_1_1"))
                    .with_text(Text::new().with_content("content_2_1_2")),
            ),
        ]);

        assert_eq!(project.pages().len(), 2);

        assert_eq!(project.pages()[0].data(), image.as_slice());
        assert_eq!(project.pages()[1].data(), image.as_slice());

        assert_eq!(project.pages()[0].notes().len(), 1);
        assert_eq!(project.pages()[1].notes().len(), 1);

        assert_eq!(project.pages()[0].notes()[0].texts().len(), 2);
        assert_eq!(project.pages()[1].notes()[0].texts().len(), 2);

        assert_eq!(
            project.pages()[0].notes()[0].texts()[0].content(),
            "content_1_1_1"
        );
        assert_eq!(
            project.pages()[0].notes()[0].texts()[1].content(),
            "content_1_1_2"
        );

        assert_eq!(
            project.pages()[1].notes()[0].texts()[0].content(),
            "content_2_1_1"
        );
        assert_eq!(
            project.pages()[1].notes()[0].texts()[1].content(),
            "content_2_1_2"
        );
    }

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
        assert_eq!(read_project.updated_date(), project.updated_date());

        for (read_page, page) in read_project.pages().iter().zip(project.pages()) {
            assert_eq!(read_page.data(), page.data());

            for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
                assert!(read_note.x() - note.x() <= 0.1);
                assert!(read_note.y() - note.y() <= 0.1);

                let (content, comment) = note.merge_texts();

                assert_eq!(read_note.texts()[0].content(), content);
                assert_eq!(read_note.texts()[0].comment(), comment);
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
        assert_eq!(read_project.updated_date(), project.updated_date());

        for (read_page, page) in read_project.pages().iter().zip(project.pages()) {
            assert_eq!(read_page.data(), page.data());

            for (read_note, note) in read_page.notes().iter().zip(page.notes()) {
                assert!(read_note.x() - note.x() <= 0.1);
                assert!(read_note.y() - note.y() <= 0.1);

                let (content, comment) = note.merge_texts();

                assert_eq!(read_note.texts()[0].content(), content);
                assert_eq!(read_note.texts()[0].comment(), comment);
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
