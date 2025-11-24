mod bound;
mod error;
mod reader;
mod writer;

pub use crate::codec::error::Error;
pub use crate::codec::error::Result;
pub use reader::Reader;
pub use writer::Writer;

use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub trait Codec: Sized {
    fn decode<S>(reader: &mut Reader<S>) -> Result<Self>
    where
        S: Read + Seek;

    fn encode<S>(&self, writer: &mut Writer<S>) -> Result<()>
    where
        S: Write + Seek;
}

impl<T> Codec for Option<T>
where
    T: Codec,
{
    fn decode<S>(reader: &mut Reader<S>) -> Result<Self>
    where
        S: Read + Seek,
    {
        if reader.read_primitive()? {
            Ok(Some(T::decode(reader)?))
        } else {
            Ok(None)
        }
    }

    fn encode<S>(&self, writer: &mut Writer<S>) -> Result<()>
    where
        S: Write + Seek,
    {
        writer.write_primitive(self.is_some())?;

        if let Some(value) = self {
            T::encode(value, writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn primitive() -> anyhow::Result<()> {
        let u8_value = 8u8;
        let u16_value = 16u16;
        let u32_value = 32u32;
        let u64_value = 64u64;
        let u128_value = 128u128;

        let u8_min_value = u8::MIN;
        let u16_min_value = u16::MIN;
        let u32_min_value = u32::MIN;
        let u64_min_value = u64::MIN;
        let u128_min_value = u128::MIN;

        let u8_max_value = u8::MAX;
        let u16_max_value = u16::MAX;
        let u32_max_value = u32::MAX;
        let u64_max_value = u64::MAX;
        let u128_max_value = u128::MAX;

        let i8_nagative_value = -8i8;
        let i16_nagative_value = -16i16;
        let i32_nagative_value = -32i32;
        let i64_nagative_value = -64i64;
        let i128_nagative_value = -128i128;

        let i8_min_value = i8::MIN;
        let i16_min_value = i16::MIN;
        let i32_min_value = i32::MIN;
        let i64_min_value = i64::MIN;
        let i128_min_value = i128::MIN;

        let i8_positive_value = 8i8;
        let i16_positive_value = 16i16;
        let i32_positive_value = 32i32;
        let i64_positive_value = 64i64;
        let i128_positive_value = 128i128;

        let i8_max_value = i8::MAX;
        let i16_max_value = i16::MAX;
        let i32_max_value = i32::MAX;
        let i64_max_value = i64::MAX;
        let i128_max_value = i128::MAX;

        let f32_nagative_value = -3.2f32;
        let f64_nagative_value = -6.4f64;

        let f32_min_value = f32::MIN;
        let f64_min_value = f64::MIN;

        let f32_positive_value = 3.2f32;
        let f64_positive_value = 6.4f64;

        let f32_max_value = f32::MAX;
        let f64_max_value = f64::MAX;

        let bool_true_value = true;
        let bool_false_value = false;

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_primitive(u8_value)?;
        writer.write_primitive(u16_value)?;
        writer.write_primitive(u32_value)?;
        writer.write_primitive(u64_value)?;
        writer.write_primitive(u128_value)?;

        writer.write_primitive(u8_min_value)?;
        writer.write_primitive(u16_min_value)?;
        writer.write_primitive(u32_min_value)?;
        writer.write_primitive(u64_min_value)?;
        writer.write_primitive(u128_min_value)?;

        writer.write_primitive(u8_max_value)?;
        writer.write_primitive(u16_max_value)?;
        writer.write_primitive(u32_max_value)?;
        writer.write_primitive(u64_max_value)?;
        writer.write_primitive(u128_max_value)?;

        writer.write_primitive(i8_nagative_value)?;
        writer.write_primitive(i16_nagative_value)?;
        writer.write_primitive(i32_nagative_value)?;
        writer.write_primitive(i64_nagative_value)?;
        writer.write_primitive(i128_nagative_value)?;

        writer.write_primitive(i8_min_value)?;
        writer.write_primitive(i16_min_value)?;
        writer.write_primitive(i32_min_value)?;
        writer.write_primitive(i64_min_value)?;
        writer.write_primitive(i128_min_value)?;

        writer.write_primitive(i8_positive_value)?;
        writer.write_primitive(i16_positive_value)?;
        writer.write_primitive(i32_positive_value)?;
        writer.write_primitive(i64_positive_value)?;
        writer.write_primitive(i128_positive_value)?;

        writer.write_primitive(i8_max_value)?;
        writer.write_primitive(i16_max_value)?;
        writer.write_primitive(i32_max_value)?;
        writer.write_primitive(i64_max_value)?;
        writer.write_primitive(i128_max_value)?;

        writer.write_primitive(f32_nagative_value)?;
        writer.write_primitive(f64_nagative_value)?;

        writer.write_primitive(f32_min_value)?;
        writer.write_primitive(f64_min_value)?;

        writer.write_primitive(f32_positive_value)?;
        writer.write_primitive(f64_positive_value)?;

        writer.write_primitive(f32_max_value)?;
        writer.write_primitive(f64_max_value)?;

        writer.write_primitive(bool_true_value)?;
        writer.write_primitive(bool_false_value)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_primitive::<u8>()?, u8_value);
        assert_eq!(reader.read_primitive::<u16>()?, u16_value);
        assert_eq!(reader.read_primitive::<u32>()?, u32_value);
        assert_eq!(reader.read_primitive::<u64>()?, u64_value);
        assert_eq!(reader.read_primitive::<u128>()?, u128_value);

        assert_eq!(reader.read_primitive::<u8>()?, u8_min_value);
        assert_eq!(reader.read_primitive::<u16>()?, u16_min_value);
        assert_eq!(reader.read_primitive::<u32>()?, u32_min_value);
        assert_eq!(reader.read_primitive::<u64>()?, u64_min_value);
        assert_eq!(reader.read_primitive::<u128>()?, u128_min_value);

        assert_eq!(reader.read_primitive::<u8>()?, u8_max_value);
        assert_eq!(reader.read_primitive::<u16>()?, u16_max_value);
        assert_eq!(reader.read_primitive::<u32>()?, u32_max_value);
        assert_eq!(reader.read_primitive::<u64>()?, u64_max_value);
        assert_eq!(reader.read_primitive::<u128>()?, u128_max_value);

        assert_eq!(reader.read_primitive::<i8>()?, i8_nagative_value);
        assert_eq!(reader.read_primitive::<i16>()?, i16_nagative_value);
        assert_eq!(reader.read_primitive::<i32>()?, i32_nagative_value);
        assert_eq!(reader.read_primitive::<i64>()?, i64_nagative_value);
        assert_eq!(reader.read_primitive::<i128>()?, i128_nagative_value);

        assert_eq!(reader.read_primitive::<i8>()?, i8_min_value);
        assert_eq!(reader.read_primitive::<i16>()?, i16_min_value);
        assert_eq!(reader.read_primitive::<i32>()?, i32_min_value);
        assert_eq!(reader.read_primitive::<i64>()?, i64_min_value);
        assert_eq!(reader.read_primitive::<i128>()?, i128_min_value);

        assert_eq!(reader.read_primitive::<i8>()?, i8_positive_value);
        assert_eq!(reader.read_primitive::<i16>()?, i16_positive_value);
        assert_eq!(reader.read_primitive::<i32>()?, i32_positive_value);
        assert_eq!(reader.read_primitive::<i64>()?, i64_positive_value);
        assert_eq!(reader.read_primitive::<i128>()?, i128_positive_value);

        assert_eq!(reader.read_primitive::<i8>()?, i8_max_value);
        assert_eq!(reader.read_primitive::<i16>()?, i16_max_value);
        assert_eq!(reader.read_primitive::<i32>()?, i32_max_value);
        assert_eq!(reader.read_primitive::<i64>()?, i64_max_value);
        assert_eq!(reader.read_primitive::<i128>()?, i128_max_value);

        assert_eq!(reader.read_primitive::<f32>()?, f32_nagative_value);
        assert_eq!(reader.read_primitive::<f64>()?, f64_nagative_value);

        assert_eq!(reader.read_primitive::<f32>()?, f32_min_value);
        assert_eq!(reader.read_primitive::<f64>()?, f64_min_value);

        assert_eq!(reader.read_primitive::<f32>()?, f32_positive_value);
        assert_eq!(reader.read_primitive::<f64>()?, f64_positive_value);

        assert_eq!(reader.read_primitive::<f32>()?, f32_max_value);
        assert_eq!(reader.read_primitive::<f64>()?, f64_max_value);

        assert_eq!(reader.read_primitive::<bool>()?, bool_true_value);
        assert_eq!(reader.read_primitive::<bool>()?, bool_false_value);

        Ok(())
    }

    #[test]
    fn bytes() -> anyhow::Result<()> {
        let data = [0x01, 0x04, 0x03, 0x02, 0x05];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_bytes(data)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_bytes(data.len())?, data);

        Ok(())
    }

    #[test]
    fn bytes_with_len() -> anyhow::Result<()> {
        let data = [0x01, 0x04, 0x03, 0x02, 0x05];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_bytes_with_len::<u8>(data)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_bytes_with_len::<u8>()?, data);

        Ok(())
    }

    #[test]
    fn string_with_len() -> anyhow::Result<()> {
        let data = "Hello, world!";

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_string_with_len::<u8>(data)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_string_with_len::<u8>()?, data);

        Ok(())
    }

    #[test]
    fn string_with_nil() -> anyhow::Result<()> {
        let data = "Hello, world!";

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_string_with_nil(data)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_string_with_nil()?, data);

        Ok(())
    }
}
