use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub type Year = u16;
pub type Month = u8;
pub type Day = u8;

pub type Hour = u8;
pub type Minute = u8;
pub type Second = u8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl Codec for Date {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> codec::Result<Self> {
        Ok(Self {
            year: reader.read_primitive()?,
            month: reader.read_primitive()?,
            day: reader.read_primitive()?,

            hour: reader.read_primitive()?,
            minute: reader.read_primitive()?,
            second: reader.read_primitive()?,
        })
    }

    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> codec::Result<()> {
        writer.write_primitive(self.year)?;
        writer.write_primitive(self.month)?;
        writer.write_primitive(self.day)?;

        writer.write_primitive(self.hour)?;
        writer.write_primitive(self.minute)?;
        writer.write_primitive(self.second)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Date;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn codec() -> anyhow::Result<()> {
        let date = Date::now();

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&date)?;

        writer.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(writer.into_inner());

        assert_eq!(reader.read_object::<Date>()?, date);

        Ok(())
    }
}
