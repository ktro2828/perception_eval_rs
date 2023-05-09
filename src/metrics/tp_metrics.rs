use std::f64::consts::PI;

use crate::result::PerceptionResult;

pub(super) trait TPMetrics {
    fn get_value(&self, result: &PerceptionResult) -> f64;
}

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
