use std::vec;

use crate::matching::{
    CenterDistanceMatching, Iou2dMatching, Iou3dMatching, MatchingMode, MatchingResult,
    PlaneDistanceMatching,
};

use super::matching::MatchingMethod;
use super::object::object3d::DynamicObject;

#[derive(Debug, Clone)]
pub struct PerceptionResult {
    estimated_object: DynamicObject,
    ground_truth_object: Option<DynamicObject>,
}

impl PerceptionResult {
    pub fn new(
        estimated_object: DynamicObject,
        ground_truth_object: Option<DynamicObject>,
    ) -> Self {
        Self {
            estimated_object: estimated_object,
            ground_truth_object: ground_truth_object,
        }
    }
    pub fn is_label_correct(&self) -> bool {
        match &self.ground_truth_object {
            Some(gt) => self.estimated_object.label == gt.label,
            None => false,
        }
    }

    pub fn is_result_correct(
        &self,
        matching_mode: &MatchingMode,
        threshold: &f64,
    ) -> MatchingResult<bool> {
        let matching_method: Box<dyn MatchingMethod> = {
            match matching_mode {
                MatchingMode::CenterDistance => Box::new(CenterDistanceMatching),
                MatchingMode::PlaneDistance => Box::new(PlaneDistanceMatching),
                MatchingMode::Iou2d => Box::new(Iou2dMatching),
                MatchingMode::Iou3d => Box::new(Iou3dMatching),
            }
        };
        let is_correct = {
            match &self.ground_truth_object {
                Some(gt) => matching_method.is_better_than(&self.estimated_object, &gt, threshold),
                None => false,
            }
        };
        Ok(is_correct)
    }
}

/// Returns list of `PerceptionResult`.
///
/// * `estimated_objects`       - List of estimated objects.
/// * `ground_truth_objects`    - List of ground truth objects.
pub fn get_perception_results(
    estimated_objects: &Vec<DynamicObject>,
    ground_truth_objects: &Vec<DynamicObject>,
) -> Vec<PerceptionResult> {
    let mut results: Vec<PerceptionResult> = Vec::new();

    // Use CenterDistance by default
    let matching_method = CenterDistanceMatching;

    if estimated_objects.len() == 0 {
        results
    } else if ground_truth_objects.len() == 0 {
        get_fp_perception_results(estimated_objects)
    } else {
        let mut estimated_object_list = estimated_objects.clone();
        let mut ground_truth_object_list = ground_truth_objects.clone();
        let mut score_table: Vec<Vec<Option<f64>>> =
            get_score_table(estimated_objects, ground_truth_objects, matching_method);
        for _ in 0..estimated_objects.len() {
            for (est_idx, row_table) in score_table.iter_mut().enumerate() {
                let (gt_idx, _) = row_table.iter().enumerate().fold(
                    (usize::MAX, f64::MAX),
                    |(a_idx, a), (b_idx, b)| match b {
                        Some(b) => {
                            if a < *b {
                                (a_idx, a)
                            } else {
                                (b_idx, *b)
                            }
                        }
                        None => (a_idx, a),
                    },
                );

                results.push(PerceptionResult {
                    estimated_object: estimated_object_list[est_idx].to_owned(),
                    ground_truth_object: Some(ground_truth_object_list[gt_idx].to_owned()),
                });

                row_table[gt_idx] = None;
                estimated_object_list.remove(est_idx);
                ground_truth_object_list.remove(gt_idx);
            }
        }

        if 0 < estimated_object_list.len() {
            let mut fp_results = get_fp_perception_results(&estimated_object_list);
            results.append(&mut fp_results);
        }
        results
    }
}

/// Returns list of `PerceptionResult` that ground_truth_object of each result is None, it means FP.
///
/// * `estimated_objects`   - List of estimated objects.
fn get_fp_perception_results(estimated_objects: &Vec<DynamicObject>) -> Vec<PerceptionResult> {
    estimated_objects
        .iter()
        .map(|obj| PerceptionResult::new(obj.to_owned(), None))
        .collect::<Vec<PerceptionResult>>()
}

/// Returns NxM score table.
/// N: Number of estimated objects.
/// M: Number of ground truth objects.
///
/// * `estimated_objects`       - List of estimated objects.
/// * `ground_truth_objects`    - List of ground truth objects.
/// * `matching_method`         - MatchingMethod instance.
fn get_score_table<T>(
    estimated_objects: &Vec<DynamicObject>,
    ground_truth_objects: &Vec<DynamicObject>,
    matching_method: T,
) -> Vec<Vec<Option<f64>>>
where
    T: MatchingMethod,
{
    let num_est = estimated_objects.len();
    let num_gt = ground_truth_objects.len();

    // TODO: refactoring
    let mut score_table: Vec<Vec<Option<f64>>> = vec![vec![None; num_gt]; num_est];
    for (i, est) in estimated_objects.iter().enumerate() {
        for (j, gt) in ground_truth_objects.iter().enumerate() {
            if est.label == gt.label {
                score_table[i][j] = Some(matching_method.calculate_matching_score(est, gt));
            }
        }
    }
    score_table
}
