use crate::codec;
use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::rc::Rc;
use zip::ZipArchive;

pub trait AssetSource {
    fn load(&self, path: &str) -> codec::Result<Vec<u8>>;
}

pub struct ArchiveSource<S>
where
    S: Read + Seek,
{
    archive: Rc<RefCell<ZipArchive<S>>>,
}

impl<S> ArchiveSource<S>
where
    S: Read + Seek,
{
    pub fn new(archive: ZipArchive<S>) -> Self {
        ArchiveSource {
            archive: Rc::new(RefCell::new(archive)),
        }
    }
}

impl<S> AssetSource for ArchiveSource<S>
where
    S: Read + Seek,
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
}
