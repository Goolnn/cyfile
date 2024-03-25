use crate::error::{
  FileResult,
  FileError,
};

use std::mem::size_of;

use std::io::{
  Write,
  Read,
};

use std::fs::File;

pub(crate) trait Encode {
  fn encode(&self, codec: &mut Codec) -> FileResult<()>;
}

pub(crate) trait Decode: Sized {
  fn decode(&self, codec: &mut Codec) -> FileResult<Self>;
}

pub struct Codec {
  stream: File,

  version: (u8, u8),
}

impl Codec {
  pub fn new(stream: File, version: (u8, u8)) -> Self {
    Self {
      stream,

      version,
    }
  }

  pub fn into_inner(self) -> File {
    self.stream
  }

  pub fn version(&self) -> (u8, u8) {
    self.version
  }

  pub fn write_primitive<T: Copy>(&mut self, data: T) -> FileResult<()> {
    let buffer = unsafe {
      std::slice::from_raw_parts(&data as *const T as *const u8, size_of::<T>())
    };

    if self.stream.write(buffer)? != buffer.len() {
      Err(FileError::WriteFailed)
    } else {
      Ok(())
    }
  }

  pub fn write_data<T: Copy>(&mut self, data: &[u8]) -> FileResult<()>
    where usize: TryInto<T>
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
    where usize: TryInto<T>
  {
    let mut data = Vec::from(data.as_bytes());

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
    where usize: TryInto<T>
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

  pub fn read_primitive<T: Copy>(&mut self) -> FileResult<T> {
    let mut buffer = vec![0u8; size_of::<T>()];

    if self.stream.read(&mut buffer)? != buffer.len() {
      return Err(FileError::ReadFailed);
    }

    unsafe {
      Ok(std::ptr::read(buffer.as_ptr() as *const T))
    }
  }

  pub fn read_data<T: Copy>(&mut self) -> FileResult<Vec<u8>>
    where T: TryInto<usize>
  {
    let mut buffer = vec![0u8; self.read_len::<T>()?];

    if self.stream.read(&mut buffer)? != buffer.len() {
      Err(FileError::Undefined)
    } else {
      Ok(buffer)
    }
  }

  pub fn read_string<T: Copy>(&mut self) -> FileResult<String>
    where T: TryInto<usize>
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

  pub fn read_string_with_nil<T: Copy>(&mut self) -> FileResult<String>
    where T: TryInto<usize>
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

  fn read_len<T: Copy>(&mut self) -> FileResult<usize>
    where T: TryInto<usize>
  {
    if let Ok(len) = self.read_primitive::<T>()?.try_into() {
      Ok(len)
    } else {
      Err(FileError::ReadFailed)
    }
  }
}