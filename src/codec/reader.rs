use crate::Codec;
use crate::codec;
use crate::codec::ArchiveSource;
use crate::codec::AssetSource;
use crate::file::Manifest;
use serde_json::Value;
use std::io::Read;
use std::io::Seek;
use std::sync::Arc;
use zip::ZipArchive;

pub struct Reader<'a> {
    manifest: &'a Manifest,

    value: &'a Value,

    source: Arc<dyn AssetSource>,
}

impl<'a> Reader<'a> {
    pub fn new<R>(manifest: &'a Manifest, value: &'a Value, archive: ZipArchive<R>) -> Reader<'a>
    where
        R: Read + Seek + Send + 'static,
    {
        Reader {
            manifest,

            value,

            source: Arc::new(ArchiveSource::new(archive)),
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

    pub fn asset(&self) -> Arc<dyn AssetSource> {
        Arc::clone(&self.source)
    }

    pub fn clone(&self, value: &'a Value) -> Reader<'a> {
        Reader {
            manifest: self.manifest,

            value,

            source: Arc::clone(&self.source),
        }
    }
}
