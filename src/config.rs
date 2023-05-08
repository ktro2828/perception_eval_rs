use crate::label::{convert_labels, LabelConverter};
use crate::{frame_id::FrameID, label::Label};
use log4rs::filter::Filter;
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
    pub dataset_path: PathBuf,
    pub frame_id: FrameID,
    pub result_dir: PathBuf,
    pub log_dir: PathBuf,
    pub viz_dir: PathBuf,
    pub filter_params: FilterParams,
    pub load_raw_data: bool,
}

impl PerceptionEvaluationConfig {
    pub fn new(
        dataset_path: &str,
        frame_id: &str,
        result_dir: &str,
        filter_params: &FilterParams,
        load_raw_data: &bool,
    ) -> Self {
        let dataset_path = Path::new(dataset_path);
        let frame_id = FrameID::from_str(frame_id).unwrap();
        let result_dir = Path::new(result_dir);
        let log_dir = result_dir.join("log");
        let viz_dir = result_dir.join("visualize");

        Self {
            dataset_path: dataset_path.to_owned(),
            frame_id: frame_id,
            result_dir: result_dir.to_owned(),
            log_dir: log_dir,
            viz_dir: viz_dir,
            filter_params: filter_params.to_owned(),
            load_raw_data: load_raw_data.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) max_x_positions: Vec<f64>,
    pub(crate) max_y_positions: Vec<f64>,
    pub(crate) min_point_numbers: Vec<u64>,
    pub(crate) uuids: Option<Vec<String>>,
}

impl FilterParams {
    pub fn new(
        target_labels: &Vec<&str>,
        max_x_position: f64,
        max_y_position: f64,
        min_point_number: u64,
        uuids: Option<Vec<String>>,
    ) -> Self {
        let label_converter = LabelConverter::new(Some("autoware")).unwrap();
        let target_labels = convert_labels(target_labels, &label_converter).unwrap();
        let num_target_labels = target_labels.len();
        let max_x_positions = vec![max_x_position; num_target_labels];
        let max_y_positions = vec![max_y_position; num_target_labels];
        let min_point_numbers = vec![min_point_number; num_target_labels];

        Self {
            target_labels: target_labels,
            max_x_positions: max_x_positions,
            max_y_positions: max_y_positions,
            min_point_numbers: min_point_numbers,
            uuids: uuids,
        }
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
    ) -> Self {
        let label_converter = LabelConverter::new(Some("autoware")).unwrap();
        let target_labels = convert_labels(target_labels, &label_converter).unwrap();
        let num_target_labels = target_labels.len();
        let center_distance_thresholds = vec![center_distance_threshold; num_target_labels];
        let plane_distance_thresholds = vec![plane_distance_threshold; num_target_labels];
        let iou2d_thresholds = vec![iou2d_threshold; num_target_labels];
        let iou3d_thresholds = vec![iou3d_threshold; num_target_labels];

        Self {
            target_labels: target_labels,
            center_distance_thresholds: center_distance_thresholds,
            plane_distance_thresholds: plane_distance_thresholds,
            iou2d_thresholds: iou2d_thresholds,
            iou3d_thresholds: iou3d_thresholds,
        }
    }
}

pub fn get_evaluation_params(
    target_labels: &Vec<&str>,
    max_x_position: f64,
    max_y_position: f64,
    min_point_number: u64,
    uuids: Option<Vec<String>>,
    center_distance_threshold: f64,
    plane_distance_threshold: f64,
    iou2d_threshold: f64,
    iou3d_threshold: f64,
) -> (FilterParams, MetricsParams) {
    let f_params = FilterParams::new(
        target_labels,
        max_x_position,
        max_y_position,
        min_point_number,
        uuids,
    );

    let m_params = MetricsParams::new(
        target_labels,
        center_distance_threshold,
        plane_distance_threshold,
        iou2d_threshold,
        iou3d_threshold,
    );

    (f_params, m_params)
}
