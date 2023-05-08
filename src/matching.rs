use crate::point::distance_points;

use super::object::object3d::DynamicObject;
use thiserror::Error as ThisError;

pub type MatchingResult<T> = Result<T, MatchingError>;

#[derive(Debug, ThisError)]
pub enum MatchingError {
    #[error("internal error")]
    InternalError,
    #[error("value error")]
    ValueError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchingMode {
    CenterDistance,
    PlaneDistance,
    Iou2d,
    Iou3d,
}

pub(crate) trait MatchingMethod {
    fn calculate_matching_score(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
    ) -> f64;

    fn is_better_than(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
        threshold: &f64,
    ) -> bool;
}

/// Matching object with euclidean distance of center of objects.
#[derive(Debug, Clone)]
pub struct CenterDistanceMatching;

impl MatchingMethod for CenterDistanceMatching {
    fn calculate_matching_score(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
    ) -> f64 {
        estimated_object.distance_from(&ground_truth_object.position)
    }

    fn is_better_than(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
        threshold: &f64,
    ) -> bool {
        let distance = self.calculate_matching_score(estimated_object, ground_truth_object);
        distance < *threshold
    }
}

// #[derive(Debug, Clone)]
// pub struct PlaneDistanceMatching;

// impl MatchingMethod for PlaneDistanceMatching {
//     fn calculate_matching_score(
//         estimated_object: &DynamicObject,
//         ground_truth_object: &Option<DynamicObject>,
//     ) -> Option<f64> {
//         match ground_truth_object {
//             Some(gt) => {
//                 let mut est_footprint = estimated_object.get_footprint();
//                 let mut gt_footprint = gt.get_footprint();
//             }
//             None => None,
//         }
//     }

//     fn is_better_than(
//         estimated_object: &DynamicObject,
//         ground_truth_object: &Option<DynamicObject>,
//         threshold: &f64,
//     ) -> bool {
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Iou2dMatching;

// impl MatchingMethod for Iou2dMatching {}

// #[derive(Debug, Clone)]
// pub struct Iou3dMatching;

// impl MatchingMethod for Iou3dMatching {}
