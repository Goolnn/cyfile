use crate::file::constants::VERSION_LATEST;
use crate::Version;
use std::path::Path;
use std::path::PathBuf;

pub struct ExportArguments {
    pub(super) filepath: PathBuf,
    pub(super) version: Version,
}

impl ExportArguments {
    pub fn new(filepath: impl AsRef<Path>) -> Self {
        Self {
            filepath: filepath.as_ref().to_path_buf(),
            version: VERSION_LATEST.into(),
        }
    }

    pub fn with_version(mut self, version: impl Into<Version>) -> Self {
        self.version = version.into();

        self
    }
}
