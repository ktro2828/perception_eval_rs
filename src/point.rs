pub fn distance_points(point1: [f64; 3], point2: [f64; 3]) -> f64 {
    let mut sum = 0.0;
    for i in 0..3 {
        sum += (point1[i] - point2[i]).powi(2);
    }
    sum.powf(0.5)
}

pub fn distance_points_bev(point1: [f64; 3], point2: [f64; 3]) -> f64 {
    let mut sum = 0.0;
    for i in 0..2 {
        sum += (point1[i] - point2[i]).powi(2);
    }
    sum.powf(0.5)
}

pub fn get_point_left_right(point1: [f64; 3], point2: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let cross_product = point1[0] * point2[1] - point1[1] * point2[0];
    if cross_product < 0.0 {
        (point1, point2)
    } else {
        (point2, point1)
    }
}
