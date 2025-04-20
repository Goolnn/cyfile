use crate::codec;
use crate::Version;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid header")]
    InvalidHeader,
    #[error("unsupported version: {version:?}")]
    UnsupportedVersion { version: Version },
    #[error("\"{}\" is directory", path.display())]
    PathIsDirectory { path: PathBuf },

    #[error("an image error occurred: {0}")]
    Image(#[from] image::ImageError),
    #[error("an io error occurred: {0}")]
    Io(#[from] io::Error),
    #[error("an codec error occurred: {0}")]
    Codec(#[from] codec::Error),
}
