use crate::Codec;
use crate::codec;
use crate::codec::AssetSnap;
use crate::file::Manifest;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

type Assets = Arc<Mutex<HashMap<String, AssetSnap>>>;

pub struct Writer<'a> {
    manifest: &'a Manifest,

    value: Value,

    assets: Assets,
}

impl<'a> Writer<'a> {
    pub fn new(manifest: &'a Manifest) -> Self {
        Self {
            manifest,

            value: Value::Null,

            assets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn manifest(&self) -> &Manifest {
        self.manifest
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

        let (_, value) = writer.end();

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

    pub fn asset(&mut self, path: String, snap: AssetSnap) {
        if let Ok(mut assets) = self.assets.lock() {
            assets.insert(path, snap);
        }
    }

    pub fn end(self) -> (Assets, Value) {
        (self.assets, self.value)
    }
}

impl Clone for Writer<'_> {
    fn clone(&self) -> Self {
        Writer {
            manifest: self.manifest,

            value: Value::Null,

            assets: Arc::clone(&self.assets),
        }
    }
}
