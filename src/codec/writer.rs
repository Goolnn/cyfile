use crate::Codec;
use crate::codec;
use crate::codec::AssetSnap;
use crate::file::Manifest;
use serde_json::Map;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Assets = Rc<RefCell<HashMap<String, AssetSnap>>>;

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

            assets: Rc::new(RefCell::new(HashMap::new())),
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

    pub fn assets(&self) -> HashMap<String, AssetSnap> {
        self.assets.borrow().clone()
    }

    pub fn asset(&mut self, path: String, snap: AssetSnap) {
        let mut assets = self.assets.borrow_mut();

        assets.insert(path, snap);
    }

    pub fn end(self) -> (Assets, Value) {
        (self.assets, self.value)
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

            assets: Rc::clone(&self.assets),
        }
    }
}
