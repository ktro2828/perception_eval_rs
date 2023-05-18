use crate::{config::MetricsParams, evaluation_task::EvaluationTask, label::Label};

#[derive(Debug, Clone)]
pub struct MetricsConfig<'a> {
    pub(crate) evaluation_task: EvaluationTask,
    pub(crate) target_labels: &'a Vec<Label>,
    pub(crate) center_distance_thresholds: &'a Vec<f64>,
    pub(crate) plane_distance_thresholds: &'a Vec<f64>,
    pub(crate) iou2d_thresholds: &'a Vec<f64>,
    pub(crate) iou3d_thresholds: &'a Vec<f64>,
}

impl<'a> MetricsConfig<'a> {
    pub fn new(evaluation_task: EvaluationTask, params: &'a MetricsParams) -> Self {
        Self {
            evaluation_task: evaluation_task,
            target_labels: &params.target_labels,
            center_distance_thresholds: &params.center_distance_thresholds,
            plane_distance_thresholds: &params.plane_distance_thresholds,
            iou2d_thresholds: &params.iou2d_thresholds,
            iou3d_thresholds: &params.iou3d_thresholds,
        }
    }
}
