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
