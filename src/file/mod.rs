pub mod constants;

mod arguments;

pub use arguments::ExportArguments;

use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::file::constants::HEADER_DATA;
use crate::file::constants::VERSIONS;
use crate::Project;
use std::fs;
use std::io::Read;
use std::io::Write;

pub struct File {
    project: Project,
}

impl File {
    pub fn create(project: Project) -> File {
        File { project }
    }

    pub fn open(mut stream: impl Read) -> anyhow::Result<File> {
        let mut header = [0u8; 15];
        let mut version = [0u8; 2];

        stream.read_exact(&mut header)?;
        stream.read_exact(&mut version)?;

        if header != constants::HEADER_DATA {
            anyhow::bail!(FileError::InvalidHeader);
        }

        let version = (version[0], version[1]);

        if !constants::VERSIONS.contains(&version) {
            anyhow::bail!(FileError::UnsupportedVersion {
                major: version.0,
                minor: version.1
            });
        }

        let mut reader = Reader::new(stream).with_version(version);

        let project = reader.read_object()?;

        Ok(File { project })
    }

    pub fn project_mut(&mut self) -> &mut Project {
        &mut self.project
    }

    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn export(&self, arguments: ExportArguments) -> anyhow::Result<()> {
        if arguments.filepath.is_dir() {
            anyhow::bail!(FileError::PathIsDirectory {
                path: arguments.filepath
            })
        }

        let mut file = fs::File::create(&arguments.filepath)?;

        // 写入头部数据
        file.write_all(&HEADER_DATA)?;

        // 写入版本数据
        if !VERSIONS.contains(&arguments.version) {
            anyhow::bail!(FileError::UnsupportedVersion {
                major: arguments.version.0,
                minor: arguments.version.1
            });
        }

        let version = [arguments.version.0, arguments.version.1];

        file.write_all(&version)?;

        // 写入项目数据
        let mut writer = Writer::new(file).with_version(arguments.version);

        writer.write_object(&self.project)?;

        Ok(())
    }
}
