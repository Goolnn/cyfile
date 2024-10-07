use crate::{
    codec::{Decode, Encode, Reader, Writer},
    error::{FileError, FileResult},
};
use chrono::{Datelike, Local, Timelike};
use std::io::Write;

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
    pub fn new(
        year: Year,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
    ) -> Self {
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
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> FileResult<()> {
        match writer.version() {
            (0, 1) | (0, 2) => {
                writer.write_primitive(self.year)?;
                writer.write_primitive(self.month)?;
                writer.write_primitive(self.day)?;

                writer.write_primitive(self.hour)?;
                writer.write_primitive(self.minute)?;
                writer.write_primitive(self.second)?;

                Ok(())
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}

impl Decode for Date {
    fn decode<S: std::io::Read>(reader: &mut Reader<S>) -> FileResult<Self> {
        match reader.version() {
            (0, 1) | (0, 2) => {
                let year = reader.read_primitive()?;
                let month = reader.read_primitive()?;
                let day = reader.read_primitive()?;

                let hour = reader.read_primitive()?;
                let minute = reader.read_primitive()?;
                let second = reader.read_primitive()?;

                Ok(Self {
                    year,
                    month,
                    day,

                    hour,
                    minute,
                    second,
                })
            }

            _ => Err(FileError::InvalidVersion),
        }
    }
}
