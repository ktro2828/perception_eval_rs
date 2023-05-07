use crate::{frame_id::FrameID, label::Label};
use std::io::Error as IoError;
use std::path::PathBuf;
use thiserror::Error as ThisError;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("internal error")]
    InternalError,
    #[error("I/O error: {0}")]
    IoError(#[from] IoError),
    #[error("value error: {0}")]
    ValueError(f64),
    #[error("key error: {0}")]
    KeyError(String),
}

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationConfig {
    pub dataset_path: PathBuf,
    pub frame_id: FrameID,
    pub result_dir: PathBuf,
    pub load_raw_data: bool,
    pub filter_params: FilterParams,
}

impl PerceptionEvaluationConfig {
    pub fn log_dir(&self) -> PathBuf {
        self.result_dir.join("log")
    }

    pub fn vis_dir(&self) -> PathBuf {
        self.result_dir.join("visualize")
    }
}

#[derive(Debug, Clone)]
pub struct FilterParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) max_x_position: Vec<f64>,
    pub(crate) max_y_position: Vec<f64>,
    pub(crate) min_point_numbers: Vec<u64>,
    pub(crate) uuids: Vec<String>,
}
