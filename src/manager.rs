use chrono::NaiveDateTime;

use crate::{
    config::PerceptionEvaluationConfig,
    dataset::{get_current_frame, load_dataset, nuscenes::error::NuScenesResult, FrameGroundTruth},
    filter::filter_objects,
    metrics::error::MetricsResult,
    object::object3d::DynamicObject,
    result::{frame::PerceptionFrameResult, object::get_perception_results},
};

#[derive(Debug, Clone)]
pub struct PerceptionEvaluationManager<'a> {
    config: &'a PerceptionEvaluationConfig,
    frame_ground_truths: Vec<FrameGroundTruth>,
    frame_results: Vec<PerceptionFrameResult<'a>>,
}

impl<'a> PerceptionEvaluationManager<'a> {
    pub fn new(config: &'a PerceptionEvaluationConfig) -> NuScenesResult<Self> {
        let frame_ground_truths = load_dataset(
            config.version.to_owned(),
            config.dataset_path.to_owned(),
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
        estimated_objects: &mut Vec<DynamicObject>,
        frame_ground_truth: &mut FrameGroundTruth,
    ) -> MetricsResult<()> {
        let estimated_objects = self.filter_objects(estimated_objects, false);
        let frame_gt = self.filter_objects(frame_ground_truth.objects.as_mut(), true);
        frame_ground_truth.objects = frame_gt;

        let object_results =
            get_perception_results(&estimated_objects, &frame_ground_truth.objects);

        let frame_result = PerceptionFrameResult::new(
            &self.config.metrics_config,
            object_results,
            frame_ground_truth.clone(),
        )?;
        self.frame_results.push(frame_result);
        Ok(())
    }

    pub fn get_frame_ground_truth(&self, timestamp: NaiveDateTime) -> Option<FrameGroundTruth> {
        get_current_frame(&self.frame_ground_truths, &timestamp)
    }

    fn filter_objects(&self, objects: &Vec<DynamicObject>, is_gt: bool) -> Vec<DynamicObject> {
        filter_objects(objects, is_gt, &self.config.filter_params)
    }
}
