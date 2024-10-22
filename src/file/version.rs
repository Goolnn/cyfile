use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl From<(u8, u8)> for Version {
    fn from((major, minor): (u8, u8)) -> Self {
        Self { major, minor }
    }
}

impl From<[u8; 2]> for Version {
    fn from(value: [u8; 2]) -> Self {
        Self {
            major: value[0],
            minor: value[1],
        }
    }
}

impl From<Version> for (u8, u8) {
    fn from(value: Version) -> Self {
        (value.major, value.minor)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}
