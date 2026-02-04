mod error;

pub use error::Error;
pub use error::Result;

use crate::file::Manifest;
use serde_json::Value;
use std::marker::Sized;

pub trait Codec {
    fn encode(&self, manifest: &Manifest) -> error::Result<Value>;

    fn decode(manifest: &Manifest, value: Value) -> error::Result<Self>
    where
        Self: Sized;
}
