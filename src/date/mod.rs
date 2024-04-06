use crate::error::{
  FileResult,
  FileError,
};

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

use chrono::{
  Timelike,
  Datelike,
  Local,
};

use std::fmt::{
  Formatter,
  Debug,
};

#[derive(Copy, Clone)]
pub struct Date {
  year: u16,
  month: u8,
  day: u8,

  hour: u8,
  minute: u8,
  second: u8,
}

impl Date {
  pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
    Self {
      year,
      month,
      day,

      hour,
      minute,
      second,
    }
  }

  pub fn now() -> Self {
    Self::default()
  }

  pub fn year(&self) -> u16 {
    self.year
  }

  pub fn month(&self) -> u8 {
    self.month
  }

  pub fn day(&self) -> u8 {
    self.day
  }

  pub fn hour(&self) -> u8 {
    self.hour
  }

  pub fn minute(&self) -> u8 {
    self.minute
  }

  pub fn second(&self) -> u8 {
    self.second
  }
}

impl Default for Date {
  fn default() -> Self {
    // 获取当前系统时间
    let local = Local::now();

    let year = local.year() as u16;
    let month = local.month() as u8;
    let day = local.day() as u8;

    let hour = local.hour() as u8;
    let minute = local.minute() as u8;
    let second = local.second() as u8;

    Self {
      year,
      month,
      day,

      hour,
      minute,
      second,
    }
  }
}

impl Encode for Date {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    match codec.version() {
      (0, 2) => {
        codec.write_primitive(self.year)?;
        codec.write_primitive(self.month)?;
        codec.write_primitive(self.day)?;

        codec.write_primitive(self.hour)?;
        codec.write_primitive(self.minute)?;
        codec.write_primitive(self.second)?;

        Ok(())
      }

      _ => {
        Err(FileError::InvalidVersion)
      }
    }
  }
}

impl Decode for Date {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    match codec.version() {
      (0, 2) => {
        let year = codec.read_primitive::<u16>()?;
        let month = codec.read_primitive::<u8>()?;
        let day = codec.read_primitive::<u8>()?;

        let hour = codec.read_primitive::<u8>()?;
        let minute = codec.read_primitive::<u8>()?;
        let second = codec.read_primitive::<u8>()?;

        Ok(Self {
          year,
          month,
          day,

          hour,
          minute,
          second,
        })
      }

      _ => {
        Err(FileError::InvalidVersion)
      }
    }
  }
}

impl Debug for Date {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}/{} {:2}:{:2}:{:2}",
      self.year,
      self.month,
      self.day,
      self.hour,
      self.minute,
      self.second,
    )
  }
}