#[cfg(test)]
mod debug {
  use cyfile::Text;

  #[test]
  fn empty() {
    let text = Text::new();

    assert_eq!(format!("{:?}", text), "");
  }

  #[test]
  fn with_content() {
    let text = Text::with_content("content of the text");

    assert_eq!(format!("{:?}", text), "Content:\n  content of the text");
  }

  #[test]
  fn with_comment() {
    let text = Text::with_comment("comment of the text");

    assert_eq!(format!("{:?}", text), "Comment:\n  comment of the text");
  }

  #[test]
  fn with_content_and_comment() {
    let text = Text::with_content_and_comment("content of the text", "comment of the text");

    assert_eq!(format!("{:?}", text), "Content:\n  content of the text\n\nComment:\n  comment of the text");
  }
}