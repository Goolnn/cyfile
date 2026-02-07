use crate::Codec;
use crate::codec;
use crate::codec::AssetSnap;
use crate::codec::EmptySource;
use crate::codec::Reader;
use crate::codec::Writer;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Asset {
    path: String,

    source: Rc<dyn codec::AssetSource>,

    data: Option<Vec<u8>>,

    track: Track,
}

enum Track {
    Clean,
    Dirty,
}

impl Asset {
    pub fn new<P>(path: P, data: Vec<u8>) -> Self
    where
        P: ToString,
    {
        Asset {
            path: path.to_string(),

            source: Rc::new(EmptySource),

            data: Some(data),

            track: Track::Dirty,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn load(&mut self) -> codec::Result<&[u8]> {
        if self.data.is_none() {
            let data = self.source.load(&self.path)?;

            self.data = Some(data);
        }

        self.data
            .as_ref()
            .ok_or(codec::Error::AssetNotFound {
                path: self.path.to_string(),
            })
            .map(|data| data.as_slice())
    }
}

impl Codec for Asset {
    fn encode(&self, writer: &mut Writer) -> codec::Result<()> {
        match writer.manifest().version() {
            0 => {
                writer.value(self.path.clone());

                writer.asset(
                    self.path.clone(),
                    match self.track {
                        Track::Clean => AssetSnap::Clean(Rc::clone(&self.source)),
                        Track::Dirty => AssetSnap::Dirty(self.data.clone().ok_or(
                            codec::Error::AssetNotFound {
                                path: self.path.to_string(),
                            },
                        )?),
                    },
                );

                Ok(())
            }

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }

    fn decode(reader: &Reader) -> codec::Result<Self> {
        match reader.manifest().version() {
            0 => Ok(Asset {
                path: reader
                    .value()
                    .as_str()
                    .ok_or(codec::Error::MismatchType {
                        expected: "string".to_string(),
                        found: reader.value().to_string(),
                    })?
                    .to_string(),

                source: reader.asset(),

                data: None,

                track: Track::Clean,
            }),

            version => Err(codec::Error::UnsupportedVersion { version }),
        }
    }
}

impl Debug for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Asset(\"{}\")", self.path))
    }
}
