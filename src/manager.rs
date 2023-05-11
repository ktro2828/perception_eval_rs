use chrono::NaiveDateTime;

use crate::{
    config::PerceptionEvaluationConfig,
    dataset::{get_current_frame, load_dataset, nuscenes::error::NuScenesResult, FrameGroundTruth},
    filter::filter_objects,
    object::object3d::DynamicObject,
};

pub trait EvaluationManager {
    fn get_frame_result<T>(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: Vec<DynamicObject>,
    ) -> T;
    fn filter_objects(&self, objects: &Vec<DynamicObject>, is_gt: bool) -> Vec<DynamicObject>;
    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> Option<FrameGroundTruth>;
}

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager {
    config: PerceptionEvaluationConfig,
    frame_ground_truths: Vec<FrameGroundTruth>,
}

impl EvaluationManager for PerceptionEvaluationManager {
    fn get_frame_result<T>(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: Vec<DynamicObject>,
    ) -> T {
    }
    fn filter_objects(&self, objects: &Vec<DynamicObject>, is_gt: bool) -> Vec<DynamicObject> {
        filter_objects(objects, is_gt, &self.config.filter_params)
    }
    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> Option<FrameGroundTruth> {
        get_current_frame(&self.frame_ground_truths, &timestamp)
    }
}

impl PerceptionEvaluationManager {
    pub fn new(config: PerceptionEvaluationConfig) -> NuScenesResult<Self> {
        let frame_ground_truths = load_dataset(version, data_root, evaluation_task, frame_id)?;
        let ret = Self {
            config: config,
            frame_ground_truths: frame_ground_truths,
        };
        Ok(ret)
    }
}
