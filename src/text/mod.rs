use crate::error::FileResult;

use crate::file::codec::{
  Encode,
  Decode,
  Codec,
};

#[derive(Default)]
pub struct Text {
  content: String,
  comment: String,
}

impl Text {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_content_and_comment(content: &str, comment: &str) -> Self {
    Self {
      content: content.to_string(),
      comment: comment.to_string(),
    }
  }

  pub fn with_content(content: &str) -> Self {
    Self {
      content: content.to_string(),
      comment: String::new(),
    }
  }

  pub fn with_comment(comment: &str) -> Self {
    Self {
      content: String::new(),
      comment: comment.to_string(),
    }
  }

  pub fn set_content(&mut self, content: &str) {
    self.content = content.to_string();
  }

  pub fn set_comment(&mut self, comment: &str) {
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
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    codec.write_string::<u32>(&self.content)?;
    codec.write_string::<u32>(&self.comment)?;

    Ok(())
  }
}

impl Decode for Text {
  fn decode(&self, codec: &mut Codec) -> FileResult<Self> {
    Ok(Self {
      content: codec.read_string::<u32>()?,
      comment: codec.read_string::<u32>()?,
    })
  }
}