use crate::error::{FileError, FileResult};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::size_of;

pub trait Encode {
    fn encode(&self, codec: &mut Codec) -> FileResult<()>;
}

pub trait Decode: Sized {
    fn decode(codec: &mut Codec) -> FileResult<Self>;
}

pub struct Codec {
    stream: File,

    filepath: String,
    version: (u8, u8),
}

impl Codec {
    pub fn new(stream: File, filepath: &str) -> Self {
        Self {
            stream,

            filepath: filepath.to_string(),
            version: (0, 0),
        }
    }

    pub fn with_version(stream: File, filepath: &str, version: (u8, u8)) -> Self {
        Self {
            version,

            ..Self::new(stream, filepath)
        }
    }

    pub fn set_version(&mut self, version: (u8, u8)) {
        self.version = version;
    }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    pub fn version(&self) -> (u8, u8) {
        self.version
    }

    pub fn write_object<O: Encode>(&mut self, object: &O) -> FileResult<()> {
        object.encode(self)?;

        Ok(())
    }

    pub fn write_primitive<T: Copy>(&mut self, data: T) -> FileResult<()> {
        let buffer =
            unsafe { std::slice::from_raw_parts(&data as *const T as *const u8, size_of::<T>()) };

        if self.stream.write(buffer)? != buffer.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_data(&mut self, data: &[u8]) -> FileResult<()> {
        if self.stream.write(data)? != data.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_data_with_len<T: Copy>(&mut self, data: &[u8]) -> FileResult<()>
    where
        usize: TryInto<T>,
    {
        let len = if let Ok(len) = data.len().try_into() {
            len
        } else {
            return Err(FileError::WriteFailed);
        };

        self.write_primitive(len)?;

        if self.stream.write(data)? != data.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_string<T: Copy>(&mut self, data: &str) -> FileResult<()>
    where
        usize: TryInto<T>,
    {
        let data = Vec::from(data.as_bytes());

        let len = if let Ok(len) = data.len().try_into() {
            len
        } else {
            return Err(FileError::WriteFailed);
        };

        self.write_primitive(len)?;

        if self.stream.write(&data)? != data.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn write_string_with_nil<T: Copy>(&mut self, data: &str) -> FileResult<()>
    where
        usize: TryInto<T>,
    {
        let mut data = Vec::from(data.as_bytes());

        data.push(0);

        let len = if let Ok(len) = data.len().try_into() {
            len
        } else {
            return Err(FileError::WriteFailed);
        };

        self.write_primitive(len)?;

        if self.stream.write(&data)? != data.len() {
            Err(FileError::WriteFailed)
        } else {
            Ok(())
        }
    }

    pub fn read_object<O: Decode>(&mut self) -> FileResult<O> {
        O::decode(self)
    }

    pub fn read_primitive<T: Copy>(&mut self) -> FileResult<T> {
        let mut buffer = vec![0u8; size_of::<T>()];

        if self.stream.read(&mut buffer)? != buffer.len() {
            return Err(FileError::ReadFailed);
        }

        unsafe { Ok(std::ptr::read(buffer.as_ptr() as *const T)) }
    }

    pub fn read_data(&mut self, len: usize) -> FileResult<Vec<u8>> {
        let mut buffer = vec![0u8; len];

        if self.stream.read(&mut buffer)? != buffer.len() {
            Err(FileError::Undefined)
        } else {
            Ok(buffer)
        }
    }

    pub fn read_data_with_len<T>(&mut self) -> FileResult<Vec<u8>>
    where
        T: TryInto<usize> + Copy,
    {
        let mut buffer = vec![0u8; self.read_len::<T>()?];

        if self.stream.read(&mut buffer)? != buffer.len() {
            Err(FileError::Undefined)
        } else {
            Ok(buffer)
        }
    }

    pub fn read_string<T>(&mut self) -> FileResult<String>
    where
        T: TryInto<usize> + Copy,
    {
        let mut buffer = vec![0u8; self.read_len::<T>()?];

        if self.stream.read(&mut buffer)? != buffer.len() {
            return Err(FileError::ReadFailed);
        }

        if let Ok(string) = String::from_utf8(buffer) {
            Ok(string)
        } else {
            Err(FileError::ReadFailed)
        }
    }

    pub fn read_string_with_nil<T>(&mut self) -> FileResult<String>
    where
        T: TryInto<usize> + Copy,
    {
        let mut buffer = vec![0u8; self.read_len::<T>()?];

        if self.stream.read(&mut buffer)? != buffer.len() {
            return Err(FileError::ReadFailed);
        }

        buffer.pop();

        if let Ok(string) = String::from_utf8(buffer) {
            Ok(string)
        } else {
            Err(FileError::ReadFailed)
        }
    }

    pub fn read_collection<T, O>(&mut self) -> FileResult<VecDeque<O>>
    where
        T: TryInto<usize> + Copy,
        O: Decode,
    {
        let count = self.read_len::<T>()?;

        let mut pages = VecDeque::with_capacity(count);

        for _ in 0..count {
            pages.push_back(self.read_object()?);
        }

        Ok(pages)
    }

    fn read_len<T>(&mut self) -> FileResult<usize>
    where
        T: TryInto<usize> + Copy,
    {
        if let Ok(len) = self.read_primitive::<T>()?.try_into() {
            Ok(len)
        } else {
            Err(FileError::ReadFailed)
        }
    }
}
