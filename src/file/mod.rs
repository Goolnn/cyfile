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

#[derive(Default)]
pub struct File {
  filepath: String,
  version: u16,

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

  pub fn version(&self) -> u16 {
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