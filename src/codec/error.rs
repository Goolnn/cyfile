use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("required field `{field}` is missing")]
    MissingField { field: String },

    #[error("expect {expected}, but found `{found}`")]
    MismatchType { expected: String, found: String },

    #[error("unsupported version: {version}")]
    UnsupportedVersion { version: u8 },
}
