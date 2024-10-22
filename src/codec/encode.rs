use crate::codec::Length;
use crate::codec::Primitive;
use crate::file::VERSION_LATEST;
use crate::Version;
use std::io::Write;

pub trait Encode {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()>;
}

pub struct Writer<S: Write> {
    stream: S,

    version: Version,
}

impl<S: Write> Writer<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            version: VERSION_LATEST.into(),
        }
    }

    pub fn with_version(mut self, version: impl Into<Version>) -> Self {
        self.version = version.into();

        self
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn write_object<T: Encode>(&mut self, object: &T) -> anyhow::Result<()> {
        object.encode(self)
    }

    pub fn write_primitive<T: Primitive>(&mut self, data: T) -> anyhow::Result<()> {
        let ptr = &data as *const T as *const u8;
        let len = std::mem::size_of::<T>();

        let buffer = unsafe { std::slice::from_raw_parts(ptr, len) };

        self.stream.write_all(buffer)?;

        Ok(())
    }

    pub fn write_bytes(&mut self, data: impl AsRef<[u8]>) -> anyhow::Result<()> {
        self.stream.write_all(data.as_ref())?;

        Ok(())
    }

    pub fn write_bytes_with_len<T: Length>(
        &mut self,
        data: impl AsRef<[u8]>,
    ) -> anyhow::Result<()> {
        self.write_len::<T>(data.as_ref().len())?;
        self.write_bytes(data)
    }

    pub fn write_string_with_len<T: Length>(&mut self, data: &str) -> anyhow::Result<()> {
        self.write_len::<T>(data.len())?;
        self.write_bytes(data.as_bytes())
    }

    pub fn write_string_with_nil(&mut self, data: &str) -> anyhow::Result<()> {
        self.write_bytes(data.as_bytes())?;
        self.write_primitive::<u8>(0)
    }

    fn write_len<T: Length>(&mut self, len: usize) -> anyhow::Result<()> {
        if let Ok(len) = len.try_into() {
            self.write_primitive::<T>(len)
        } else {
            anyhow::bail!("failed to convert length while writing")
        }
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> S {
        self.stream
    }
}
