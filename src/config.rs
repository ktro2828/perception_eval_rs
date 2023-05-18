use crate::evaluation_task::EvaluationTask;
use crate::label::{convert_labels, LabelConverter, LabelResult};
use crate::metrics::config::MetricsConfig;
use crate::{frame_id::FrameID, label::Label};
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::vec;
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
    pub version: String,
    pub dataset_path: PathBuf,
    pub evaluation_task: EvaluationTask,
    pub frame_id: FrameID,
    pub result_dir: PathBuf,
    pub log_dir: PathBuf,
    pub viz_dir: PathBuf,
    pub filter_params: FilterParams,
    pub metrics_params: MetricsParams,
    pub load_raw_data: bool,
}

impl PerceptionEvaluationConfig {
    pub fn new<S>(
        version: &str,
        dataset_path: S,
        evaluation_task: EvaluationTask,
        frame_id: S,
        result_dir: S,
        filter_params: FilterParams,
        metrics_params: MetricsParams,
        load_raw_data: bool,
    ) -> Self
    where
        S: AsRef<str>,
    {
        let dataset_path = Path::new(dataset_path.as_ref());
        let frame_id = FrameID::from_str(frame_id.as_ref()).unwrap();
        let result_dir = Path::new(result_dir.as_ref());
        let log_dir = result_dir.join("log");
        let viz_dir = result_dir.join("visualize");

        Self {
            version: version.to_owned(),
            dataset_path: dataset_path.to_owned(),
            evaluation_task: evaluation_task,
            frame_id: frame_id,
            result_dir: result_dir.to_owned(),
            log_dir: log_dir,
            viz_dir: viz_dir,
            filter_params: filter_params,
            metrics_params: metrics_params,
            load_raw_data: load_raw_data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) max_x_positions: Vec<f64>,
    pub(crate) max_y_positions: Vec<f64>,
    pub(crate) min_point_numbers: Option<Vec<isize>>,
    pub(crate) target_uuids: Option<Vec<String>>,
}

impl FilterParams {
    pub fn new(
        target_labels: &Vec<&str>,
        max_x_position: f64,
        max_y_position: f64,
        min_point_number: Option<isize>,
        target_uuids: Option<Vec<String>>,
    ) -> LabelResult<Self> {
        let label_converter = LabelConverter::new("autoware")?;
        let target_labels = convert_labels(target_labels, &label_converter)?;
        let num_target_labels = target_labels.len();
        let max_x_positions = vec![max_x_position; num_target_labels];
        let max_y_positions = vec![max_y_position; num_target_labels];
        let min_point_numbers = {
            match min_point_number {
                Some(num_pt) => Some(vec![num_pt; num_target_labels]),
                None => None,
            }
        };

        let ret = Self {
            target_labels: target_labels,
            max_x_positions: max_x_positions,
            max_y_positions: max_y_positions,
            min_point_numbers: min_point_numbers,
            target_uuids: target_uuids,
        };
        Ok(ret)
    }
}

#[derive(Debug, Clone)]
pub struct MetricsParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) center_distance_thresholds: Vec<f64>,
    pub(crate) plane_distance_thresholds: Vec<f64>,
    pub(crate) iou2d_thresholds: Vec<f64>,
    pub(crate) iou3d_thresholds: Vec<f64>,
}

impl MetricsParams {
    pub fn new(
        target_labels: &Vec<&str>,
        center_distance_threshold: f64,
        plane_distance_threshold: f64,
        iou2d_threshold: f64,
        iou3d_threshold: f64,
    ) -> LabelResult<Self> {
        let label_converter = LabelConverter::new("autoware")?;
        let target_labels = convert_labels(target_labels, &label_converter)?;
        let num_target_labels = target_labels.len();
        let center_distance_thresholds = vec![center_distance_threshold; num_target_labels];
        let plane_distance_thresholds = vec![plane_distance_threshold; num_target_labels];
        let iou2d_thresholds = vec![iou2d_threshold; num_target_labels];
        let iou3d_thresholds = vec![iou3d_threshold; num_target_labels];

        let ret = Self {
            target_labels: target_labels,
            center_distance_thresholds: center_distance_thresholds,
            plane_distance_thresholds: plane_distance_thresholds,
            iou2d_thresholds: iou2d_thresholds,
            iou3d_thresholds: iou3d_thresholds,
        };
        Ok(ret)
    }

    pub(crate) fn get_metrics_config(&self, evaluation_task: &EvaluationTask) -> MetricsConfig {
        MetricsConfig::new(evaluation_task.to_owned(), self)
    }
}

pub fn get_evaluation_params(
    target_labels: &Vec<&str>,
    max_x_position: f64,
    max_y_position: f64,
    min_point_number: Option<isize>,
    target_uuids: Option<Vec<String>>,
    center_distance_threshold: f64,
    plane_distance_threshold: f64,
    iou2d_threshold: f64,
    iou3d_threshold: f64,
) -> LabelResult<(FilterParams, MetricsParams)> {
    let f_params = FilterParams::new(
        target_labels,
        max_x_position,
        max_y_position,
        min_point_number,
        target_uuids,
    )?;

    let m_params = MetricsParams::new(
        target_labels,
        center_distance_threshold,
        plane_distance_threshold,
        iou2d_threshold,
        iou3d_threshold,
    )?;

    Ok((f_params, m_params))
}
