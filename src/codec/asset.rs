use crate::codec;
use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::rc::Rc;
use zip::ZipArchive;
use zip::ZipWriter;

pub trait DynWrite: Write + Seek {}

impl<T> DynWrite for T where T: Write + Seek {}

pub trait AssetSource {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>>;

    fn copy(&self, path: &str, writer: &mut ZipWriter<&mut dyn DynWrite>) -> codec::Result<()>;
}

pub struct ArchiveSource<R>
where
    R: Read + Seek,
{
    archive: Rc<RefCell<ZipArchive<R>>>,
}

impl<R> ArchiveSource<R>
where
    R: Read + Seek,
{
    pub fn new(archive: ZipArchive<R>) -> Self {
        ArchiveSource {
            archive: Rc::new(RefCell::new(archive)),
        }
    }
}

impl<R> AssetSource for ArchiveSource<R>
where
    R: Read + Seek + 'static,
{
    fn load(&self, path: &str) -> codec::Result<Vec<u8>> {
        let mut archive = self.archive.borrow_mut();

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
            return Err(codec::Error::Undefined);
        }

        Ok(data)
    }

    fn copy(&self, path: &str, writer: &mut ZipWriter<&mut dyn DynWrite>) -> codec::Result<()> {
        let mut archive = self.archive.borrow_mut();

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
            .map_err(|_| codec::Error::Undefined)
    }
}

pub struct EmptySource;

impl AssetSource for EmptySource {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>> {
        Err(codec::Error::AssetNotFound {
            path: path.to_string(),
        })
    }

    fn copy(&self, path: &str, _: &mut ZipWriter<&mut dyn DynWrite>) -> codec::Result<()> {
        Err(codec::Error::AssetNotFound {
            path: path.to_string(),
        })
    }
}

#[derive(Clone)]
pub enum AssetSnap {
    Clean(Rc<dyn AssetSource>),
    Dirty(Vec<u8>),
}
