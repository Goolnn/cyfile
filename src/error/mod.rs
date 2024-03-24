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

pub type FileResult<T> = Result<T, FileError>;
