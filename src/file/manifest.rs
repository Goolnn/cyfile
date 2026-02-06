use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub version: u8,
}
