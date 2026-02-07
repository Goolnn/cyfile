use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manifest {
    version: u8,
}

impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn set_version(&mut self, version: u8) {
        self.version = version;
    }

    pub fn with_version(mut self, version: u8) -> Self {
        self.version = version;

        self
    }
}
