use std::error::Error;
use std::fmt::Display;
use std::io;

pub type FileResult<T> = Result<T, FileError>;

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

impl Error for FileError {}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeader => write!(f, "invalid header"),
            Self::InvalidVersion => write!(f, "invalid version"),
            Self::InvalidStructure => write!(f, "invalid structure"),

            Self::PathIsDirectory => write!(f, "path is directory"),
            Self::PathNotExists => write!(f, "path not exists"),
            Self::PathNotFile => write!(f, "path not file"),

            Self::WriteFailed => write!(f, "write failed"),
            Self::ReadFailed => write!(f, "read failed"),

            Self::Undefined => write!(f, "undefined"),
        }
    }
}
