mod data;
mod export;
mod version;

pub use data::HEADER_DATA;
pub use data::VERSIONS;
pub use data::VERSION_LATEST;
pub use export::ExportArguments;
pub use version::Version;

use crate::codec::Reader;
use crate::codec::Writer;
use crate::error::FileError;
use crate::Project;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct File;

impl File {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Project> {
        let mut stream = fs::File::open(path)?;

        let mut header = [0u8; 15];
        let mut version = [0u8; 2];

        stream.read_exact(&mut header)?;
        stream.read_exact(&mut version)?;

        if header != data::HEADER_DATA {
            anyhow::bail!(FileError::InvalidHeader);
        }

        let version = Version::from(version);

        if !data::VERSIONS.contains(&version.into()) {
            anyhow::bail!(FileError::UnsupportedVersion { version });
        }

        let mut reader = Reader::new(stream).with_version(version);

        let project = reader.read_object()?;

        Ok(project)
    }

    pub fn export(project: &Project, arguments: ExportArguments) -> anyhow::Result<()> {
        if arguments.filepath.is_dir() {
            anyhow::bail!(FileError::PathIsDirectory {
                path: arguments.filepath
            })
        }

        let mut file = fs::File::create(&arguments.filepath)?;

        // 写入头部数据
        file.write_all(&HEADER_DATA)?;

        // 写入版本数据
        if !VERSIONS.contains(&arguments.version.into()) {
            anyhow::bail!(FileError::UnsupportedVersion {
                version: arguments.version
            });
        }

        file.write_all(&[arguments.version.major, arguments.version.minor])?;

        // 写入项目数据
        let mut writer = Writer::new(file).with_version(arguments.version);

        writer.write_object(project)?;

        Ok(())
    }
}
