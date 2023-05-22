use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::{
    config::PerceptionEvaluationConfig,
    dataset::{get_current_frame, load_dataset, DatasetResult, FrameGroundTruth},
    evaluation_task::EvaluationTask,
    filter::{divide_objects_to_num, divide_results, filter_objects},
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

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager<'a> {
    pub config: &'a PerceptionEvaluationConfig,
    pub frame_ground_truths: Vec<FrameGroundTruth>,
    pub frame_results: Vec<PerceptionFrameResult>,
}

impl<'a> PerceptionEvaluationManager<'a> {
    pub fn new(config: &'a PerceptionEvaluationConfig) -> DatasetResult<Self> {
        let frame_ground_truths = load_dataset(
            &config.version,
            &config.dataset_path,
            &config.evaluation_task,
            &config.frame_id,
        )?;

        let ret = Self {
            config: config,
            frame_ground_truths: frame_ground_truths,
            frame_results: Vec::new(),
        };
        Ok(ret)
    }

    pub fn add_frame_result(
        &mut self,
        estimated_objects: &Vec<DynamicObject>,
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

    pub fn get_frame_ground_truth(&self, timestamp: &NaiveDateTime) -> Option<FrameGroundTruth> {
        get_current_frame(&self.frame_ground_truths, &timestamp)
    }

    pub fn get_scene_score(&self) -> MetricsResult<MetricsScore> {
        let target_labels = &self.config.metrics_params.target_labels;
        let mut score =
            MetricsScore::new(&self.config.evaluation_task, &self.config.metrics_params);
        let mut scene_results: HashMap<String, Vec<PerceptionResult>> = HashMap::new();
        let mut num_scene_gt = HashMap::new();

        target_labels.iter().for_each(|label| {
            let label_name = label.to_string();
            scene_results.insert(label_name.to_owned(), Vec::new());
            num_scene_gt.insert(label_name.to_owned(), 0);
        });

        self.frame_results.iter().for_each(|frame| {
            let mut result_map = divide_results(frame.results(), &target_labels);
            let num_gt_map =
                divide_objects_to_num(&frame.frame_ground_truth().objects, &target_labels);
            for label in target_labels {
                let label_name = label.to_string();
                match scene_results.get_mut(&label_name) {
                    Some(results) => match result_map.get_mut(&label_name) {
                        Some(result) => results.append(result),
                        None => (),
                    },
                    None => (),
                };
                match num_scene_gt.get_mut(&label_name) {
                    Some(num_gts) => match num_gt_map.get(&label_name) {
                        Some(num_gt) => *num_gts += num_gt,
                        None => (),
                    },
                    None => (),
                };
            }
        });

        match self.config.evaluation_task {
            EvaluationTask::Detection => score.evaluate_detection(&scene_results, &num_scene_gt),
            _ => Err(MetricsError::NotImplementedError(
                self.config.evaluation_task.clone(),
            ))?,
        }
        Ok(score)
    }

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
