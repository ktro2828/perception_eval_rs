use crate::{
    label::Label, matching::MatchingMode, result::PerceptionResult, threshold::get_label_threshold,
};

use super::tp_metrics::TPMetrics;

// #[derive(Debug)]
// pub(crate) struct DetectionMetricsScore<'a> {
//     scores: Vec<Ap<'a>>,
// }

// impl<'a> DetectionMetricsScore<'a> {
//     pub(crate) new()
// }

#[derive(Debug)]
pub(super) struct Ap<'a> {
    results: &'a Vec<PerceptionResult>,
    num_ground_truth: isize,
    target_labels: &'a Vec<Label>,
}

impl<'a> Ap<'a> {
    pub(super) fn new(
        results: &'a Vec<PerceptionResult>,
        num_ground_truth: isize,
        target_labels: &'a Vec<Label>,
    ) -> Self {
        Self {
            results: results,
            num_ground_truth: num_ground_truth,
            target_labels: target_labels,
        }
    }

    pub(super) fn calculate_ap<T>(
        &self,
        tp_metrics: T,
        matching_mode: &MatchingMode,
        thresholds: &Vec<f64>,
    ) -> f64
    where
        T: TPMetrics,
    {
        let (tp_list, _) = self.calculate_tp_fp(tp_metrics, matching_mode, thresholds);
        let (precision_list, recall_list) = self.calculate_precision_recall(&tp_list);
        let (max_precision_list, max_recall_list) =
            self.interpolate_precision_recall(precision_list, recall_list);

        if max_precision_list.len() == 0 {
            f64::NAN
        } else {
            let mut ap = 0.0;
            let num_max_precision_list = max_precision_list.len();
            for i in 0..num_max_precision_list - 1 {
                ap += max_precision_list[i] * (max_recall_list[i] - max_recall_list[i + 1]);
            }
            ap
        }
    }

    fn interpolate_precision_recall(
        &self,
        precision_list: Vec<f64>,
        recall_list: Vec<f64>,
    ) -> (Vec<f64>, Vec<f64>) {
        if self.results.len() == 0 && self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            // TODO
            (Vec::new(), Vec::new())
        }
    }

    fn calculate_precision_recall(&self, tp_list: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        if self.results.len() == 0 && self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let num_results = self.results.len();
            let mut precision_list = vec![0.0; num_results];
            let mut recall_list = vec![0.0; num_results];

            for (i, ((precision, recall), tp)) in precision_list
                .iter_mut()
                .zip(recall_list.iter_mut())
                .zip(tp_list.iter())
                .enumerate()
            {
                let i_float = i as f64;
                let num_gt_float = self.num_ground_truth as f64;
                *precision = tp / (i_float + 1.0);
                if self.num_ground_truth > 0 {
                    *recall = tp / num_gt_float;
                }
            }

            (precision_list, recall_list)
        }
    }

    fn calculate_tp_fp<T>(
        &self,
        tp_metrics: T,
        matching_mode: &MatchingMode,
        thresholds: &Vec<f64>,
    ) -> (Vec<f64>, Vec<f64>)
    where
        T: TPMetrics,
    {
        if self.results.len() == 0 && self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let num_results = self.results.len();
            let mut tp_list = vec![0.0; num_results];
            let mut fp_list = vec![0.0; num_results];

            for (i, result) in self.results.iter().enumerate() {
                let threshold = get_label_threshold(
                    &result.estimated_object.label,
                    self.target_labels,
                    thresholds,
                )
                .unwrap();
                if result.is_result_correct(matching_mode, &threshold).unwrap() {
                    tp_list[i] = tp_metrics.get_value(result);
                } else {
                    fp_list[i] = 1.0;
                }
            }

            tp_list.iter_mut().fold(0.0, |acc, x| {
                *x += acc;
                *x
            });

            fp_list.iter_mut().fold(0.0, |acc, x| {
                *x += acc;
                *x
            });

            (tp_list, fp_list)
        }
    }
}
