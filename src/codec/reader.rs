use crate::Codec;
use crate::codec;
use serde_json::Value;

pub struct Reader<'a> {
    value: &'a Value,
}

impl<'a> Reader<'a> {
    pub fn new(value: &'a Value) -> Reader<'a> {
        Reader { value }
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

        let reader = Reader::new(value);

        Codec::decode(&reader)
    }

    pub fn value(&self) -> &Value {
        self.value
    }
}
