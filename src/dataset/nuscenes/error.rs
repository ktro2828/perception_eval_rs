use image::ImageError;
use std::{io::Error as IoError, path::PathBuf};
use thiserror::Error as ThisError;

pub type NuScenesResult<T> = Result<T, NuScenesError>;

#[derive(Debug, ThisError)]
pub enum NuScenesError {
    #[error("internal error, please report bug")]
    InternalBug,
    #[error("corrupted file: {0}")]
    CorruptedFile(PathBuf),
    #[error("corrupted file: {0}")]
    CorruptedDataset(String),
    #[error("I/O error: {0}")]
    IoError(#[from] IoError),
    #[error("image error: {0}")]
    ImageError(#[from] ImageError),
    #[error("parsing error: {0}")]
    ParseError(String),
}
