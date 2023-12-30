pub mod schema;

use crate::evaluation_task::EvaluationTask;
use crate::label::{convert_labels, LabelConverter, LabelResult};
use crate::utils::logger::configure_logger;
use crate::{frame_id::FrameID, label::Label};
use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::{
    fs::File,
    io::{BufReader, Error as IoError},
    path::{Path, PathBuf},
    vec,
};
use thiserror::Error as ThisError;

use self::schema::Scenario;

pub type ConfigResult<T> = Result<T, ConfigError>;

/// Represents errors that is associated with `PerceptionEvaluationConfig`.
#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("internal error")]
    InternalError,
    #[error("corrupted file: {0}")]
    CorruptedFile(String),
    #[error("I/O error: {0}")]
    IoError(#[from] IoError),
    #[error("value error: {0}")]
    ValueError(f64),
    #[error("key error: {0}")]
    KeyError(String),
}

/// Configuration of entire evaluation settings.
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
    /// Construct `PerceptionEvaluationConfig` instance.
    ///
    /// * `scenario`        - Scenario path of `.yaml`.
    /// * `result_dir`      - Root directory path to save productions such as log.
    /// * `load_raw_data`   - Indicates whether to load raw data, which is pointcloud or image.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::config::PerceptionEvaluationConfig;
    /// use std::error::Error;
    ///
    /// type Result<T> = std::result::Result<T, Box<dyn Error>>;
    ///
    /// fn main() -> Result<()> {
    ///     let scenario = "tests/config/perception.yaml";
    ///     let result_dir = &format!(
    ///         "./work_dir/{}",
    ///         chrono::Local::now().format("%Y%m%d_%H%M%S")
    ///     );
    ///
    ///     let config = PerceptionEvaluationConfig::from(&scenario, result_dir, false)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from(scenario: &str, result_dir: &str, load_raw_data: bool) -> ConfigResult<Self> {
        let scenario: Scenario = load_yaml(scenario)?;
        let datasets = scenario.evaluation.datasets;

        // TODO
        let mut dataset_path = PathBuf::new();
        let mut version = String::new();
        for (key, value) in &datasets[0] {
            dataset_path.set_file_name(key);
            version = value.version.clone();
        }

        let params = scenario.evaluation.config.params;
        let target_labels = params.target_labels.iter().map(|s| s as &str).collect_vec();
        let filter_params = FilterParams::new(
            &target_labels,
            params.max_x_position,
            params.max_y_position,
            params.min_point_number,
            params.target_uuids,
        )
        .unwrap(); // TODO
        let metrics_params = MetricsParams::new(
            &target_labels,
            params.center_distance_threshold,
            params.plane_distance_threshold,
            params.iou_2d_threshold,
            params.iou_3d_threshold,
        )
        .unwrap(); // TODO

        let result_dir = Path::new(result_dir);
        let log_dir = result_dir.join("log");
        let viz_dir = result_dir.join("visualize");

        configure_logger(&log_dir, log::Level::Debug).unwrap();

        let config = Self {
            version,
            dataset_path,
            evaluation_task: params.evaluation_task,
            frame_id: params.frame_id,
            result_dir: result_dir.to_owned(),
            log_dir,
            viz_dir,
            filter_params,
            metrics_params,
            load_raw_data,
        };
        Ok(config)
    }
}

/// Parameter set to filter out objects.
#[derive(Debug, Clone)]
pub struct FilterParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) max_x_positions: Vec<f64>,
    pub(crate) max_y_positions: Vec<f64>,
    pub(crate) min_point_numbers: Option<Vec<usize>>,
    pub(crate) target_uuids: Option<Vec<String>>,
}

impl FilterParams {
    /// Construct `FilterParams`.
    ///
    /// * `target_labels`       - List of labels should be evaluated.
    /// * `max_x_position`      - Maximum absolute value in the x direction from ego that can be evaluated.
    /// * `max_y_position`      - Maximum absolute value in the y direction from ego that can be evaluated.
    /// * `min_point_number`    - Minimum number of points that GT that can be evaluated should contain.
    /// * `target_uuids`        - List of uuids that GT that can be evaluated should have.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::config::FilterParams;
    ///
    /// let params = FilterParams::new(&vec!["Car", "Pedestrian", "Bus"], 100.0, 100.0, Some(0), None);
    /// ```
    pub fn new(
        target_labels: &Vec<&str>,
        max_x_position: f64,
        max_y_position: f64,
        min_point_number: Option<usize>,
        target_uuids: Option<Vec<String>>,
    ) -> LabelResult<Self> {
        let label_converter = LabelConverter::new("autoware")?;
        let target_labels = convert_labels(target_labels, &label_converter)?;
        let num_target_labels = target_labels.len();
        let max_x_positions = vec![max_x_position; num_target_labels];
        let max_y_positions = vec![max_y_position; num_target_labels];
        let min_point_numbers = min_point_number.map(|num_pt| vec![num_pt; num_target_labels]);

        let ret = Self {
            target_labels,
            max_x_positions,
            max_y_positions,
            min_point_numbers,
            target_uuids,
        };
        Ok(ret)
    }
}

/// Parameter set to calculate metrics score.
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MetricsParams {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) center_distance_thresholds: Vec<f64>,
    pub(crate) plane_distance_thresholds: Vec<f64>,
    pub(crate) iou2d_thresholds: Vec<f64>,
    pub(crate) iou3d_thresholds: Vec<f64>,
}

impl MetricsParams {
    /// Construct `MetricsParams`.
    ///
    /// * `target_labels`               - List of labels should be evaluated.
    /// * `center_distance_threshold`   - Center distance threshold.
    /// * `plane_distance_threshold`    - Plane distance threshold.
    /// * `iou2d_threshold`             - IoU2D threshold.
    /// * `iou3d_threshold`             - IoU3D threshold.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::config::MetricsParams;
    ///
    /// let params = MetricsParams::new(&vec!["Car", "Pedestrian", "Bus"], 1.0, 1.0, 0.5, 0.5);
    /// ```
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
            target_labels,
            center_distance_thresholds,
            plane_distance_thresholds,
            iou2d_thresholds,
            iou3d_thresholds,
        };
        Ok(ret)
    }
}

fn load_yaml<T, P>(path: P) -> ConfigResult<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let reader = BufReader::new(File::open(path.as_ref())?);
    let value = serde_yaml::from_reader(reader).map_err(|err| {
        let msg = format!(
            "failed to load scenario file {}: {:?}",
            path.as_ref().display(),
            err
        );
        ConfigError::CorruptedFile(msg)
    })?;
    Ok(value)
}
