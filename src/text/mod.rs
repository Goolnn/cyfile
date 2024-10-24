use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use std::io::Read;
use std::io::Write;

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Text {
    content: String,
    comment: String,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();

        self
    }

    pub fn with_comment(mut self, comment: impl ToString) -> Self {
        self.comment = comment.to_string();

        self
    }

    pub fn set_content(&mut self, content: impl ToString) {
        self.content = content.to_string();
    }

    pub fn set_comment(&mut self, comment: impl ToString) {
        self.comment = comment.to_string();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

impl Encode for Text {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_string_with_len::<u32>(&self.content)?;
        writer.write_string_with_len::<u32>(&self.comment)?;

        Ok(())
    }
}

impl Decode for Text {
    fn decode<S: Read>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        Ok(Self {
            content: reader.read_string_with_len::<u32>()?,
            comment: reader.read_string_with_len::<u32>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Text;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn codec() -> anyhow::Result<()> {
        let text = Text::new().with_content("Content").with_comment("Comment");

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&text)?;

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0))?;

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_object::<Text>()?, text);

        Ok(())
    }
}
