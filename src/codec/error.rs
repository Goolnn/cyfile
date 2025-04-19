use std::io;
use std::string;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to convert length")]
    InvalidLength,

    #[error("an I/O error occurred: {0}")]
    Io(#[from] io::Error),
    #[error("an invalid UTF-8 sequence was found: {0}")]
    Utf8(#[from] string::FromUtf8Error),
}
