use crate::Codec;
use crate::Project;
use crate::codec;
use crate::codec::AssetSnap;
use crate::codec::Writer;
use crate::file;
use crate::file::Manifest;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use zip::ZipWriter;
use zip::write::FileOptions;

pub fn save_to_path<P>(path: P, manifest: &Manifest, project: &Project) -> file::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if path.exists() && !path.is_file() {
        return Err(file::Error::PathNotFile {
            path: path.to_path_buf(),
        });
    }

    if let Some(parent) = path.parent()
        && let Ok(mut tempfile) = NamedTempFile::new_in(parent)
    {
        save_to_stream(&mut tempfile, manifest, project)?;

        if let Err(err) = tempfile.persist(path) {
            if path.is_file() {
                std::fs::remove_file(path).map_err(|_| codec::Error::Undefined)?;
            }

            err.file
                .persist(path)
                .map_err(|_| codec::Error::Undefined)?;
        }

        Ok(())
    } else {
        Err(file::Error::Undefined)
    }
}

pub fn save_to_stream(
    stream: &mut dyn codec::Stream,
    manifest: &Manifest,
    project: &Project,
) -> file::Result<()> {
    let mut writer = Writer::new(manifest);

    Codec::encode(project, &mut writer)?;

    let (assets, value) = writer.end();

    let manifest =
        serde_json::to_string_pretty(&manifest).map_err(|err| file::Error::ParseFailure {
            file: String::new(),
            line: err.line(),
            column: err.column(),
        })?;

    let project =
        serde_json::to_string_pretty(&value).map_err(|err| file::Error::ParseFailure {
            file: String::new(),
            line: err.line(),
            column: err.column(),
        })?;

    let mut writer = ZipWriter::new(stream);

    writer.set_comment(crate::file::IDENTIFIER.to_owned().into_boxed_str());

    let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);

    writer.start_file("cangyan.json", options)?;
    writer.write_all(manifest.as_bytes())?;

    writer.start_file("project.json", options)?;
    writer.write_all(project.as_bytes())?;

    for (path, snap) in assets.borrow().iter() {
        match snap {
            AssetSnap::Clean(source) => {
                source.copy(path.as_str(), &mut writer)?;
            }

            AssetSnap::Dirty(data) => {
                writer.start_file(path.as_str(), options)?;
                writer.write_all(data)?;
            }
        }
    }

    writer.finish()?;

    Ok(())
}
