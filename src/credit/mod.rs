use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Credit {
    Artists,
    Translators,
    Proofreaders,
    Retouchers,
    Typesetters,
    Supervisors,
}

impl From<u8> for Credit {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Artists,
            1 => Self::Translators,
            2 => Self::Proofreaders,
            3 => Self::Retouchers,
            4 => Self::Typesetters,
            _ => Self::Supervisors,
        }
    }
}

impl Decode for Credit {
    fn decode<S: Read>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        Ok(reader.read_primitive::<u8>()?.into())
    }
}

impl Encode for Credit {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(*self as u8)
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Writer;
    use crate::Credit;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn codec() {
        let artists = Credit::Artists;
        let translators = Credit::Translators;
        let proofreaders = Credit::Proofreaders;
        let retouchers = Credit::Retouchers;
        let typesetters = Credit::Typesetters;
        let supervisors = Credit::Supervisors;

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&artists).unwrap();
        writer.write_object(&translators).unwrap();
        writer.write_object(&proofreaders).unwrap();
        writer.write_object(&retouchers).unwrap();
        writer.write_object(&typesetters).unwrap();
        writer.write_object(&supervisors).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = crate::codec::Reader::new(cursor);

        assert_eq!(reader.read_object::<Credit>().unwrap(), artists);
        assert_eq!(reader.read_object::<Credit>().unwrap(), translators);
        assert_eq!(reader.read_object::<Credit>().unwrap(), proofreaders);
        assert_eq!(reader.read_object::<Credit>().unwrap(), retouchers);
        assert_eq!(reader.read_object::<Credit>().unwrap(), typesetters);
        assert_eq!(reader.read_object::<Credit>().unwrap(), supervisors);
    }
}
