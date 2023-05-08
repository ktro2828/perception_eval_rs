use crate::point::{distance_points_bev, get_point_left_right};

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

#[derive(Debug, Clone)]
pub struct PlaneDistanceMatching;

impl MatchingMethod for PlaneDistanceMatching {
    fn calculate_matching_score(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
    ) -> f64 {
        let mut est_footprint = estimated_object.footprint();
        est_footprint.sort_by(|p1, p2| {
            let d1 = p1[0].powi(2) + p1[1].powi(2);
            let d2 = p2[0].powi(2) + p1[1].powi(2);
            d1.partial_cmp(&d2).unwrap()
        });

        let mut gt_footprint = ground_truth_object.footprint();
        gt_footprint.sort_by(|p1, p2| {
            let d1 = p1[0].powi(2) + p1[1].powi(2);
            let d2 = p2[0].powi(2) + p1[1].powi(2);
            d1.partial_cmp(&d2).unwrap()
        });

        let (est_left_point, est_right_point) =
            get_point_left_right(&est_footprint[0], &est_footprint[1]);

        let (gt_left_point, gt_right_point) =
            get_point_left_right(&gt_footprint[0], &gt_footprint[1]);

        let distance_left = distance_points_bev(est_left_point, gt_left_point).abs();
        let distance_right = distance_points_bev(est_right_point, gt_right_point).abs();

        ((distance_left.powi(2) + distance_right.powi(2)) / 2.0).sqrt()
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
// pub struct Iou2dMatching;

// impl MatchingMethod for Iou2dMatching {}

// #[derive(Debug, Clone)]
// pub struct Iou3dMatching;

// impl MatchingMethod for Iou3dMatching {}
