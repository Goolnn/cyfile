use crate::codec::Length;
use crate::codec::Primitive;
use crate::file::VERSION_LATEST;
use crate::Version;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;

pub trait Decode: Sized {
    fn decode<S>(reader: &mut Reader<S>) -> anyhow::Result<Self>
    where
        S: Read + Seek;
}

pub struct Reader<S>
where
    S: Read + Seek,
{
    stream: S,

    version: Version,
}

impl<S> Reader<S>
where
    S: Read + Seek,
{
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

    pub fn read_object<T>(&mut self) -> anyhow::Result<T>
    where
        T: Decode,
    {
        T::decode(self)
    }

    pub fn read_primitive<T>(&mut self) -> anyhow::Result<T>
    where
        T: Primitive,
    {
        let mut buffer = vec![0u8; size_of::<T>()];

        self.stream.read_exact(&mut buffer)?;

        unsafe { Ok(std::ptr::read(buffer.as_ptr() as *const T)) }
    }

    pub fn read_bytes(&mut self, len: usize) -> anyhow::Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];

        self.stream.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn read_bytes_with_len<T>(&mut self) -> anyhow::Result<Vec<u8>>
    where
        T: Length,
    {
        let len = self.read_len::<T>()?;

        self.read_bytes(len)
    }

    pub fn read_string_with_len<T>(&mut self) -> anyhow::Result<String>
    where
        T: Length,
    {
        let len = self.read_len::<T>()?;
        let buffer = self.read_bytes(len)?;

        Ok(String::from_utf8(buffer)?)
    }

    pub fn read_string_with_nil(&mut self) -> anyhow::Result<String> {
        let mut buffer = Vec::new();

        loop {
            let byte = self.read_primitive::<u8>()?;

            if byte == 0x00 {
                break;
            }

            buffer.push(byte);
        }

        Ok(String::from_utf8(buffer)?)
    }

    fn read_len<T>(&mut self) -> anyhow::Result<usize>
    where
        T: Length,
    {
        if let Ok(len) = self.read_primitive::<T>()?.try_into() {
            Ok(len)
        } else {
            anyhow::bail!("failed to convert length while reading");
        }
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> S {
        self.stream
    }
}

impl<S> Seek for Reader<S>
where
    S: Read + Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.stream.seek(pos)
    }
}
