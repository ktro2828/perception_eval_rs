use std::f64::consts::PI;

use crate::result::object::PerceptionResult;

/// Trait for TP metrics strategy.
pub(super) trait TPMetrics {
    /// Returns TP score depending on strategy.
    ///
    /// * `result`  - PerceptionResult instance.
    fn get_value(&self, result: &PerceptionResult) -> f64;
}

/// AP metrics that always returns 1.0 for TP results.
#[derive(Debug)]
pub(super) struct TPMetricsAP;

impl TPMetrics for TPMetricsAP {
    fn get_value(&self, result: &PerceptionResult) -> f64 {
        match &result.ground_truth_object {
            Some(_) => 1.0,
            None => 0.0,
        }
    }
}

/// APH metrics that returns the error of heading between estimation and GT.
#[derive(Debug)]
pub(super) struct TPMetricsAPH;

impl TPMetrics for TPMetricsAPH {
    fn get_value(&self, result: &PerceptionResult) -> f64 {
        match &result.ground_truth_object {
            Some(gt) => {
                let mut diff_heading = (result.estimated_object.heading() - gt.heading()).abs();

                if PI < diff_heading {
                    diff_heading = 2.0 * PI - diff_heading;
                }
                (1.0 - diff_heading / PI).max(0.0).min(1.0)
            }
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TPMetrics;
    use crate::{
        frame_id::FrameID,
        label::Label,
        metrics::tp_metrics::{TPMetricsAP, TPMetricsAPH},
        object::object3d::DynamicObject,
        result::object::PerceptionResult,
    };
    use chrono::NaiveDateTime;

    #[test]
    fn test_tp_metrics_ap() {
        let estimation = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [1.0, 1.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };

        let ground_truth = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [10.0, 10.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };
        let result = PerceptionResult::new(estimation, Some(ground_truth));
        let value = TPMetricsAP.get_value(&result);
        assert_eq!(value, 1.0);
    }

    #[test]
    fn test_tp_metrics_aph() {
        let estimation = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [1.0, 1.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };

        let ground_truth = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [10.0, 10.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };
        let result = PerceptionResult::new(estimation, Some(ground_truth));
        let value = TPMetricsAPH.get_value(&result);
        assert_eq!(value, 1.0);
    }
}
