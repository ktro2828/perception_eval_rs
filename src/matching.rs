use crate::utils::point::{distance_points_bev, get_point_left_right};

use super::object::object3d::DynamicObject;
use geo::{polygon, Area, BooleanOps, Coord, Polygon};
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
            let d1 = p1[0].hypot(p1[1]);
            let d2 = p2[0].hypot(p2[1]);
            d1.partial_cmp(&d2).unwrap()
        });

        let mut gt_footprint = ground_truth_object.footprint();
        gt_footprint.sort_by(|p1, p2| {
            let d1 = p1[0].hypot(p1[1]);
            let d2 = p2[0].hypot(p2[1]);
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

#[derive(Debug, Clone)]
pub struct Iou2dMatching;

impl MatchingMethod for Iou2dMatching {
    fn calculate_matching_score(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
    ) -> f64 {
        let est_area = estimated_object.area();
        let gt_area = ground_truth_object.area();
        let intersection_area = get_intersection_area(estimated_object, ground_truth_object);
        intersection_area / (est_area + gt_area - intersection_area)
    }

    fn is_better_than(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
        threshold: &f64,
    ) -> bool {
        let iou = self.calculate_matching_score(estimated_object, ground_truth_object);
        *threshold < iou
    }
}

#[derive(Debug, Clone)]
pub struct Iou3dMatching;

impl MatchingMethod for Iou3dMatching {
    fn calculate_matching_score(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
    ) -> f64 {
        let est_volume = estimated_object.volume();
        let gt_volume = ground_truth_object.volume();
        let intersection_volume = get_intersection_volume(estimated_object, ground_truth_object);
        intersection_volume / (est_volume + gt_volume - intersection_volume)
    }

    fn is_better_than(
        &self,
        estimated_object: &DynamicObject,
        ground_truth_object: &DynamicObject,
        threshold: &f64,
    ) -> bool {
        let iou = self.calculate_matching_score(estimated_object, ground_truth_object);
        *threshold < iou
    }
}

fn get_intersection_area(
    estimated_object: &DynamicObject,
    ground_truth_object: &DynamicObject,
) -> f64 {
    let get_polygon = |object: &DynamicObject| -> Polygon<f64> {
        let footprint = object.footprint();
        let poly = polygon![
            Coord {
                x: footprint[0][0],
                y: footprint[0][1]
            },
            Coord {
                x: footprint[1][0],
                y: footprint[1][1]
            },
            Coord {
                x: footprint[2][0],
                y: footprint[2][1]
            },
            Coord {
                x: footprint[3][0],
                y: footprint[3][1]
            },
            Coord {
                x: footprint[0][0],
                y: footprint[0][1]
            },
        ];
        poly
    };

    let est_polygon = get_polygon(estimated_object);
    let gt_polygon = get_polygon(ground_truth_object);

    est_polygon.intersection(&gt_polygon).unsigned_area()
}

fn get_intersection_height(
    estimated_object: &DynamicObject,
    ground_truth_object: &DynamicObject,
) -> f64 {
    let min_z = {
        [
            estimated_object.position[2] - estimated_object.size[2] * 0.5,
            ground_truth_object.position[2] - ground_truth_object.size[2] * 0.5,
        ]
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
    };

    let max_z = {
        [
            estimated_object.position[2] - estimated_object.size[2] * 0.5,
            ground_truth_object.position[2] - ground_truth_object.size[2] * 0.5,
        ]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
    };

    [0.0, max_z - min_z]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn get_intersection_volume(
    estimated_object: &DynamicObject,
    ground_truth_object: &DynamicObject,
) -> f64 {
    get_intersection_area(estimated_object, ground_truth_object)
        * get_intersection_height(estimated_object, ground_truth_object)
}
