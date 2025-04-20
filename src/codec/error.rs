use crate::Version;
use std::io;
use std::string;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to convert length")]
    InvalidLength,
    #[error("unsupported version: {version:?}")]
    UnsupportedVersion { version: Version },
    #[error("failed to parse image data")]
    InvalidImage,
    #[error("failed to parse text data")]
    ParseFailed,

    #[error("an I/O error occurred: {0}")]
    Io(#[from] io::Error),
    #[error("an utf-8 error occurred: {0}")]
    Utf8(#[from] string::FromUtf8Error),
}
