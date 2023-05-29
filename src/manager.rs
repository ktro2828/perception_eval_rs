use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::{
    config::PerceptionEvaluationConfig,
    dataset::{get_current_frame, load_dataset, DatasetResult, FrameGroundTruth},
    evaluation_task::EvaluationTask,
    filter::{filter_objects, hash_num_objects, hash_results},
    label::Label,
    matching::{MatchingMode, MatchingResult},
    metrics::{
        error::{MetricsError, MetricsResult},
        score::MetricsScore,
    },
    object::object3d::DynamicObject,
    result::{
        frame::PerceptionFrameResult, object::get_perception_results, object::PerceptionResult,
    },
};

/// Manager of perception evaluation.
///
/// In order to construct, use the `::new()` method.
///
/// For each frame, the evaluated `PerceptionFrameResult` is accumulated in `frame_results`
/// with the `add_frame_result()` method.
///
/// The `get_metrics_score()` method calculates a total metrics score with stacked `frame_results` till that time.
#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager<'a> {
    pub config: &'a PerceptionEvaluationConfig,
    pub frame_ground_truths: Vec<FrameGroundTruth>,
    pub frame_results: Vec<PerceptionFrameResult>,
}

impl<'a> PerceptionEvaluationManager<'a> {
    /// Construct `PerceptionEvaluationManager` from `PerceptionEvaluationConfig`.
    ///
    /// * `config`  - Evaluation configuration.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::{
    ///     config::{get_evaluation_params, PerceptionEvaluationConfig},
    ///     evaluation_task::EvaluationTask,
    ///     frame_id::FrameID,
    ///     manager::PerceptionEvaluationManager,
    /// };  
    /// use std::error::Error;
    ///
    /// type Result<T> = std::result::Result<T, Box<dyn Error>>;
    ///
    /// fn main() -> Result<()> {
    ///     let result_dir = &format!(
    ///         "./work_dir/{}",
    ///         chrono::Local::now().format("%Y%m%d_%H%M%S")
    ///     );
    ///
    ///     let (filter_params, metrics_params) = get_evaluation_params(
    ///         &vec!["Car", "Bus", "Pedestrian"],
    ///         100.0,
    ///         100.0,
    ///         Some(0),
    ///         None,
    ///         1.0,
    ///         2.0,
    ///         0.5,
    ///         0.5,
    ///     )?;
    ///
    ///     let config = PerceptionEvaluationConfig::new(
    ///         "annotation",
    ///         "./tests/sample_data",
    ///         EvaluationTask::Detection,
    ///         FrameID::BaseLink,
    ///         result_dir,
    ///         filter_params,
    ///         metrics_params,
    ///         false,
    ///     );
    ///
    ///     let manager = PerceptionEvaluationManager::from(&config)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from(config: &'a PerceptionEvaluationConfig) -> DatasetResult<Self> {
        let frame_ground_truths = load_dataset(
            &config.version,
            &config.dataset_path,
            &config.evaluation_task,
            &config.frame_id,
        )?;

        let ret = Self {
            config,
            frame_ground_truths,
            frame_results: Vec::new(),
        };
        Ok(ret)
    }

    /// Add estimated objects and ground truths at current frame.
    ///
    /// * `estimated_objects`   - List of estimated objects.
    /// * `frame_ground_truth`  - Set of GTs that has the nearest timestamp.
    pub fn add_frame_result(
        &mut self,
        estimated_objects: &[DynamicObject],
        frame_ground_truth: &FrameGroundTruth,
    ) -> MatchingResult<()> {
        let filtered_estimations =
            filter_objects(estimated_objects, false, &self.config.filter_params);
        let filtered_frame_ground_truth = self.filter_frame_ground_truth(frame_ground_truth);

        let results =
            get_perception_results(&filtered_estimations, &filtered_frame_ground_truth.objects);

        let frame_result = PerceptionFrameResult::new(
            results,
            filtered_frame_ground_truth,
            &self.config.filter_params.target_labels,
            MatchingMode::PlaneDistance,
            &self.config.metrics_params.plane_distance_thresholds,
        )?;
        self.frame_results.push(frame_result);
        Ok(())
    }

    /// Returns `FrameGroundTruth` that has the nearest timestamp to the current timestamp.
    ///
    /// * `timestamp`   - Current timestamp.
    pub fn get_frame_ground_truth(&self, timestamp: &NaiveDateTime) -> Option<FrameGroundTruth> {
        get_current_frame(&self.frame_ground_truths, timestamp)
    }

    /// Returns the `MetricsScore` that calculated metrics score with having been accumulated frame results till that time.
    pub fn get_metrics_score(&self) -> MetricsResult<MetricsScore> {
        let target_labels = &self.config.metrics_params.target_labels;
        let mut score = MetricsScore::new(&self.config.metrics_params);
        let mut scene_results: HashMap<Label, Vec<PerceptionResult>> = HashMap::new();
        let mut num_scene_gt = HashMap::new();

        target_labels.iter().for_each(|label| {
            scene_results.insert(label.to_owned(), Vec::new());
            num_scene_gt.insert(label.to_owned(), 0);
        });

        self.frame_results.iter().for_each(|frame| {
            let mut result_map = hash_results(frame.results(), target_labels);
            let num_gt_map = hash_num_objects(&frame.frame_ground_truth().objects, target_labels);
            target_labels.iter().for_each(|label| {
                if let Some(results) = scene_results.get_mut(label) {
                    if let Some(result) = result_map.get_mut(label) {
                        results.append(result)
                    }
                };
                if let Some(num_gts) = num_scene_gt.get_mut(label) {
                    if let Some(num_gt) = num_gt_map.get(label) {
                        *num_gts += num_gt
                    }
                };
            });
        });

        match self.config.evaluation_task {
            EvaluationTask::Detection => score.evaluate_detection(&scene_results, &num_scene_gt),
            _ => Err(MetricsError::NotImplementedError(
                self.config.evaluation_task.clone(),
            ))?,
        }
        Ok(score)
    }

    /// Filter `FrameGroundTruth` with `FilterParams`.
    ///
    /// * `frame_ground_truth`  - Set of GTs at one frame.
    fn filter_frame_ground_truth(&self, frame_ground_truth: &FrameGroundTruth) -> FrameGroundTruth {
        let filtered_gt = filter_objects(
            &frame_ground_truth.objects,
            true,
            &self.config.filter_params,
        );

        FrameGroundTruth {
            timestamp: frame_ground_truth.timestamp.to_owned(),
            objects: filtered_gt,
        }
    }
}
