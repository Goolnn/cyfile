use crate::codec;
use std::any::Any;
use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::rc::Rc;
use zip::ZipArchive;
use zip::ZipWriter;

pub trait AssetSource: Any {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>>;

    fn as_any(&self) -> &dyn Any;
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

    pub fn copy<P, W>(&self, path: P, writer: &mut ZipWriter<W>) -> codec::Result<()>
    where
        P: AsRef<str>,
        W: Write + Seek,
    {
        let mut archive = self.archive.borrow_mut();

        let path = path.as_ref();

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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
