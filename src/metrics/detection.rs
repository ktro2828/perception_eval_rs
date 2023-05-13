use super::tp_metrics::{TPMetrics, TPMetricsAP, TPMetricsAPH};
use crate::{label::Label, matching::MatchingMode, result::PerceptionResult};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct DetectionMetricsScore {
    pub(crate) scores: HashMap<String, f64>,
}

impl DetectionMetricsScore {
    pub(crate) fn new(
        results_map: &HashMap<String, Vec<PerceptionResult>>,
        num_gt_map: &HashMap<String, usize>,
        target_labels: &Vec<Label>,
        matching_mode: &MatchingMode,
        matching_thresholds: &Vec<f64>,
    ) -> Self {
        let mut scores = HashMap::new();
        let mut ap = 0.0;
        let mut aph = 0.0;
        let mut n = 0.0;
        for (target_label, threshold) in target_labels.iter().zip(matching_thresholds.iter()) {
            let results = results_map[&target_label.to_string()];
            let num_gt = num_gt_map[&target_label.to_string()];
            ap += Ap::new(&results, &num_gt, target_label).calculate_ap(
                TPMetricsAP,
                matching_mode,
                threshold,
            );

            aph += Ap::new(&results, &num_gt, target_label).calculate_ap(
                TPMetricsAPH,
                matching_mode,
                threshold,
            );

            n += 1.0;
        }

        scores.insert("ap".to_string(), ap / n);
        scores.insert("aph".to_string(), aph / n);

        Self { scores: scores }
    }
}

#[derive(Debug)]
pub(super) struct Ap<'a> {
    results: &'a Vec<PerceptionResult>,
    num_ground_truth: &'a usize,
    target_label: &'a Label,
}

impl<'a> Ap<'a> {
    pub(super) fn new(
        results: &'a Vec<PerceptionResult>,
        num_ground_truth: &usize,
        target_label: &'a Label,
    ) -> Self {
        Self {
            results: results,
            num_ground_truth: num_ground_truth,
            target_label: target_label,
        }
    }

    pub(super) fn calculate_ap<T>(
        &self,
        tp_metrics: T,
        matching_mode: &MatchingMode,
        threshold: &f64,
    ) -> f64
    where
        T: TPMetrics,
    {
        let (tp_list, _) = self.calculate_tp_fp(tp_metrics, matching_mode, threshold);
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
        if self.results.len() == 0 && *self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let mut max_precision_list = vec![*precision_list.last().unwrap()];
            let mut max_recall_list = vec![*recall_list.last().unwrap()];
            for i in 0..recall_list.len() - 1 {
                if max_precision_list.last().unwrap() < &precision_list[i] {
                    max_precision_list.push(precision_list[i]);
                    max_recall_list.push(recall_list[i]);
                }
            }
            (max_precision_list, max_recall_list)
        }
    }

    fn calculate_precision_recall(&self, tp_list: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        if self.results.len() == 0 && *self.num_ground_truth == 0 {
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
                let num_gt_float = *self.num_ground_truth as f64;
                *precision = tp / (i_float + 1.0);
                if *self.num_ground_truth > 0 {
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
        threshold: &f64,
    ) -> (Vec<f64>, Vec<f64>)
    where
        T: TPMetrics,
    {
        if self.results.len() == 0 && *self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let num_results = self.results.len();
            let mut tp_list = vec![0.0; num_results];
            let mut fp_list = vec![0.0; num_results];

            for (i, result) in self.results.iter().enumerate() {
                if result.is_result_correct(matching_mode, threshold).unwrap() {
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
