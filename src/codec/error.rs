use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("required field `{field}` is missing")]
    FieldMissing { field: String },

    #[error("expect {expected}, but found `{found}`")]
    TypeMismatch { expected: String, found: String },
}
