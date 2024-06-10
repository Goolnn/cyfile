use crate::error::{
  FileResult,
  FileError,
};

use crate::{
  Encode,
  Decode,
  Codec,
};

use chrono::{
  Timelike,
  Datelike,
  Local,
};

pub type Year = u16;
pub type Month = u8;
pub type Day = u8;

pub type Hour = u8;
pub type Minute = u8;
pub type Second = u8;

#[derive(Copy, Clone)]
pub struct Date {
  year: Year,
  month: Month,
  day: Day,

  hour: Hour,
  minute: Minute,
  second: Second,
}

impl Date {
  pub fn new(year: Year, month: Month, day: Day, hour: Hour, minute: Minute, second: Second) -> Self {
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
    // 获取当前系统时间
    let local = Local::now();

    let year = local.year() as Year;
    let month = local.month() as Month;
    let day = local.day() as Day;

    let hour = local.hour() as Hour;
    let minute = local.minute() as Minute;
    let second = local.second() as Second;

    Self {
      year,
      month,
      day,

      hour,
      minute,
      second,
    }
  }

  pub fn year(&self) -> Year {
    self.year
  }

  pub fn month(&self) -> Month {
    self.month
  }

  pub fn day(&self) -> Day {
    self.day
  }

  pub fn hour(&self) -> Hour {
    self.hour
  }

  pub fn minute(&self) -> Minute {
    self.minute
  }

  pub fn second(&self) -> Second {
    self.second
  }
}

impl Default for Date {
  fn default() -> Self {
    Self::now()
  }
}

impl Encode for Date {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    match codec.version() {
      (0, 1) | (0, 2) => {
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
      (0, 1) | (0, 2) => {
        let year = codec.read_primitive()?;
        let month = codec.read_primitive()?;
        let day = codec.read_primitive()?;

        let hour = codec.read_primitive()?;
        let minute = codec.read_primitive()?;
        let second = codec.read_primitive()?;

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
