use std::io;

#[derive(Debug, Eq, PartialEq)]
pub enum FileError {
  InvalidHeader,
  InvalidVersion,
  InvalidStructure,

  PathIsDirectory,
  PathNotExists,
  PathNotFile,

  WriteFailed,
  ReadFailed,

  Undefined,
}

impl From<io::Error> for FileError {
  fn from(_: io::Error) -> Self {
    Self::Undefined
  }
}

pub type FileResult<T> = Result<T, FileError>;
