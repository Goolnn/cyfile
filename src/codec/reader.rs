use crate::Codec;
use crate::codec;
use crate::codec::AssetSource;
use crate::codec::asset::ArchiveSource;
use crate::file::Manifest;
use serde_json::Value;
use std::borrow::Cow;
use std::io::Read;
use std::io::Seek;
use std::marker::PhantomData;
use std::rc::Rc;
use zip::ZipArchive;

pub struct Reader<'a, R>
where
    R: Read + Seek,
{
    source: Rc<dyn AssetSource>,

    manifest: Rc<Manifest>,

    value: Cow<'a, Value>,

    marker: PhantomData<R>,
}

impl<'a, R> Reader<'a, R>
where
    R: Read + Seek + 'static,
{
    pub fn new(archive: ZipArchive<R>, manifest: Manifest, value: Value) -> Self {
        Self {
            source: Rc::new(ArchiveSource::new(archive)),

            manifest: Rc::new(manifest),

            value: Cow::Owned(value),

            marker: PhantomData,
        }
    }
}

impl<'a, R> Reader<'a, R>
where
    R: Read + Seek,
{
    pub fn source(&self) -> Rc<dyn AssetSource> {
        Rc::clone(&self.source)
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn current(&self) -> &Value {
        self.value.as_ref()
    }

    pub fn field<'b, K>(&'b self, key: K) -> codec::Result<Reader<'b, R>>
    where
        K: AsRef<str>,
    {
        let field = self
            .value
            .as_ref()
            .get(key.as_ref())
            .ok_or(codec::Error::MissingField {
                field: key.as_ref().to_string(),
            })?;

        Ok(Reader {
            manifest: Rc::clone(&self.manifest),

            value: Cow::Borrowed(field),

            source: Rc::clone(&self.source),

            marker: PhantomData,
        })
    }

    pub fn at<'b>(&'b self, value: &'b Value) -> Reader<'b, R> {
        Reader {
            manifest: Rc::clone(&self.manifest),

            value: Cow::Borrowed(value),

            source: Rc::clone(&self.source),

            marker: PhantomData,
        }
    }

    pub fn read<K, T>(&self, key: K) -> codec::Result<T>
    where
        K: AsRef<str>,
        T: Codec,
    {
        T::decode(self.field(key)?)
    }
}
