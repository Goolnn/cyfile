use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("invalid header")]
    InvalidHeader,

    #[error("unsupported version: {major}.{minor}")]
    UnsupportedVersion { major: u8, minor: u8 },

    #[error("\"{}\" is directory", path.display())]
    PathIsDirectory { path: PathBuf },

    #[error("\"{}\" is not a file", path.display())]
    PathNotFile { path: PathBuf },
}
