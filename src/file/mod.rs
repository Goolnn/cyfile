use crate::credit::Credit;
use crate::page::Page;
use crate::date::Date;

use crate::error::{
  FileResult,
  FileError,
};

use std::collections::{
  HashSet,
  HashMap,
};

use std::path::Path;

// 头部数据
const HEADER_DATA: [u8; 15] = [0xE8, 0x8B, 0x8D, 0xE7, 0x9C, 0xBC, 0xE6, 0xB1, 0x89, 0xE5, 0x8C, 0x96, 0xE7, 0xBB, 0x84];

// 版本数据
const VERSION_0_0: [u8; 2] = [0x00, 0x00];
const VERSION_0_1: [u8; 2] = [0x00, 0x01];
const VERSION_0_2: [u8; 2] = [0x00, 0x02];

const VERSION_LATEST: [u8; 2] = VERSION_0_2;

const VERSIONS: [[u8; 2]; 3] = [
  VERSION_0_0,
  VERSION_0_1,
  VERSION_0_2,
];

#[derive(Default)]
pub struct File {
  filepath: String,
  version: (u8, u8),

  tags: HashSet<String>,

  created_date: Date,
  modified_date: Date,

  credits: HashMap<Credit, HashSet<String>>,

  pages: Vec<Page>,
}

impl File {
  pub fn new(filepath: &str) -> FileResult<Self> {
    let path = Path::new(filepath);

    if path.exists() && path.is_dir() {
      return Err(FileError::PathIsDirectory);
    }

    Ok(Self {
      filepath: filepath.to_string(),

      ..Self::default()
    })
  }

  pub fn filepath(&self) -> &str {
    &self.filepath
  }

  pub fn version(&self) -> (u8, u8) {
    self.version
  }

  pub fn tags_mut(&mut self) -> &mut HashSet<String> {
    &mut self.tags
  }

  pub fn tags(&self) -> &HashSet<String> {
    &self.tags
  }

  pub fn created_date(&self) -> Date {
    self.created_date
  }

  pub fn modified_date(&self) -> Date {
    self.modified_date
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

  pub fn pages(&mut self) -> &Vec<Page> {
    &self.pages
  }
}