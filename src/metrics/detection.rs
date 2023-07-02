use super::tp_metrics::{TPMetrics, TPMetricsAP, TPMetricsAPH};
use crate::{label::Label, matching::MatchingMode, result::object::PerceptionResult};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
};

/// Manager to calculate metrics score for detection task.
#[derive(Debug, Clone)]
pub(crate) struct DetectionMetricsScore {
    pub(crate) target_labels: Vec<Label>,
    pub(crate) matching_mode: MatchingMode,
    pub(crate) thresholds: Vec<f64>,
    pub(crate) scores: HashMap<String, Vec<f64>>,
}

impl DetectionMetricsScore {
    /// Construct `DetectionMetricsScore`.
    ///
    /// * `results_map`         - Hashmap that key is the name of label and value is list of corresponding PerceptionResult.
    /// * `num_gt_map`          - Hashmap that key is the name of label and value is the number of corresponding GTs.
    /// * `target_labels`       - List of Label instances.
    /// * `matching_mode`       - MatchingMode instance.
    /// * `matching_thresholds` - List of matching thresholds.
    pub(crate) fn new(
        results_map: &HashMap<Label, Vec<PerceptionResult>>,
        num_gt_map: &HashMap<Label, usize>,
        target_labels: &Vec<Label>,
        matching_mode: &MatchingMode,
        matching_thresholds: &Vec<f64>,
    ) -> Self {
        let mut scores = HashMap::new();
        let num_targets = target_labels.len();
        let mut ap_list = vec![0.0; num_targets];
        let mut aph_list = vec![0.0; num_targets];
        for (i, (target_label, threshold)) in target_labels
            .iter()
            .zip(matching_thresholds.iter())
            .enumerate()
        {
            let results = results_map.get(target_label).unwrap();
            let num_gt = num_gt_map.get(target_label).unwrap();
            ap_list[i] =
                Ap::new(results, num_gt).calculate_ap(TPMetricsAP, matching_mode, threshold);
            aph_list[i] =
                Ap::new(results, num_gt).calculate_ap(TPMetricsAPH, matching_mode, threshold);
        }

        scores.insert(String::from("AP"), ap_list);
        scores.insert(String::from("APH"), aph_list);

        // TODO: Refactor DO NOT USE to_owned()
        Self {
            target_labels: target_labels.to_owned(),
            matching_mode: matching_mode.to_owned(),
            thresholds: matching_thresholds.to_owned(),
            scores,
        }
    }
}

impl Display for DetectionMetricsScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut msg = "\n".to_string();
        msg += &format!("[{:?}]\n", self.matching_mode);

        self.scores.iter().for_each(|(key, values)| {
            msg += &format!(
                "m{}: {:.3} ",
                key,
                values.iter().sum::<f64>() / values.len() as f64
            )
        });

        msg += &format!("\n|{0:>10}|", "Label");
        self.target_labels
            .iter()
            .enumerate()
            .for_each(|(i, label)| {
                msg += &format!("{0:^10}({1:<.3}) |", label, self.thresholds[i])
            });

        self.scores.iter().for_each(|(key, values)| {
            msg += &format!("\n|{0:>10}|", key);
            values
                .iter()
                .for_each(|ap| msg += &format!(" {0:>10.3} | ", ap));
        });

        writeln!(f, "{}\n", msg)
    }
}

/// Manager to calculate Average Precision (AP) and Average Precision Heading (APH) for each set of labels.
#[derive(Debug)]
pub(super) struct Ap<'a> {
    results: &'a Vec<PerceptionResult>,
    num_ground_truth: &'a usize,
}

impl<'a> Ap<'a> {
    /// Construct `Ap`  instance.
    ///
    /// * `results`             - List of PerceptionResult.
    /// * `num_ground_truth`    - Number of GTs.
    pub(super) fn new(results: &'a Vec<PerceptionResult>, num_ground_truth: &'a usize) -> Self {
        Self {
            results,
            num_ground_truth,
        }
    }

    /// Calculate AP or APH score. Whether which metrics is used, that means AP or APH, depends on `tp_metrics`.
    ///
    /// * `tp_metrics`      - TP metrics. `TPMetricsAP` or `TPMetricsAPH`.
    /// * `matching_mode`   - MatchingMode instance.
    /// * `threshold`       - Matching threshold.
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

        if max_precision_list.is_empty() {
            f64::NAN
        } else {
            let mut ap = 0.0;
            for i in 0..max_precision_list.len() - 1 {
                ap += max_precision_list[i] * (max_recall_list[i] - max_recall_list[i + 1]);
            }
            ap
        }
    }

    /// Interpolate precision and recall values.
    ///
    /// * `precision_list`  - List of precisions.
    /// * `recall_list`     - List of recalls.
    fn interpolate_precision_recall(
        &self,
        precision_list: Vec<f64>,
        recall_list: Vec<f64>,
    ) -> (Vec<f64>, Vec<f64>) {
        if self.results.is_empty() && *self.num_ground_truth == 0 {
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

    /// Compute precision and recall values.
    ///
    /// * `tp_list` - List of TP values.
    fn calculate_precision_recall(&self, tp_list: &[f64]) -> (Vec<f64>, Vec<f64>) {
        if self.results.is_empty() && *self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let num_results = self.results.len();
            let mut precision_list = vec![0.0; num_results];
            let mut recall_list = vec![0.0; num_results];

            precision_list
                .iter_mut()
                .zip(recall_list.iter_mut())
                .zip(tp_list.iter())
                .enumerate()
                .for_each(|(i, ((precision, recall), tp))| {
                    *precision = tp / (1.0 + i as f64);
                    if *self.num_ground_truth > 0 {
                        *recall = tp / *self.num_ground_truth as f64;
                    }
                });
            (precision_list, recall_list)
        }
    }

    /// Compute TP and FP values.
    ///
    /// * `tp_metrics`      - TP metrics.
    /// * `matching_mode`   - MatchingMode instance.
    /// * `threshold`       - Threshold value.
    fn calculate_tp_fp<T>(
        &self,
        tp_metrics: T,
        matching_mode: &MatchingMode,
        threshold: &f64,
    ) -> (Vec<f64>, Vec<f64>)
    where
        T: TPMetrics,
    {
        if self.results.is_empty() && *self.num_ground_truth == 0 {
            (Vec::new(), Vec::new())
        } else {
            let num_results = self.results.len();
            let mut tp_list = vec![0.0; num_results];
            let mut fp_list = vec![0.0; num_results];

            self.results.iter().enumerate().for_each(|(i, result)| {
                if result.is_result_correct(matching_mode, threshold).unwrap() {
                    tp_list[i] = tp_metrics.get_value(result);
                } else {
                    fp_list[i] = 1.0;
                }
            });

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
