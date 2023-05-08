use crate::result::PerceptionResult;

use super::config::MetricsConfig;

pub(crate) struct MetricsScore<'a> {
    pub(crate) config: &'a MetricsConfig<'a>,
}

impl<'a> MetricsScore<'a> {
    pub(crate) fn new(config: &'a MetricsConfig) -> Self {
        Self { config: config }
    }

    pub(crate) fn evaluate_detection(&self, results: &Vec<PerceptionResult>) {
        for threshold in self.config.center_distance_thresholds {}

        for threshold in self.config.plane_distance_thresholds {}

        for threshold in self.config.iou2d_thresholds {}

        for threshold in self.config.iou3d_thresholds {}
    }
}
