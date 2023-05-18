use std::collections::HashMap;

use crate::{matching::MatchingMode, result::object::PerceptionResult};

use super::{config::MetricsConfig, detection::DetectionMetricsScore};

#[derive(Debug, Clone)]
pub(crate) struct MetricsScore<'a> {
    pub(crate) config: &'a MetricsConfig<'a>,
    pub(crate) scores: HashMap<String, HashMap<String, f64>>,
}

impl<'a> MetricsScore<'a> {
    pub(crate) fn new(config: &'a MetricsConfig) -> Self {
        let scores: HashMap<String, HashMap<String, f64>> = HashMap::new();
        Self {
            config: config,
            scores: scores,
        }
    }

    pub(crate) fn evaluate_detection(
        &self,
        results_map: &HashMap<String, Vec<PerceptionResult>>,
        num_gt_map: &HashMap<String, usize>,
    ) -> HashMap<String, HashMap<String, f64>> {
        let mut scores = HashMap::new();
        let center_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            self.config.target_labels,
            &MatchingMode::CenterDistance,
            self.config.center_distance_thresholds,
        )
        .scores;

        scores.insert("CenterDistance".to_string(), center_distance_scores_map);

        let plane_distance_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            self.config.target_labels,
            &MatchingMode::PlaneDistance,
            self.config.plane_distance_thresholds,
        )
        .scores;

        scores.insert("PlaneDistance".to_string(), plane_distance_scores_map);

        let iou2d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            self.config.target_labels,
            &MatchingMode::Iou2d,
            self.config.iou2d_thresholds,
        )
        .scores;

        scores.insert("Iou2d".to_string(), iou2d_scores_map);

        let iou3d_scores_map = DetectionMetricsScore::new(
            results_map,
            num_gt_map,
            self.config.target_labels,
            &MatchingMode::Iou3d,
            self.config.iou3d_thresholds,
        )
        .scores;

        scores.insert("Iou3d".to_string(), iou3d_scores_map);

        scores
    }
}
