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