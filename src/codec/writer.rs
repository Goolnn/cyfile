use crate::Codec;
use crate::codec;
use crate::file::Manifest;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug)]
pub struct Writer<'a> {
    manifest: &'a Manifest,

    value: Value,
}

impl<'a> Writer<'a> {
    pub fn new(manifest: &'a Manifest) -> Self {
        Self {
            manifest,

            value: Value::Null,
        }
    }

    pub fn field<K, V>(&mut self, key: K, value: &V) -> codec::Result<()>
    where
        K: AsRef<str>,
        V: Codec,
    {
        if self.value.is_null() {
            self.value = Value::Object(Map::new());
        }

        let mut writer = self.clone();

        Codec::encode(value, &mut writer)?;

        let value = writer.into_value();

        if let Value::Object(map) = &mut self.value {
            map.insert(key.as_ref().to_string(), value);
        }

        Ok(())
    }

    pub fn value<T>(&mut self, value: T)
    where
        T: Into<Value>,
    {
        self.value = value.into();
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

impl Clone for Writer<'_> {
    fn clone(&self) -> Self {
        Writer {
            manifest: self.manifest,

            value: Value::Null,
        }
    }
}
