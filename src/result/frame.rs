use crate::{
    dataset::FrameGroundTruth,
    evaluation_task::EvaluationTask,
    filter::{divide_objects_to_num, divide_results},
    metrics::{
        config::MetricsConfig,
        error::{MetricsError, MetricsResult},
        score::MetricsScore,
    },
};

use super::object::PerceptionResult;

#[derive(Debug, Clone)]
pub struct PerceptionFrameResult<'a> {
    pub results: Vec<PerceptionResult>,
    pub frame_ground_truth: FrameGroundTruth,
    pub score: MetricsScore<'a>,
}

impl<'a> PerceptionFrameResult<'a> {
    pub fn new(
        config: &'a MetricsConfig,
        results: Vec<PerceptionResult>,
        frame_ground_truth: FrameGroundTruth,
    ) -> MetricsResult<Self> {
        let mut score = MetricsScore::new(&config);
        let results_map = divide_results(&results, &config.target_labels);
        let num_gt_map = divide_objects_to_num(&frame_ground_truth.objects, &config.target_labels);
        match config.evaluation_task {
            EvaluationTask::Detection => score.evaluate_detection(&results_map, &num_gt_map),
            _ => Err(MetricsError::NotImplementedError(
                config.evaluation_task.clone(),
            ))?,
        }
        Ok(Self {
            results,
            frame_ground_truth,
            score,
        })
    }
}
