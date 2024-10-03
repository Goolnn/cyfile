mod credits;

use crate::error::{FileError, FileResult};
use crate::{Codec, Decode, Encode};
pub use credits::Credits;

/// Be used to enumerate different staff positions. It will only be used in the versions 0.2 or
/// greater.
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum Credit {
    /// Someone who specialize in converting written or spoken text from one language to another.
    Translators,
    /// Someone who are responsible for reviewing written text to identify and correct errors in
    /// spelling, grammar, punctuation, and formatting.
    Proofreaders,
    /// Someone who specialize in the post-production process of enhancing or altering digital images.
    Retouchers,
    /// Someone who are responsible for arranging and formatting text in a visually appealing manner
    /// for publication.
    Typesetters,
    /// Someone who oversee and manage the work of others within an organization or a specific area.
    Supervisors,
}

impl TryFrom<u8> for Credit {
    type Error = FileError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Translators),
            1 => Ok(Self::Proofreaders),
            2 => Ok(Self::Retouchers),
            3 => Ok(Self::Typesetters),
            4 => Ok(Self::Supervisors),

            _ => Err(Self::Error::InvalidStructure),
        }
    }
}

impl Encode for Credit {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        codec.write_primitive(*self as u8)
    }
}

impl Decode for Credit {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        Self::try_from(codec.read_primitive::<u8>()?)
    }
}
