use crate::Codec;
use crate::codec;
use crate::codec::AssetSnap;
use crate::codec::EmptySource;
use crate::codec::Reader;
use crate::codec::Writer;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Asset {
    path: String,

    source: Arc<dyn codec::AssetSource>,

    data: RwLock<Option<Vec<u8>>>,

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

            source: Arc::new(EmptySource),

            data: RwLock::new(Some(data)),

            track: Track::Dirty,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn load(&self) -> codec::Result<Vec<u8>> {
        if self
            .data
            .read()
            .map_err(|_| codec::Error::AssetAccessFailed {
                path: self.path.to_string(),
            })?
            .is_none()
        {
            let data = self.source.load(&self.path)?;

            *self
                .data
                .write()
                .map_err(|_| codec::Error::AssetAccessFailed {
                    path: self.path.to_string(),
                })? = Some(data);
        }

        self.data
            .read()
            .map_err(|_| codec::Error::AssetAccessFailed {
                path: self.path.to_string(),
            })?
            .as_ref()
            .ok_or(codec::Error::AssetNotFound {
                path: self.path.to_string(),
            })
            .cloned()
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
                        Track::Clean => AssetSnap::Clean(Arc::clone(&self.source)),
                        Track::Dirty => AssetSnap::Dirty(
                            self.data
                                .read()
                                .map_err(|_| codec::Error::AssetAccessFailed {
                                    path: self.path.to_string(),
                                })?
                                .clone()
                                .ok_or(codec::Error::AssetNotFound {
                                    path: self.path.to_string(),
                                })?,
                        ),
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

                data: RwLock::new(None),

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

#[cfg(test)]
mod tests {
    use crate::Asset;
    use crate::Codec;
    use crate::codec::Writer;
    use crate::file::Manifest;

    #[test]
    fn path() {
        let asset = Asset::new("asset.png", Vec::new());

        assert_eq!(asset.path(), "asset.png");
    }

    #[test]
    fn load() {
        let data = vec![0, 1, 2, 3];

        let asset = Asset::new("asset.png", data.clone());

        match asset.load() {
            Ok(data) => assert_eq!(data, data),

            Err(err) => panic!("Failed to load asset: {:?}", err),
        }
    }

    #[test]
    fn encode() {
        let asset = Asset::new("asset.png", vec![0, 1, 2, 3]);

        let manifest = Manifest::default();

        let mut writer = Writer::new(&manifest);

        assert!(Codec::encode(&asset, &mut writer).is_ok());

        let value = writer.into_value();

        assert_eq!(value, "asset.png");
    }
}
