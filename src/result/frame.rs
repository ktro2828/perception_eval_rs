use crate::{
    dataset::FrameGroundTruth,
    filter::{divide_objects_to_num, divide_results},
    label::Label,
    metrics::{config::MetricsConfig, score::MetricsScore},
};

use super::object::PerceptionResult;

#[derive(Debug, Clone)]
pub struct PerceptionFrameResult<'a> {
    pub score: MetricsScore<'a>,
}

impl<'a> PerceptionFrameResult<'a> {
    pub fn new(config: &'a MetricsConfig) -> Self {
        Self {
            score: MetricsScore::new(config),
        }
    }

    pub fn evaluate(
        &self,
        results: &Vec<PerceptionResult>,
        frame_ground_truth: &FrameGroundTruth,
        target_labels: &Vec<Label>,
    ) {
        let results_map = divide_results(results, target_labels);
        let num_gt_map = divide_objects_to_num(&frame_ground_truth.objects, target_labels);
        self.score.evaluate_detection(&results_map, &num_gt_map);
    }
}
