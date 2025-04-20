use crate::codec::bound::Length;
use crate::codec::bound::Primitive;
use crate::codec::Codec;
use crate::codec::Error;
use crate::codec::Result;
use crate::file::VERSION_LATEST;
use crate::Version;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

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

    pub fn read_object<T>(&mut self) -> Result<T>
    where
        T: Codec,
    {
        T::decode(self)
    }

    pub fn read_objects<L, T>(&mut self) -> Result<Vec<T>>
    where
        L: Length,
        T: Codec,
    {
        let len = self.read_len::<L>()?;

        let mut objects = Vec::with_capacity(len);

        for _ in 0..len {
            objects.push(self.read_object()?);
        }

        Ok(objects)
    }

    pub fn read_primitive<T>(&mut self) -> Result<T>
    where
        T: Primitive,
    {
        let mut buffer = vec![0u8; size_of::<T>()];

        self.stream.read_exact(&mut buffer)?;

        unsafe { Ok(std::ptr::read(buffer.as_ptr() as *const T)) }
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];

        self.stream.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn read_bytes_with_len<T>(&mut self) -> Result<Vec<u8>>
    where
        T: Length,
    {
        let len = self.read_len::<T>()?;

        self.read_bytes(len)
    }

    pub fn read_string_with_len<T>(&mut self) -> Result<String>
    where
        T: Length,
    {
        let len = self.read_len::<T>()?;
        let buffer = self.read_bytes(len)?;

        Ok(String::from_utf8(buffer)?)
    }

    pub fn read_string_with_nil(&mut self) -> Result<String> {
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

    fn read_len<T>(&mut self) -> Result<usize>
    where
        T: Length,
    {
        if let Ok(len) = self.read_primitive::<T>()?.try_into() {
            Ok(len)
        } else {
            Err(Error::InvalidLength)
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
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.stream.seek(pos)
    }
}
