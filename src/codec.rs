mod error;

pub use error::Error;
pub use error::Result;

use crate::codec;
use crate::file::Manifest;
use serde_json::Value;
use std::marker::Sized;

pub trait Codec: Sized {
    fn encode(&self, manifest: &Manifest) -> codec::Result<Value>;

    fn decode(manifest: &Manifest, value: &Value) -> codec::Result<Self>;
}
