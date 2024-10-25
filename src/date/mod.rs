use crate::codec::Decode;
use crate::codec::Encode;
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

impl Decode for Date {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        Ok(Self {
            year: reader.read_primitive()?,
            month: reader.read_primitive()?,
            day: reader.read_primitive()?,

            hour: reader.read_primitive()?,
            minute: reader.read_primitive()?,
            second: reader.read_primitive()?,
        })
    }
}

impl Encode for Date {
    fn encode<S: Write + Seek>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
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
    fn decode() {
        let year = 2024u16;
        let month = 10u8;
        let day = 8u8;

        let hour = 15u8;
        let minute = 19u8;
        let second = 20u8;

        let mut buffer = Vec::new();

        buffer.extend(year.to_le_bytes());
        buffer.extend(month.to_le_bytes());
        buffer.extend(day.to_le_bytes());

        buffer.extend(hour.to_le_bytes());
        buffer.extend(minute.to_le_bytes());
        buffer.extend(second.to_le_bytes());

        let cursor = Cursor::new(buffer);

        let mut reader = Reader::new(cursor);

        let date = reader.read_object::<Date>().unwrap();

        assert_eq!(date.year(), year);
        assert_eq!(date.month(), month);
        assert_eq!(date.day(), day);

        assert_eq!(date.hour(), hour);
        assert_eq!(date.minute(), minute);
        assert_eq!(date.second(), second);
    }

    #[test]
    fn encode() {
        let date = Date::now();

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&date).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_primitive::<u16>().unwrap(), date.year());
        assert_eq!(reader.read_primitive::<u8>().unwrap(), date.month());
        assert_eq!(reader.read_primitive::<u8>().unwrap(), date.day());

        assert_eq!(reader.read_primitive::<u8>().unwrap(), date.hour());
        assert_eq!(reader.read_primitive::<u8>().unwrap(), date.minute());
        assert_eq!(reader.read_primitive::<u8>().unwrap(), date.second());
    }
}
