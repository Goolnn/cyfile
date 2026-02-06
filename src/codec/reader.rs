use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use std::rc::Rc;

pub struct Reader<'a> {
    manifest: Rc<Manifest>,

    value: &'a Value,
}

impl<'a> Reader<'a> {
    pub fn new(manifest: Manifest, value: &'a Value) -> Reader<'a> {
        Reader {
            manifest: Rc::new(manifest),

            value,
        }
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

    pub fn clone(&self, value: &'a Value) -> Reader<'a> {
        Reader {
            manifest: Rc::clone(&self.manifest),

            value,
        }
    }
}
