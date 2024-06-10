use crate::error::FileResult;

pub trait Export {
  fn export_to_with_version(&mut self, filepath: &str, version: (u8, u8)) -> FileResult<()>;
  fn export_with_version(&mut self, version: (u8, u8)) -> FileResult<()>;
  fn export_to(&mut self, filepath: &str) -> FileResult<()>;
  fn export(&mut self) -> FileResult<()>;
}
