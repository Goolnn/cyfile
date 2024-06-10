use crate::codec::{Codec, Encode};
use crate::error::FileResult;

#[derive(Default)]
pub struct Image(Vec<u8>);

impl Image {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn inner_mut(&mut self) -> &mut Vec<u8> {
    &mut self.0
  }

  pub fn inner(&self) -> &Vec<u8> {
    &self.0
  }
}

impl From<Vec<u8>> for Image {
  fn from(value: Vec<u8>) -> Self {
    Self(value)
  }
}

impl Encode for Image {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    codec.write_data_with_len::<u32>(self.inner())
  }
}
