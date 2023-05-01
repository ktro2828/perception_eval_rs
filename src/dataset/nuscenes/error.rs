use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub type NuScenesResult<T> = Result<T, NuScenesError>;

#[derive(Debug, ThisError)]
pub enum NuScenesError {
    #[error("internal error")]
    InternalError,
    #[error("I/O error: {0}")]
    IoError(#[from] IoError),
    #[error("fail to parse: {0}")]
    ParseError(String),
}
