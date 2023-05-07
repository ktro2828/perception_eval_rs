use chrono::NaiveDateTime;

use crate::{config::PerceptionEvaluationConfig, object::object3d::DynamicObject};

pub trait EvaluationManager {
    fn get_frame_result<T>(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: Vec<DynamicObject>,
    ) -> T;
    fn filter_objects(&self, estimated_objects: Vec<DynamicObject>) -> Vec<DynamicObject>;
    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> FrameGroundTruth;
}

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager {
    config: PerceptionEvaluationConfig,
}

impl EvaluationManager for PerceptionEvaluationManager {
    fn get_frame_result<T>(
        &self,
        timestamp: NaiveDateTime,
        estimated_objects: Vec<DynamicObject>,
    ) -> T {
    }
    fn filter_objects(&self, estimated_objects: Vec<DynamicObject>) {}
    fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) {}
}
