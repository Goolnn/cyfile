use crate::codec;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path `{}` does not exist", path.display())]
    PathNotExist { path: PathBuf },

    #[error("path `{}` is not a file", path.display())]
    PathNotFile { path: PathBuf },

    #[error("permission denied to access path `{}`", path.display())]
    PermissionDenied { path: PathBuf },

    #[error("invalid archive format")]
    InvalidFormat,

    #[error("unsupported archive format")]
    UnsupportedFormat,

    #[error("password is not correct")]
    PasswordNotCorrect,

    #[error("file `{}` not found in archive", file)]
    FileNotFound { file: String },

    #[error("failed to parse file `{}` at line {}, column {}", file, line, column)]
    ParseFailure {
        file: String,

        line: usize,
        column: usize,
    },

    #[error("{}", source)]
    CodecError {
        #[from]
        source: codec::Error,
    },

    #[error("undefined error")]
    Undefined,
}
