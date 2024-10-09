pub mod constants;

mod arguments;

pub use arguments::ExportArguments;

use crate::codec::Writer;
use crate::error::FileError;
use crate::error::FileResult;
use crate::Project;
use constants::HEADER_DATA;
use constants::VERSIONS;
use std::fs::File;
use std::io::Write;

pub struct Exporter {
    project: Project,
    arguments: ExportArguments,
}

impl Exporter {
    pub fn new(project: Project, arguments: ExportArguments) -> Self {
        Self { project, arguments }
    }

    pub fn export(&self) -> FileResult<()> {
        if self.arguments.filepath.is_dir() {
            return Err(FileError::PathIsDirectory);
        }

        let mut file = File::create(&self.arguments.filepath)?;

        // 写入头部数据
        file.write_all(&HEADER_DATA)?;

        // 写入版本数据
        if !VERSIONS.contains(&self.arguments.version) {
            return Err(FileError::InvalidVersion);
        }

        let version = [self.arguments.version.0, self.arguments.version.1];

        file.write_all(&version)?;

        // 写入项目数据
        let mut writer = Writer::with_version(file, self.arguments.version);

        writer.write_object(&self.project)?;

        Ok(())
    }
}
