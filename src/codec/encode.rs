use super::Length;
use super::Primitive;
use super::Version;
use crate::error::FileError;
use crate::error::FileResult;
use crate::file::constants::VERSION_LATEST;
use std::io::Write;

pub trait Encode {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> FileResult<()>;
}

pub struct Writer<S: Write> {
    stream: S,

    version: Version,
}

impl<S: Write> Writer<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            version: VERSION_LATEST,
        }
    }

    pub fn with_version(mut self, version: impl Into<Version>) -> Self {
        self.version = version.into();

        self
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn write_object<T: Encode>(&mut self, object: &T) -> FileResult<()> {
        object.encode(self)
    }

    pub fn write_primitive<T: Primitive>(&mut self, data: T) -> FileResult<()> {
        let ptr = &data as *const T as *const u8;
        let len = std::mem::size_of::<T>();

        let buffer = unsafe { std::slice::from_raw_parts(ptr, len) };

        if self.stream.write(buffer)? != buffer.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_bytes(&mut self, data: impl AsRef<[u8]>) -> FileResult<()> {
        if self.stream.write(data.as_ref())? != data.as_ref().len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_bytes_with_len<T: Length>(&mut self, data: impl AsRef<[u8]>) -> FileResult<()> {
        self.write_len::<T>(data.as_ref().len())?;
        self.write_bytes(data)
    }

    pub fn write_string_with_len<T: Length>(&mut self, data: &str) -> FileResult<()> {
        self.write_len::<T>(data.len())?;
        self.write_bytes(data.as_bytes())
    }

    pub fn write_string_with_nil(&mut self, data: &str) -> FileResult<()> {
        self.write_bytes(data.as_bytes())?;
        self.write_primitive::<u8>(0)
    }

    fn write_len<T: Length>(&mut self, len: usize) -> FileResult<()> {
        if let Ok(len) = len.try_into() {
            self.write_primitive::<T>(len)
        } else {
            Err(FileError::WriteFailed)
        }
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> S {
        self.stream
    }
}
