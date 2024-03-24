use std::io;

#[derive(Debug)]
pub enum FileError {
  InvalidHeader,
  InvalidVersion,
  InvalidStructure,

  PathIsDirectory,
  PathNotExists,
  PathNotFile,

  Undefined,
}

impl From<io::Error> for FileError {
  fn from(_: io::Error) -> Self {
    Self::Undefined
  }
}

pub type FileResult<T> = Result<T, FileError>;
