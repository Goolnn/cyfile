use super::Length;
use super::Primitive;
use super::Version;
use crate::error::FileError;
use crate::error::FileResult;
use crate::export::constants::VERSION_LATEST;
use std::io::Read;

pub trait Decode: Sized {
    fn decode<S: Read>(reader: &mut Reader<S>) -> FileResult<Self>;
}

pub struct Reader<S: Read> {
    stream: S,

    version: Version,
}

impl<S: Read> Reader<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            version: VERSION_LATEST,
        }
    }

    pub fn with_version(stream: S, version: impl Into<Version>) -> Self {
        Self {
            stream,
            version: version.into(),
        }
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn read_object<T: Decode>(&mut self) -> FileResult<T> {
        T::decode(self)
    }

    pub fn read_primitive<T: Primitive>(&mut self) -> FileResult<T> {
        let mut buffer = vec![0u8; size_of::<T>()];

        if self.stream.read(&mut buffer)? != buffer.len() {
            return Err(FileError::ReadFailed);
        }

        unsafe { Ok(std::ptr::read(buffer.as_ptr() as *const T)) }
    }

    pub fn read_bytes(&mut self, len: usize) -> FileResult<Vec<u8>> {
        let mut buffer = vec![0u8; len];

        if self.stream.read(&mut buffer)? != buffer.len() {
            Err(FileError::Undefined)
        } else {
            Ok(buffer)
        }
    }

    pub fn read_bytes_with_len<T: Length>(&mut self) -> FileResult<Vec<u8>> {
        let len = self.read_len::<T>()?;

        self.read_bytes(len)
    }

    pub fn read_string_with_len<T: Length>(&mut self) -> FileResult<String> {
        let len = self.read_len::<T>()?;
        let buffer = self.read_bytes(len)?;

        if let Ok(string) = String::from_utf8(buffer) {
            Ok(string)
        } else {
            Err(FileError::ReadFailed)
        }
    }

    pub fn read_string_with_nil(&mut self) -> FileResult<String> {
        let mut buffer = Vec::new();

        loop {
            let byte = self.read_primitive::<u8>()?;

            if byte == 0x00 {
                break;
            }

            buffer.push(byte);
        }

        if let Ok(string) = String::from_utf8(buffer) {
            Ok(string)
        } else {
            Err(FileError::ReadFailed)
        }
    }

    fn read_len<T: Length>(&mut self) -> FileResult<usize> {
        if let Ok(len) = self.read_primitive::<T>()?.try_into() {
            Ok(len)
        } else {
            Err(FileError::ReadFailed)
        }
    }

    pub fn into_inner(self) -> S {
        self.stream
    }
}
