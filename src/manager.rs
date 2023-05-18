use chrono::NaiveDateTime;

use crate::{
    config::PerceptionEvaluationConfig,
    dataset::{get_current_frame, load_dataset, nuscenes::error::NuScenesResult, FrameGroundTruth},
    filter::filter_objects,
    object::object3d::DynamicObject,
    result::{frame::PerceptionFrameResult, object::get_perception_results},
};

pub trait EvaluationManager {
    fn add_frame_result(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: &mut Vec<DynamicObject>,
        frame_ground_truth: &mut FrameGroundTruth,
    );
    fn filter_objects(&self, objects: &Vec<DynamicObject>, is_gt: bool) -> Vec<DynamicObject>;
    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> Option<FrameGroundTruth>;
}

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager<'a> {
    config: PerceptionEvaluationConfig,
    frame_ground_truths: Vec<FrameGroundTruth>,
    frame_results: Vec<PerceptionFrameResult<'a>>,
}

impl<'a> EvaluationManager for PerceptionEvaluationManager<'a> {
    fn add_frame_result(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: &mut Vec<DynamicObject>,
        frame_ground_truth: &mut FrameGroundTruth,
    ) {
        let estimated_objects = self.filter_objects(estimated_objects, false);
        let frame_gt = self.filter_objects(frame_ground_truth.objects.as_mut(), true);
        frame_ground_truth.objects = frame_gt;

        let object_results =
            get_perception_results(&estimated_objects, &frame_ground_truth.objects);

        let metrics_config = self
            .config
            .metrics_params
            .get_metrics_config(&self.config.evaluation_task);
        let frame_result = PerceptionFrameResult::new(&metrics_config);
        self.frame_results.push(frame_result)
    }

    fn filter_objects(&self, objects: &Vec<DynamicObject>, is_gt: bool) -> Vec<DynamicObject> {
        filter_objects(objects, is_gt, &self.config.filter_params)
    }

    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> Option<FrameGroundTruth> {
        get_current_frame(&self.frame_ground_truths, &timestamp)
    }
}

impl<'a> PerceptionEvaluationManager<'a> {
    pub fn new(config: PerceptionEvaluationConfig) -> NuScenesResult<Self> {
        let frame_ground_truths = load_dataset(
            config.version,
            config.dataset_path,
            &config.evaluation_task,
            &config.frame_id,
        )?;

        let ret = Self {
            config: config.to_owned(),
            frame_ground_truths: frame_ground_truths,
            frame_results: Vec::new(),
        };
        Ok(ret)
    }
}
