use crate::codec;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use zip::ZipArchive;
use zip::ZipWriter;

pub trait Stream: Write + Seek {}

impl<T> Stream for T where T: Write + Seek {}

pub trait AssetSource: Send + Sync {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>>;

    fn copy(&self, path: &str, writer: &mut ZipWriter<&mut dyn Stream>) -> codec::Result<()>;
}

pub struct ArchiveSource<R>
where
    R: Read + Seek,
{
    archive: Arc<Mutex<ZipArchive<R>>>,
}

impl<R> ArchiveSource<R>
where
    R: Read + Seek,
{
    pub fn new(archive: ZipArchive<R>) -> Self {
        ArchiveSource {
            archive: Arc::new(Mutex::new(archive)),
        }
    }
}

impl<R> AssetSource for ArchiveSource<R>
where
    R: Read + Seek + Send + 'static,
{
    fn load(&self, path: &str) -> codec::Result<Vec<u8>> {
        let mut archive = self
            .archive
            .lock()
            .map_err(|_| codec::Error::ArchiveAcquireFailed)?;

        let mut stream = match archive.by_name(path) {
            Ok(val) => val,

            Err(_) => {
                return Err(codec::Error::AssetNotFound {
                    path: path.to_string(),
                });
            }
        };

        let mut data = Vec::new();

        if stream.read_to_end(&mut data).is_err() {
            return Err(codec::Error::AssetLoadFailed {
                path: path.to_string(),
            });
        }

        Ok(data)
    }

    fn copy(&self, path: &str, writer: &mut ZipWriter<&mut dyn Stream>) -> codec::Result<()> {
        let mut archive = self
            .archive
            .lock()
            .map_err(|_| codec::Error::ArchiveAcquireFailed)?;

        let stream = match archive.by_name(path) {
            Ok(val) => val,

            Err(_) => {
                return Err(codec::Error::AssetNotFound {
                    path: path.to_string(),
                });
            }
        };

        writer
            .raw_copy_file(stream)
            .map_err(|_| codec::Error::AssetCopyFailed {
                path: path.to_string(),
            })
    }
}

pub struct EmptySource;

impl AssetSource for EmptySource {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>> {
        Err(codec::Error::AssetNotFound {
            path: path.to_string(),
        })
    }

    fn copy(&self, path: &str, _: &mut ZipWriter<&mut dyn Stream>) -> codec::Result<()> {
        Err(codec::Error::AssetNotFound {
            path: path.to_string(),
        })
    }
}

#[derive(Clone)]
pub enum AssetSnap {
    Clean(Arc<dyn AssetSource>),
    Dirty(Vec<u8>),
}
