use std::vec;

use crate::matching::{CenterDistanceMatching, MatchingError, MatchingMode, MatchingResult};

use super::matching::MatchingMethod;
use super::object::object3d::DynamicObject;

#[derive(Debug, Clone)]
pub struct PerceptionResult {
    estimated_object: DynamicObject,
    ground_truth_object: Option<DynamicObject>,
}

impl PerceptionResult {
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
        let matching_method;
        if *matching_mode == MatchingMode::CenterDistance {
            matching_method = CenterDistanceMatching;
        } else {
            return Err(MatchingError::ValueError);
        }
        let is_correct = {
            match &self.ground_truth_object {
                Some(gt) => matching_method.is_better_than(&self.estimated_object, &gt, threshold),
                None => false,
            }
        };
        Ok(is_correct)
    }
}

pub fn get_perception_results(
    estimated_objects: &Vec<DynamicObject>,
    ground_truth_objects: &Vec<DynamicObject>,
) -> Vec<PerceptionResult> {
    let mut results: Vec<PerceptionResult> = Vec::new();
    let num_estimated_objects = estimated_objects.len();
    let num_ground_truth_objects = ground_truth_objects.len();
    let matching_method = CenterDistanceMatching;

    if num_estimated_objects == 0 {
        results
    } else if num_ground_truth_objects == 0 {
        get_fp_perception_results(estimated_objects)
    } else {
        let mut estimated_object_list = estimated_objects.clone();
        let mut ground_truth_object_list = ground_truth_objects.clone();
        let mut score_table: Vec<Vec<Option<f64>>> =
            get_score_table(estimated_objects, ground_truth_objects, matching_method);
        for _ in 0..num_estimated_objects {
            // TODO
        }
        if 0 < estimated_object_list.len() {
            let mut fp_results = get_fp_perception_results(&estimated_object_list);
            results.append(&mut fp_results);
        }
        results
    }
}

fn get_fp_perception_results(estimated_objects: &Vec<DynamicObject>) -> Vec<PerceptionResult> {
    let mut results: Vec<PerceptionResult> = Vec::new();

    for est in estimated_objects {
        results.push(PerceptionResult {
            estimated_object: est.to_owned(),
            ground_truth_object: None,
        })
    }
    results
}

fn get_score_table<T>(
    estimated_objects: &Vec<DynamicObject>,
    ground_truth_objects: &Vec<DynamicObject>,
    matching_method: T,
) -> Vec<Vec<Option<f64>>>
where
    T: MatchingMethod,
{
    let num_estimated_objects = estimated_objects.len();
    let num_ground_truth_objects = ground_truth_objects.len();
    let mut score_table: Vec<Vec<Option<f64>>> =
        vec![vec![None; num_ground_truth_objects]; num_estimated_objects];
    for (i, est) in estimated_objects.iter().enumerate() {
        for (j, gt) in ground_truth_objects.iter().enumerate() {
            if est.label == gt.label {
                score_table[i][j] = Some(matching_method.calculate_matching_score(est, gt));
            }
        }
    }
    score_table
}
