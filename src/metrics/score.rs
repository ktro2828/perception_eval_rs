use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
};

use crate::{
    config::MetricsParams, label::Label, matching::MatchingMode, result::object::PerceptionResult,
};

use super::detection::DetectionMetricsScore;

#[derive(Debug, Clone)]
pub struct MetricsScore {
    params: MetricsParams,
    scores: Vec<DetectionMetricsScore>,
}

impl Display for MetricsScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut msg = "\n".to_string();
        self.scores
            .iter()
            .for_each(|score| msg += &format!("{}", score));
        write!(f, "{}", msg)
    }
}

impl MetricsScore {
    pub(crate) fn new(params: &MetricsParams) -> Self {
        let scores: Vec<DetectionMetricsScore> = Vec::new();
        Self {
            params: params.to_owned(),
            scores,
        }
    }

    pub(crate) fn evaluate_detection(
        &mut self,
        results_map: &HashMap<Label, Vec<PerceptionResult>>,
        num_gt_map: &HashMap<Label, usize>,
    ) {
        let center_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.params.target_labels,
            &MatchingMode::CenterDistance,
            &self.params.center_distance_thresholds,
        );

        self.scores.push(center_distance_scores_map);

        let plane_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.params.target_labels,
            &MatchingMode::PlaneDistance,
            &self.params.plane_distance_thresholds,
        );

        self.scores.push(plane_distance_scores_map);

        let iou2d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.params.target_labels,
            &MatchingMode::Iou2d,
            &self.params.iou2d_thresholds,
        );

        self.scores.push(iou2d_scores_map);

        let iou3d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.params.target_labels,
            &MatchingMode::Iou3d,
            &self.params.iou3d_thresholds,
        );

        self.scores.push(iou3d_scores_map);
    }
}
