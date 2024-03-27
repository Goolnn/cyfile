pub(crate) mod codec;

use crate::credit::Credit;
use crate::text::Text;
use crate::page::Page;
use crate::note::Note;
use crate::date::Date;

use regex::Regex;

use std::path::Path;

use std::fs;

use crate::error::{
  FileResult,
  FileError,
};

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

use std::collections::{
  HashSet,
  HashMap,
};

use std::fmt::{
  Formatter,
  Debug,
};

type Credits = HashMap<Credit, HashSet<String>>;
type Pages = Vec<Page>;
type Tags = HashSet<String>;

// 头部数据
const HEADER_DATA: [u8; 15] = [0xE8, 0x8B, 0x8D, 0xE7, 0x9C, 0xBC, 0xE6, 0xB1, 0x89, 0xE5, 0x8C, 0x96, 0xE7, 0xBB, 0x84];

// 版本数据
const VERSION_0_0: (u8, u8) = (0x00, 0x00);
const VERSION_0_1: (u8, u8) = (0x00, 0x01);
const VERSION_0_2: (u8, u8) = (0x00, 0x02);

const VERSION_LATEST: (u8, u8) = VERSION_0_2;

const VERSIONS: [(u8, u8); 3] = [
  VERSION_0_0,
  VERSION_0_1,
  VERSION_0_2,
];

pub trait Export {
  fn export_to_with_version(&mut self, filepath: &str, version: (u8, u8)) -> FileResult<()>;
  fn export_with_version(&mut self, version: (u8, u8)) -> FileResult<()>;
  fn export_to(&mut self, filepath: &str) -> FileResult<()>;
  fn export(&mut self) -> FileResult<()>;
}

#[derive(Default)]
pub struct File {
  filepath: String,
  version: (u8, u8),

  tags: Tags,
  credits: Credits,

  created_date: Date,
  saved_date: Date,

  pages: Pages,
}

impl File {
  pub fn open(filepath: &str) -> FileResult<Self> {
    let path = Path::new(filepath);

    // 判断路径是否存在
    if !path.exists() {
      return Err(FileError::PathNotExists);
    }

    // 判断路径是否是文件
    if !path.is_file() {
      return Err(FileError::PathNotFile);
    }

    // 创建编解码器
    let mut codec = Codec::new(fs::File::open(path)?, filepath);

    // 解码文件数据
    Self::decode(&mut codec)
  }

  pub fn create(filepath: &str) -> FileResult<Self> {
    let path = Path::new(filepath);

    // 判断文件是否为路径
    if path.exists() && path.is_dir() {
      return Err(FileError::PathIsDirectory);
    }

    Ok(Self {
      filepath: filepath.to_string(),
      version: VERSION_LATEST,

      ..Self::default()
    })
  }

  pub fn filepath(&self) -> &str {
    &self.filepath
  }

  pub fn version(&self) -> (u8, u8) {
    self.version
  }

  pub fn tags_mut(&mut self) -> &mut Tags {
    &mut self.tags
  }

  pub fn tags(&self) -> &Tags {
    &self.tags
  }

  pub fn created_date(&self) -> Date {
    self.created_date
  }

  pub fn saved_date(&self) -> Date {
    self.saved_date
  }

  pub fn credits_mut(&mut self) -> &mut Credits {
    &mut self.credits
  }

  pub fn credits(&self) -> &Credits {
    &self.credits
  }

  pub fn pages_mut(&mut self) -> &mut Pages {
    &mut self.pages
  }

  pub fn pages(&self) -> &Pages {
    &self.pages
  }
}

impl Export for File {
  fn export_to_with_version(&mut self, filepath: &str, version: (u8, u8)) -> FileResult<()> {
    self.filepath = filepath.to_string();
    self.version = version;

    self.saved_date = Date::now();

    self.encode(&mut Codec::with_version(fs::File::create(filepath)?, filepath, version))?;

    Ok(())
  }

  fn export_with_version(&mut self, version: (u8, u8)) -> FileResult<()> {
    self.export_to_with_version(&self.filepath.clone(), version)
  }

  fn export_to(&mut self, filepath: &str) -> FileResult<()> {
    self.export_to_with_version(filepath, self.version)
  }

  fn export(&mut self) -> FileResult<()> {
    self.export_to_with_version(&self.filepath.clone(), self.version)
  }
}

impl Encode for File {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    // 写入文件头部数据
    codec.write_data(&HEADER_DATA)?;

    // 写入文件版本数据
    let (major, minor) = codec.version();

    codec.write_primitive(major)?;
    codec.write_primitive(minor)?;

    match (major, minor) {
      (0, 0) => {
        // 图像数量
        codec.write_primitive(self.pages.len() as u8)?;

        for page in &self.pages {
          page.encode(codec)?;
        }

        Ok(())
      }

      (0, 1) => {
        // 图像数量
        codec.write_primitive(self.pages.len() as u8)?;

        // 保存次数
        codec.write_primitive(1_u8)?;

        // 保存时间
        Date::now().encode(codec)?;

        // 图像数据
        for page in &self.pages {
          codec.write_data_with_len::<u32>(page.raw())?;
        }

        // 标记数据
        for page in &self.pages {
          // 图像尺寸
          let (page_width, page_height) = page.size();

          // 标记数量
          codec.write_primitive(page.notes().len() as u8)?;

          for note in page.notes() {
            let note_x = (page_width as f64 * ((note.x() + 1.0) / 2.0)) as u16;
            let note_y = (page_height as f64 * (1.0 - (note.y() + 1.0) / 2.0)) as u16;

            codec.write_primitive(note_x)?;
            codec.write_primitive(note_y)?;

            // 合并文本
            let merged_text = note.merge_texts();

            // 初译数据
            codec.write_string_with_nil::<u16>(&merged_text)?;
            // 校对数据
            codec.write_string_with_nil::<u16>("")?;
          }
        }

        Ok(())
      }

      (0, 2) => {
        // 分类标签
        self.tags.encode(codec)?;
        // 工作人员
        self.credits.encode(codec)?;
        
        // 创建时间
        self.created_date.encode(codec)?;
        // 保存时间
        self.saved_date.encode(codec)?;
        
        // 图像数据
        self.pages.encode(codec)?;

        Ok(())
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Encode for Credits {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    // 职位数量
    codec.write_primitive(self.len() as u32)?;

    for (&credit, stuffs) in self {
      codec.write_primitive(credit as u8)?;

      // 人员数量
      codec.write_primitive(stuffs.len() as u32)?;

      for stuff in stuffs {
        codec.write_string::<u32>(stuff)?;
      }
    }

    Ok(())
  }
}

impl Encode for Pages {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    codec.write_primitive::<u32>(self.len() as u32)?;
    
    for page in self {
      page.encode(codec)?;
    }

    Ok(())
  }
}

impl Encode for Tags {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    // 标签数量
    codec.write_primitive(self.len() as u32)?;

    // 标签数据
    for tag in self {
      codec.write_string::<u32>(tag)?;
    }

    Ok(())
  }
}

impl Decode for File {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    // 头部数据
    let header_data = codec.read_data(15)?;

    if header_data != HEADER_DATA {
      return Err(FileError::InvalidHeader);
    }

    // 版本数据
    let (major, minor) = (codec.read_primitive::<u8>()?, codec.read_primitive::<u8>()?);

    if !VERSIONS.contains(&(major, minor)) {
      return Err(FileError::InvalidVersion);
    }

    codec.set_version((major, minor));

    match (major, minor) {
      (0, 0) => {
        let page_count = codec.read_primitive::<u8>()?;

        let mut pages = Vec::with_capacity(page_count as usize);

        for _ in 0..page_count {
          pages.push(Page::decode(codec)?);
        }

        Ok(Self {
          filepath: codec.filepath().to_string(),
          version: (major, minor),

          pages,

          ..Self::default()
        })
      }

      (0, 1) => {
        // 图像数量
        let page_count = codec.read_primitive::<u8>()?;

        let mut pages = Vec::with_capacity(page_count as usize);

        // 保存次数
        codec.read_primitive::<u8>()?;

        // 保存时间
        let date = Date::decode(codec)?;

        // 读取图像
        for _ in 0..page_count {
          let image_data = codec.read_data_with_len::<u32>()?;

          pages.push(Page::new(image_data));
        }

        // 读取标签
        for i in 0..page_count {
          // 标签数量
          let note_count = codec.read_primitive::<u8>()?;

          let page = &mut pages[i as usize];

          page.notes_mut().reserve(note_count as usize);

          let (page_width, page_height) = page.size();

          for _ in 0..note_count {
            let note_x = codec.read_primitive::<u16>()? as f64;
            let note_y = codec.read_primitive::<u16>()? as f64;

            let mut note = Note::with_coordinate(
              note_x / page_width as f64 * 2.0 - 1.0,
              1.0 - note_y / page_height as f64 * 2.0,
            );

            let draft = codec.read_string::<u16>()?;
            let revision = codec.read_string::<u16>()?;

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
              regex.captures_iter(text).map(|capture| {
                let (_, [text]) = capture.extract();

                if text == "<br />" {
                  String::new()
                } else {
                  text.replace("<br />", "\n")
                }
              }).collect::<Vec<String>>().join("\n")
            };

            let draft = extract(&draft);
            let revision = extract(&revision);

            // 添加文本
            if !draft.is_empty() {
              note.texts_mut().push(Text::with_content(&draft));
            }

            if !revision.is_empty() {
              note.texts_mut().push(Text::with_content(&revision));
            }

            page.notes_mut().push(note);
          }
        }

        Ok(Self {
          filepath: codec.filepath().to_string(),
          version: (major, minor),

          created_date: date,
          saved_date: date,

          pages,

          ..Self::default()
        })
      }

      (0, 2) => {
        // 分类标签
        let tags = Tags::decode(codec)?;
        // 工作人员
        let credits = Credits::decode(codec)?;

        // 创建时间
        let created_date = Date::decode(codec)?;
        // 保存时间
        let saved_date = Date::decode(codec)?;

        // 图像数据
        let pages = Pages::decode(codec)?;

        Ok(Self {
          filepath: codec.filepath().to_string(),
          version: (major, minor),

          tags,
          credits,

          created_date,
          saved_date,

          pages,
        })
      }

      _ => Err(FileError::InvalidVersion),
    }
  }
}

impl Decode for Credits {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    // 职位数量
    let credit_count = codec.read_primitive::<u32>()?;

    let mut credits = Self::with_capacity(credit_count as usize);

    for _ in 0..credit_count {
      let credit = Credit::from(codec.read_primitive::<u8>()?);

      // 人员数量
      let stuff_count = codec.read_primitive::<u32>()?;

      let mut stuffs = HashSet::with_capacity(stuff_count as usize);

      for _ in 0..stuff_count {
        let stuff = codec.read_string::<u32>()?;

        stuffs.insert(stuff);
      }

      credits.insert(credit, stuffs);
    }

    Ok(credits)
  }
}

impl Decode for Pages {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    let page_count = codec.read_primitive::<u32>()?;

    let mut pages = Vec::with_capacity(page_count as usize);

    for _ in 0..page_count {
      pages.push(Page::decode(codec)?);
    }

    Ok(pages)
  }
}

impl Decode for Tags {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    // 标签数量
    let note_count = codec.read_primitive::<u32>()?;

    let mut tags = Self::with_capacity(note_count as usize);

    // 标签数据
    for _ in 0..note_count {
      let tag = codec.read_string::<u32>()?;

      tags.insert(tag);
    }

    Ok(tags)
  }
}

impl Debug for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Filepath: {}", self.filepath)?;
    writeln!(f, "Version: {:?}", self.version)?;

    writeln!(f)?;

    writeln!(f, "Tags[{}]: [{}]", self.tags.len(), &self.tags.iter().map(|tag| tag.to_string()).collect::<Vec<String>>().join("/"))?;

    writeln!(f)?;

    writeln!(f, "Created Date: {:?}", self.created_date)?;
    writeln!(f, "Saved Date: {:?}", self.saved_date)?;

    writeln!(f)?;

    write!(f, "Credits[{}]:", self.credits.len())?;

    if self.credits.is_empty() {
      writeln!(f, " {{}}")?;
    } else {
      writeln!(f, " {{\n{}\n}}", self.credits.iter().map(|(credit, stuffs)| {
        format!("  {:?}[{}]: [{}],", credit, stuffs.len(), stuffs.iter().map(|stuff| stuff.to_string()).collect::<Vec<String>>().join("/"))
      }).collect::<Vec<String>>().join("\n"))?;
    }

    writeln!(f)?;

    writeln!(f, "Pages[{}]:", self.pages.len())?;
    write!(f, "{}", &self.pages.iter().enumerate().map(|(index, page)| format!("* {}\n{:?}", index + 1, page).lines().map(|line| format!("  {}", line)).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>().join("\n\n"))?;

    Ok(())
  }
}