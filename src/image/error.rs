use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported image format")]
    Unsupported,

    #[error("Undefined image error")]
    Undefined,
}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        match value {
            image::ImageError::Unsupported(_) => Self::Unsupported,

            _ => Self::Undefined,
        }
    }
}
