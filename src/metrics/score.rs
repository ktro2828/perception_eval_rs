use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
};

use crate::{matching::MatchingMode, result::object::PerceptionResult};

use super::{config::MetricsConfig, detection::DetectionMetricsScore};

#[derive(Debug, Clone)]
pub struct MetricsScore<'a> {
    pub(crate) config: &'a MetricsConfig,
    pub(crate) scores: Vec<DetectionMetricsScore>,
}

impl<'a> Display for MetricsScore<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut msg = "\n".to_string();
        self.scores
            .iter()
            .for_each(|score| msg += &format!("{}", score));
        write!(f, "{}", msg)
    }
}

impl<'a> MetricsScore<'a> {
    pub fn new(config: &'a MetricsConfig) -> Self {
        let scores: Vec<DetectionMetricsScore> = Vec::new();
        Self {
            config: config,
            scores: scores,
        }
    }

    pub fn evaluate_detection(
        &mut self,
        results_map: &HashMap<String, Vec<PerceptionResult>>,
        num_gt_map: &HashMap<String, usize>,
    ) {
        let center_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.config.target_labels,
            &MatchingMode::CenterDistance,
            &self.config.center_distance_thresholds,
        );

        self.scores.push(center_distance_scores_map);

        let plane_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.config.target_labels,
            &MatchingMode::PlaneDistance,
            &self.config.plane_distance_thresholds,
        );

        self.scores.push(plane_distance_scores_map);

        let iou2d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.config.target_labels,
            &MatchingMode::Iou2d,
            &self.config.iou2d_thresholds,
        );

        self.scores.push(iou2d_scores_map);

        let iou3d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            &self.config.target_labels,
            &MatchingMode::Iou3d,
            &self.config.iou3d_thresholds,
        );

        self.scores.push(iou3d_scores_map);
    }
}
