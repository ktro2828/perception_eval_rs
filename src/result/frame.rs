use crate::{
    dataset::FrameGroundTruth,
    label::Label,
    matching::{MatchingMode, MatchingResult},
    object::object3d::DynamicObject,
    threshold::get_label_threshold,
};

use super::object::PerceptionResult;

#[derive(Debug, Clone)]
pub struct PerceptionFrameResult {
    results: Vec<PerceptionResult>,
    frame_ground_truth: FrameGroundTruth,
    tp_results: Vec<PerceptionResult>,
    fp_results: Vec<PerceptionResult>,
    fn_objects: Vec<DynamicObject>,
}

impl PerceptionFrameResult {
    pub fn results(&self) -> &Vec<PerceptionResult> {
        &self.results
    }

    pub fn frame_ground_truth(&self) -> &FrameGroundTruth {
        &self.frame_ground_truth
    }

    pub fn tp_results(&self) -> &Vec<PerceptionResult> {
        &self.tp_results
    }

    pub fn fp_results(&self) -> &Vec<PerceptionResult> {
        &self.fp_results
    }

    pub fn fn_objects(&self) -> &Vec<DynamicObject> {
        &self.fn_objects
    }

    pub fn new(
        results: Vec<PerceptionResult>,
        frame_ground_truth: FrameGroundTruth,
        target_labels: &Vec<Label>,
        matching_mode: MatchingMode,
        matching_thresholds: &Vec<f64>,
    ) -> MatchingResult<Self> {
        let (tp_results, fp_results) =
            separate_tp_fp_results(&results, target_labels, &matching_mode, matching_thresholds)?;
        let fn_objects = extract_fn_objects(&frame_ground_truth.objects, &tp_results);

        let ret = Self {
            results: results,
            frame_ground_truth: frame_ground_truth,
            tp_results: tp_results,
            fp_results: fp_results,
            fn_objects: fn_objects,
        };

        Ok(ret)
    }
}

/// Separate results into TP and FP results.
///
/// TODO: remove clone
///
/// * `results`
/// * `target_labels`
/// * `matching_mode`
/// * `matching_thresholds`
fn separate_tp_fp_results(
    results: &Vec<PerceptionResult>,
    target_labels: &Vec<Label>,
    matching_mode: &MatchingMode,
    matching_thresholds: &Vec<f64>,
) -> MatchingResult<(Vec<PerceptionResult>, Vec<PerceptionResult>)> {
    let mut tp_results = Vec::new();
    let mut fp_results = Vec::new();
    results.iter().for_each(|result| {
        match get_label_threshold(
            &result.estimated_object.label,
            target_labels,
            matching_thresholds,
        ) {
            Some(threshold) => {
                let is_correct = result.is_result_correct(matching_mode, &threshold).unwrap(); // TODO
                if is_correct {
                    tp_results.push(result.clone());
                } else {
                    fp_results.push(result.clone());
                }
            }
            None => (),
        }
    });
    Ok((tp_results, fp_results))
}

/// Extract FN objects.
///
/// TODO: remove clone
///
/// * `ground_truths`
/// * `tp_results`
fn extract_fn_objects(
    ground_truths: &Vec<DynamicObject>,
    tp_results: &Vec<PerceptionResult>,
) -> Vec<DynamicObject> {
    let mut fn_objects = Vec::new();

    ground_truths.iter().for_each(|gt| {
        if is_fn_object(gt, tp_results) {
            fn_objects.push(gt.clone());
        }
    });

    fn_objects
}

/// Check whether input ground truth is contained in the input list of TP results.
///
/// * `ground_truth`
/// * `tp_results`
fn is_fn_object(ground_truth: &DynamicObject, tp_results: &Vec<PerceptionResult>) -> bool {
    for tp in tp_results {
        match &tp.ground_truth_object {
            Some(gt) => {
                if gt == ground_truth {
                    return false;
                }
            }
            None => (),
        }
    }
    true
}
