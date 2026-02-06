use crate::Codec;
use crate::codec;
use crate::codec::Reader;
use crate::codec::Writer;
use std::fmt;
use std::fmt::Debug;

pub struct Asset {
    path: String,
}

impl Asset {
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Codec for Asset {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        writer.value(self.path.clone());

        Ok(())
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        Ok(Asset {
            path: reader
                .value()
                .as_str()
                .ok_or(codec::Error::MismatchType {
                    expected: "string".to_string(),
                    found: reader.value().to_string(),
                })?
                .to_string(),
        })
    }
}

impl Debug for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Asset(\"{}\")", self.path))
    }
}
