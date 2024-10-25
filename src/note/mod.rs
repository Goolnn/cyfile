use crate::codec::Decode;
use crate::codec::Encode;
use crate::codec::Reader;
use crate::codec::Writer;
use crate::Text;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Note {
    x: f64,
    y: f64,

    choice: u32,

    texts: Vec<Text>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_coordinate(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    pub fn with_choice(mut self, choice: u32) -> Self {
        self.choice = choice;

        self
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.texts.push(text);

        self
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_choice(&mut self, choice: u32) {
        self.choice = choice;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn choice(&self) -> u32 {
        self.choice
    }

    pub fn texts_mut(&mut self) -> &mut Vec<Text> {
        &mut self.texts
    }

    pub fn texts(&self) -> &Vec<Text> {
        &self.texts
    }

    pub(crate) fn merge_texts(&self) -> String {
        self.texts
            .iter()
            .map(|text| {
                let mut result = String::new();

                if text.content().is_empty() && !text.comment().is_empty() {
                    result.push_str(text.comment());
                } else if text.comment().is_empty() && !text.content().is_empty() {
                    result.push_str(text.content());
                } else {
                    result.push_str(format!("{}\n\n{}", text.content(), text.comment()).as_ref());
                }

                result
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Encode for Note {
    fn encode<S: Write>(&self, writer: &mut Writer<S>) -> anyhow::Result<()> {
        writer.write_primitive(self.x)?;
        writer.write_primitive(self.y)?;

        writer.write_primitive(self.choice)?;

        writer.write_primitive(self.texts.len() as u32)?;

        for text in self.texts() {
            writer.write_object(text)?;
        }

        Ok(())
    }
}

impl Decode for Note {
    fn decode<S: Read + Seek>(reader: &mut Reader<S>) -> anyhow::Result<Self> {
        let x = reader.read_primitive()?;
        let y = reader.read_primitive()?;

        let choice = reader.read_primitive()?;

        let len = reader.read_primitive::<u32>()?;

        let mut texts = Vec::new();

        for _ in 0..len {
            texts.push(reader.read_object()?);
        }

        Ok(Self {
            x,
            y,

            choice,

            texts,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::Reader;
    use crate::codec::Writer;
    use crate::Note;
    use crate::Text;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

    #[test]
    fn codec() {
        let note = Note::new()
            .with_coordinate(0.5, 0.5)
            .with_choice(1)
            .with_text(
                Text::new()
                    .with_content("content_1")
                    .with_comment("comment_1"),
            )
            .with_text(
                Text::new()
                    .with_content("content_2")
                    .with_comment("comment_2"),
            )
            .with_text(
                Text::new()
                    .with_content("content_3")
                    .with_comment("comment_3"),
            );

        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);

        let mut writer = Writer::new(cursor);

        writer.write_object(&note).unwrap();

        let mut cursor = writer.into_inner();

        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = Reader::new(cursor);

        assert_eq!(reader.read_object::<Note>().unwrap(), note);
    }
}
