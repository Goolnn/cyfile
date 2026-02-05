use crate::Codec;
use crate::codec;
use crate::codec::AssetSource;
use crate::codec::Reader;
use std::fmt;
use std::fmt::Debug;
use std::io::Read;
use std::io::Seek;
use std::rc::Rc;

pub struct Asset {
    source: Rc<dyn AssetSource>,

    data: Option<Vec<u8>>,

    path: String,
}

impl Asset {
    pub fn load(&mut self) -> codec::Result<&[u8]> {
        if self.data.is_none() {
            self.data = Some(self.source.load(&self.path)?);
        }

        self.data.as_deref().ok_or(codec::Error::AssetNotFound {
            path: self.path.clone(),
        })
    }

    pub fn clean(&mut self) {
        self.data = None;
    }
}

impl Codec for Asset {
    fn encode(&self, _manifest: &crate::file::Manifest) -> codec::Result<serde_json::Value> {
        todo!()
    }

    fn decode<'a, S>(reader: Reader<'a, S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        Ok(Asset {
            source: reader.source(),

            data: None,

            path: Codec::decode(reader)?,
        })
    }
}

impl Debug for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Asset(\"{}\")", self.path))
    }
}
