mod export;
mod data;

pub use export::Export;

use crate::credit::Credits;

use regex::Regex;

use std::collections::VecDeque;

use std::path::Path;

use std::fs;

use crate::error::{
  FileResult,
  FileError,
};

use crate::{
  Encode,
  Decode,
  Pages,
  Codec,
  Text,
  Tags,
  Page,
  Note,
  Date,
};

use data::{
  HEADER_DATA,
  VERSION_LATEST,
  VERSIONS,
};

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

    // 解码文件数据
    Self::decode(&mut Codec::new(fs::File::open(path)?, filepath))
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
        // 图像数据
        self.pages.encode(codec)?;

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
        for page in self.pages.inner() {
          codec.write_object(page.source())?;
        }

        // 标记数据
        for page in self.pages.inner() {
          // 图像尺寸
          let (page_width, page_height) = page.size();

          // 标记数量
          codec.write_primitive(page.notes().len() as u8)?;

          for note in page.notes().inner() {
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
        codec.write_object(&self.tags)?;
        // 工作人员
        codec.write_object(&self.credits)?;

        // 创建时间
        codec.write_object(&self.created_date)?;
        // 保存时间
        codec.write_object(&self.saved_date)?;

        // 图像数据
        codec.write_object(&self.pages)?;

        Ok(())
      }

      _ => Err(FileError::InvalidVersion),
    }
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

        let mut pages = VecDeque::with_capacity(page_count as usize);

        for _ in 0..page_count {
          pages.push_back(codec.read_object()?);
        }

        Ok(Self {
          filepath: codec.filepath().to_string(),
          version: (major, minor),

          pages: Pages::from(pages),

          ..Self::default()
        })
      }

      (0, 1) => {
        // 图像数量
        let page_count = codec.read_primitive::<u8>()?;

        // 保存次数
        codec.read_primitive::<u8>()?;

        // 保存时间
        let date = codec.read_object()?;

        // 读取图像
        let mut pages = VecDeque::with_capacity(page_count as usize);

        for _ in 0..page_count {
          let image_data = codec.read_data_with_len::<u32>()?;

          pages.push_back(Page::with_source(image_data));
        }

        // 读取标记
        for i in 0..page_count {
          // 标记数量
          let note_count = codec.read_primitive::<u8>()?;

          let page = &mut pages[i as usize];

          let (page_width, page_height) = page.size();

          for _ in 0..note_count {
            let note_x = codec.read_primitive::<u16>()? as f64;
            let note_y = codec.read_primitive::<u16>()? as f64;

            let mut note = Note::with_coordinate(
              note_x / page_width as f64 * 2.0 - 1.0,
              1.0 - note_y / page_height as f64 * 2.0,
            );

            let draft = codec.read_string_with_nil::<u16>()?;
            let revision = codec.read_string_with_nil::<u16>()?;

            if draft.contains("DOCTYPE HTML PUBLIC") || revision.contains("DOCTYPE HTML PUBLIC") {
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
                note.texts_mut().push_back(Text::with_content(&draft));
              }

              if !revision.is_empty() {
                note.texts_mut().push_back(Text::with_content(&revision));
              }
            } else {
              // 添加文本
              if !draft.is_empty() {
                note.texts_mut().push_back(Text::with_content(&draft));
              }

              if !revision.is_empty() {
                note.texts_mut().push_back(Text::with_content(&revision));
              }
            }

            page.notes_mut().push_back(note);
          }
        }

        Ok(Self {
          filepath: codec.filepath().to_string(),
          version: (major, minor),

          created_date: date,
          saved_date: date,

          pages: Pages::from(pages),

          ..Self::default()
        })
      }

      (0, 2) => {
        // 分类标签
        let tags = codec.read_object()?;
        // 工作人员
        let credits = codec.read_object()?;

        // 创建时间
        let created_date = codec.read_object()?;
        // 保存时间
        let saved_date = codec.read_object()?;

        // 图像数据
        let pages = codec.read_object()?;

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
