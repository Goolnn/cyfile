use crate::Codec;
use crate::codec;
use crate::codec::ArchiveSource;
use crate::codec::AssetSource;
use crate::file::Manifest;
use serde_json::Value;
use std::io::Read;
use std::io::Seek;
use std::rc::Rc;
use zip::ZipArchive;

pub struct Reader<'a> {
    manifest: &'a Manifest,

    value: &'a Value,

    source: Rc<dyn AssetSource>,
}

impl<'a> Reader<'a> {
    pub fn new<R>(manifest: &'a Manifest, value: &'a Value, archive: ZipArchive<R>) -> Reader<'a>
    where
        R: Read + Seek + 'static,
    {
        Reader {
            manifest,

            value,

            source: Rc::new(ArchiveSource::new(archive)),
        }
    }

    pub fn manifest(&self) -> &Manifest {
        self.manifest
    }

    pub fn field<K, T>(&self, key: K) -> codec::Result<T>
    where
        K: AsRef<str>,
        T: Codec,
    {
        let value = self
            .value
            .get(key.as_ref())
            .ok_or(codec::Error::MissingField {
                field: key.as_ref().to_string(),
            })?;

        let reader = self.clone(value);

        Codec::decode(&reader)
    }

    pub fn value(&self) -> &Value {
        self.value
    }

    pub fn asset(&self) -> Rc<dyn AssetSource> {
        Rc::clone(&self.source)
    }

    pub fn clone(&self, value: &'a Value) -> Reader<'a> {
        Reader {
            manifest: self.manifest,

            value,

            source: Rc::clone(&self.source),
        }
    }
}
