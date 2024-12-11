use crate::Version;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid header")]
    InvalidHeader,

    #[error("unsupported version: {version:?}")]
    UnsupportedVersion { version: Version },

    #[error("\"{}\" is directory", path.display())]
    PathIsDirectory { path: PathBuf },
}
