mod decode;
mod encode;

pub use decode::Decode;
pub use decode::Reader;
pub use encode::Encode;
pub use encode::Writer;

pub trait Primitive {}

pub trait Length: Primitive + TryInto<usize> + TryFrom<usize> {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for f32 {}
impl Primitive for f64 {}
impl Primitive for bool {}

impl Length for u8 {}
impl Length for u16 {}
impl Length for u32 {}
impl Length for u64 {}
impl Length for u128 {}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn primitive() {
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

        writer.write_primitive(u8_value).unwrap();
        writer.write_primitive(u16_value).unwrap();
        writer.write_primitive(u32_value).unwrap();
        writer.write_primitive(u64_value).unwrap();
        writer.write_primitive(u128_value).unwrap();

        writer.write_primitive(u8_min_value).unwrap();
        writer.write_primitive(u16_min_value).unwrap();
        writer.write_primitive(u32_min_value).unwrap();
        writer.write_primitive(u64_min_value).unwrap();
        writer.write_primitive(u128_min_value).unwrap();

        writer.write_primitive(u8_max_value).unwrap();
        writer.write_primitive(u16_max_value).unwrap();
        writer.write_primitive(u32_max_value).unwrap();
        writer.write_primitive(u64_max_value).unwrap();
        writer.write_primitive(u128_max_value).unwrap();

        writer.write_primitive(i8_nagative_value).unwrap();
        writer.write_primitive(i16_nagative_value).unwrap();
        writer.write_primitive(i32_nagative_value).unwrap();
        writer.write_primitive(i64_nagative_value).unwrap();
        writer.write_primitive(i128_nagative_value).unwrap();

        writer.write_primitive(i8_min_value).unwrap();
        writer.write_primitive(i16_min_value).unwrap();
        writer.write_primitive(i32_min_value).unwrap();
        writer.write_primitive(i64_min_value).unwrap();
        writer.write_primitive(i128_min_value).unwrap();

        writer.write_primitive(i8_positive_value).unwrap();
        writer.write_primitive(i16_positive_value).unwrap();
        writer.write_primitive(i32_positive_value).unwrap();
        writer.write_primitive(i64_positive_value).unwrap();
        writer.write_primitive(i128_positive_value).unwrap();

        writer.write_primitive(i8_max_value).unwrap();
        writer.write_primitive(i16_max_value).unwrap();
        writer.write_primitive(i32_max_value).unwrap();
        writer.write_primitive(i64_max_value).unwrap();
        writer.write_primitive(i128_max_value).unwrap();

        writer.write_primitive(f32_nagative_value).unwrap();
        writer.write_primitive(f64_nagative_value).unwrap();

        writer.write_primitive(f32_min_value).unwrap();
        writer.write_primitive(f64_min_value).unwrap();

        writer.write_primitive(f32_positive_value).unwrap();
        writer.write_primitive(f64_positive_value).unwrap();

        writer.write_primitive(f32_max_value).unwrap();
        writer.write_primitive(f64_max_value).unwrap();

        writer.write_primitive(bool_true_value).unwrap();
        writer.write_primitive(bool_false_value).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_primitive::<u8>().unwrap(), u8_value);
        assert_eq!(reader.read_primitive::<u16>().unwrap(), u16_value);
        assert_eq!(reader.read_primitive::<u32>().unwrap(), u32_value);
        assert_eq!(reader.read_primitive::<u64>().unwrap(), u64_value);
        assert_eq!(reader.read_primitive::<u128>().unwrap(), u128_value);

        assert_eq!(reader.read_primitive::<u8>().unwrap(), u8_min_value);
        assert_eq!(reader.read_primitive::<u16>().unwrap(), u16_min_value);
        assert_eq!(reader.read_primitive::<u32>().unwrap(), u32_min_value);
        assert_eq!(reader.read_primitive::<u64>().unwrap(), u64_min_value);
        assert_eq!(reader.read_primitive::<u128>().unwrap(), u128_min_value);

        assert_eq!(reader.read_primitive::<u8>().unwrap(), u8_max_value);
        assert_eq!(reader.read_primitive::<u16>().unwrap(), u16_max_value);
        assert_eq!(reader.read_primitive::<u32>().unwrap(), u32_max_value);
        assert_eq!(reader.read_primitive::<u64>().unwrap(), u64_max_value);
        assert_eq!(reader.read_primitive::<u128>().unwrap(), u128_max_value);

        assert_eq!(reader.read_primitive::<i8>().unwrap(), i8_nagative_value);
        assert_eq!(reader.read_primitive::<i16>().unwrap(), i16_nagative_value);
        assert_eq!(reader.read_primitive::<i32>().unwrap(), i32_nagative_value);
        assert_eq!(reader.read_primitive::<i64>().unwrap(), i64_nagative_value);
        assert_eq!(
            reader.read_primitive::<i128>().unwrap(),
            i128_nagative_value
        );

        assert_eq!(reader.read_primitive::<i8>().unwrap(), i8_min_value);
        assert_eq!(reader.read_primitive::<i16>().unwrap(), i16_min_value);
        assert_eq!(reader.read_primitive::<i32>().unwrap(), i32_min_value);
        assert_eq!(reader.read_primitive::<i64>().unwrap(), i64_min_value);
        assert_eq!(reader.read_primitive::<i128>().unwrap(), i128_min_value);

        assert_eq!(reader.read_primitive::<i8>().unwrap(), i8_positive_value);
        assert_eq!(reader.read_primitive::<i16>().unwrap(), i16_positive_value);
        assert_eq!(reader.read_primitive::<i32>().unwrap(), i32_positive_value);
        assert_eq!(reader.read_primitive::<i64>().unwrap(), i64_positive_value);
        assert_eq!(
            reader.read_primitive::<i128>().unwrap(),
            i128_positive_value
        );

        assert_eq!(reader.read_primitive::<i8>().unwrap(), i8_max_value);
        assert_eq!(reader.read_primitive::<i16>().unwrap(), i16_max_value);
        assert_eq!(reader.read_primitive::<i32>().unwrap(), i32_max_value);
        assert_eq!(reader.read_primitive::<i64>().unwrap(), i64_max_value);
        assert_eq!(reader.read_primitive::<i128>().unwrap(), i128_max_value);

        assert_eq!(reader.read_primitive::<f32>().unwrap(), f32_nagative_value);
        assert_eq!(reader.read_primitive::<f64>().unwrap(), f64_nagative_value);

        assert_eq!(reader.read_primitive::<f32>().unwrap(), f32_min_value);
        assert_eq!(reader.read_primitive::<f64>().unwrap(), f64_min_value);

        assert_eq!(reader.read_primitive::<f32>().unwrap(), f32_positive_value);
        assert_eq!(reader.read_primitive::<f64>().unwrap(), f64_positive_value);

        assert_eq!(reader.read_primitive::<f32>().unwrap(), f32_max_value);
        assert_eq!(reader.read_primitive::<f64>().unwrap(), f64_max_value);

        assert_eq!(reader.read_primitive::<bool>().unwrap(), bool_true_value);
        assert_eq!(reader.read_primitive::<bool>().unwrap(), bool_false_value);
    }

    #[test]
    fn bytes() {
        let data = [0x01, 0x04, 0x03, 0x02, 0x05];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_bytes(data).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_bytes(data.len()).unwrap(), data);
    }

    #[test]
    fn bytes_with_len() {
        let data = [0x01, 0x04, 0x03, 0x02, 0x05];

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_bytes_with_len::<u8>(data).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_bytes_with_len::<u8>().unwrap(), data);
    }

    #[test]
    fn string_with_len() {
        let data = "Hello, world!";

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_string_with_len::<u8>(data).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_string_with_len::<u8>().unwrap(), data);
    }

    #[test]
    fn string_with_nil() {
        let data = "Hello, world!";

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_string_with_nil(data).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_string_with_nil().unwrap(), data);
    }
}
