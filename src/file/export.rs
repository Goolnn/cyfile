use crate::file::constants::VERSION_LATEST;
use std::path::Path;
use std::path::PathBuf;

pub struct ExportArguments {
    pub(super) filepath: PathBuf,
    pub(super) version: (u8, u8),
}

impl ExportArguments {
    pub fn new(filepath: impl AsRef<Path>) -> Self {
        Self {
            filepath: filepath.as_ref().to_path_buf(),
            version: VERSION_LATEST,
        }
    }

    pub fn with_version(mut self, version: (u8, u8)) -> Self {
        self.version = version;

        self
    }
}
