mod data;
mod error;
mod export;
mod version;

pub use crate::file::error::Error;
pub use export::ExportArguments;
pub use version::Version;

pub(crate) use data::HEADER_DATA;
pub(crate) use data::VERSIONS;
pub(crate) use data::VERSION_LATEST;

use crate::codec::Reader;
use crate::codec::Writer;
use crate::Project;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub struct File;

impl File {
    pub fn open<Stream>(mut stream: Stream) -> anyhow::Result<Project>
    where
        Stream: Read + Seek,
    {
        let mut header = [0u8; 15];
        let mut version = [0u8; 2];

        stream.read_exact(&mut header)?;
        stream.read_exact(&mut version)?;

        if header != data::HEADER_DATA {
            anyhow::bail!(Error::InvalidHeader);
        }

        let version = Version::from(version);

        if !data::VERSIONS.contains(&version.into()) {
            anyhow::bail!(Error::UnsupportedVersion { version });
        }

        let mut reader = Reader::new(stream).with_version(version);

        let project = reader.read_object()?;

        Ok(project)
    }

    pub fn export(project: &Project, arguments: ExportArguments) -> anyhow::Result<()> {
        if arguments.filepath.is_dir() {
            anyhow::bail!(Error::PathIsDirectory {
                path: arguments.filepath
            })
        }

        let mut file = fs::File::create(&arguments.filepath)?;

        // 写入头部数据
        file.write_all(&HEADER_DATA)?;

        // 写入版本数据
        if !VERSIONS.contains(&arguments.version.into()) {
            anyhow::bail!(Error::UnsupportedVersion {
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
