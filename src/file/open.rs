use crate::Codec;
use crate::Project;
use crate::file;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use zip::ZipArchive;

pub fn open_from_path<P: AsRef<Path>>(path: P) -> file::Result<Project> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(file::Error::PathNotExist {
            path: path.to_path_buf(),
        });
    }

    if !path.is_file() {
        return Err(file::Error::PathNotFile {
            path: path.to_path_buf(),
        });
    }

    let file = match File::open(path) {
        Ok(val) => val,

        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => {
                return Err(file::Error::PermissionDenied {
                    path: path.to_path_buf(),
                });
            }

            _ => {
                return Err(file::Error::Undefined);
            }
        },
    };

    open_from_stream(file)
}

pub fn open_from_stream<R: Read + Seek>(reader: R) -> file::Result<Project> {
    let mut archive = match ZipArchive::new(reader) {
        Ok(val) => val,

        Err(err) => match err {
            zip::result::ZipError::InvalidArchive(_) => {
                return Err(file::Error::InvalidFormat);
            }

            zip::result::ZipError::UnsupportedArchive(_) => {
                return Err(file::Error::UnsupportedFormat);
            }

            zip::result::ZipError::InvalidPassword => {
                return Err(file::Error::PasswordNotCorrect);
            }

            _ => {
                return Err(file::Error::Undefined);
            }
        },
    };

    let manifest = {
        let file = "cangyan.json";

        let stream = match archive.by_name(file) {
            Ok(val) => val,

            Err(_) => {
                return Err(file::Error::FileNotFound {
                    file: file.to_string(),
                });
            }
        };

        match serde_json::from_reader(stream) {
            Ok(val) => val,

            Err(err) => {
                return Err(file::Error::ParseFailure {
                    file: file.to_string(),
                    line: err.line(),
                    column: err.column(),
                });
            }
        }
    };

    let value = {
        let file = "project.json";

        let stream = match archive.by_name(file) {
            Ok(val) => val,

            Err(_) => {
                return Err(file::Error::FileNotFound {
                    file: file.to_string(),
                });
            }
        };

        match serde_json::from_reader(stream) {
            Ok(val) => val,

            Err(err) => {
                return Err(file::Error::ParseFailure {
                    file: file.to_string(),
                    line: err.line(),
                    column: err.column(),
                });
            }
        }
    };

    Ok(Project::decode(&manifest, &value)?)
}
