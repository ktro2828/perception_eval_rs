/// Calculate euclidean distance between two points.
///
/// * `point1`  - 3D coordinates point.
/// * `point2`  - 3D coordinates point.
pub fn distance_points(point1: &[f64; 3], point2: &[f64; 3]) -> f64 {
    assert!(point1.len() == point2.len());
    point1
        .iter()
        .zip(point2.iter())
        .fold(0.0, |sum, (p1, p2)| sum + (p1 - p2).powi(2))
        .sqrt()
}

/// Calculate euclidean distance in BEV between two points.
///
/// * `point1`  - 3D coordinates point.
/// * `point2`  - 3D coordinates point.
pub fn distance_points_bev(point1: &[f64; 3], point2: &[f64; 3]) -> f64 {
    assert!(point1.len() == point2.len());
    let pt1_iter = point1[..2].iter();
    let pt2_iter = point2[..2].iter();
    pt1_iter
        .zip(pt2_iter)
        .fold(0.0, |sum, (p1, p2)| sum + (p1 - p2).powi(2))
        .sqrt()
}

/// Determine which one is left and right side with cross product.
/// Returns input points (left, right) order.
///
/// * `point1`  - 3D coordinates point.
/// * `point2`  - 3D coordinates point.
pub fn get_point_left_right<'a>(
    point1: &'a [f64; 3],
    point2: &'a [f64; 3],
) -> (&'a [f64; 3], &'a [f64; 3]) {
    let cross_product = point1[0] * point2[1] - point1[1] * point2[0];
    if cross_product < 0.0 {
        (point1, point2)
    } else {
        (point2, point1)
    }
}
