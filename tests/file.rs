#[cfg(test)]
mod error {
  use cyfile::error::FileError;

  use cyfile::File;

  #[test]
  fn path_not_exists() {
    let file = File::open("tests/file.cy");

    if let Err(err) = file {
      assert_eq!(err, FileError::PathNotExists);
    }
  }

  #[test]
  fn path_not_file() {
    let file = File::open("tests/");

    if let Err(err) = file {
      assert_eq!(err, FileError::PathNotFile);
    }
  }
}