use std::f64::consts::PI;

use nalgebra::SMatrix;
pub type RotationMatrix<T> = SMatrix<T, 3, 3>;
pub(crate) type PositionMatrix = SMatrix<f64, 1, 3>;

/// Convert quaternion into 3x3 rotation matrix.
///
/// * `q`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::RotationMatrix;
/// use perception_eval::math::quaternion2rotation;
///
/// let q = [1.0, 0.0, 0.0, 0.0];
/// let rot = quaternion2rotation(&q);
///
/// let ans = RotationMatrix::new(
///     1.0, 0.0, 0.0,
///     0.0, 1.0, 0.0,
///     0.0, 0.0, 1.0,
/// );
/// assert_eq!(rot, ans);
/// ```
pub fn quaternion2rotation(q: &[f64; 4]) -> RotationMatrix<f64> {
    let [q0, q1, q2, q3] = q;
    RotationMatrix::new(
        2.0 * (q0.powi(2) + q1.powi(2)) - 1.0,
        2.0 * (q1 * q2 - q0 * q3),
        2.0 * (q1 * q3 + q0 * q2),
        2.0 * (q1 * q2 + q0 * q3),
        2.0 * (q0.powi(2) + q2.powi(2)) - 1.0,
        2.0 * (q2 * q3 - q0 * q1),
        2.0 * (q1 * q3 - q0 * q2),
        2.0 * (q2 * q3 + q0 * q1),
        2.0 * (q0.powi(2) + q3.powi(2)) - 1.0,
    )
}

/// Convert quaternion into euler angle, [roll, pitch, yaw] order.
///
/// * `q`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::quaternion2euler;
///
/// let q = [1.0, 0.0, 0.0, 0.0];
/// let euler = quaternion2euler(&q);
///
/// assert_eq!(euler, [0.0, 0.0, 0.0]);
/// ```
pub fn quaternion2euler(q: &[f64; 4]) -> [f64; 3] {
    let [q0, q1, q2, q3] = q;
    let roll = (2.0 * (q0 * q1 + q2 * q3) / (1.0 - 2.0 * (q1.powi(2) + q2.powi(2)))).atan();
    let pitch = -0.5 * PI
        + 2.0
            * ((1.0 + 2.0 * (q0 * q2 - q1 * q3)) / (1.0 - 2.0 * (q0 * q2 - q1 * q3)))
                .sqrt()
                .atan();
    let yaw = (2.0 * (q0 * q3 + q1 * q2) / (1.0 - 2.0 * (q2.powi(2) + q3.powi(2)))).atan();
    [roll, pitch, yaw]
}

/// Returns inverse quaternion.
///
/// * `q`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::inverse_quaternion;
///
/// let q = [1.0, 0.0, 0.0, 0.0];
/// let q_inv = inverse_quaternion(&q);
///
/// assert_eq!(q_inv, [1.0, 0.0, 0.0, 0.0]);
/// ```
pub fn inverse_quaternion(q: &[f64; 4]) -> [f64; 4] {
    let q_norm = q.iter().map(|e| e.powi(2)).sum::<f64>();
    [
        q[0] / q_norm,
        -q[1] / q_norm,
        -q[2] / q_norm,
        -q[3] / q_norm,
    ]
}

/// Positive translate `xyz1` with  `xyz2`.
///
/// * `xyz1`    - 3D position.
/// * `xyz2`    - 3D position.
///
/// # Examples
/// ```
/// use perception_eval::math::translate;
///
/// let xyz1 = [1.0, 1.0, 1.0];
/// let xyz2 = [2.0, 2.0, 2.0];
///
/// let ret = translate(&xyz1, &xyz2);
/// assert_eq!(ret, [3.0, 3.0, 3.0]);
/// ```
pub fn translate(xyz1: &[f64; 3], xyz2: &[f64; 3]) -> [f64; 3] {
    let mut ret = xyz1.to_owned();
    for i in 0..3 {
        ret[i] += xyz2[i];
    }
    ret
}

/// Negative translate `xyz1` with  `xyz2`.
///
/// * `xyz1`    - 3D position.
/// * `xyz2`    - 3D position.
///
/// # Examples
/// ```
/// use perception_eval::math::translate_inv;
///
/// let xyz1 = [1.0, 1.0, 1.0];
/// let xyz2 = [2.0, 2.0, 2.0];
///
/// let ret = translate_inv(&xyz1, &xyz2);
/// assert_eq!(ret, [-1.0, -1.0, -1.0]);
/// ```
pub fn translate_inv(xyz1: &[f64; 3], xyz2: &[f64; 3]) -> [f64; 3] {
    let mut ret = xyz1.to_owned();
    for i in 0..3 {
        ret[i] -= xyz2[i];
    }
    ret
}

/// Rotate `xyz` with input quaternion `q`.
///
/// * `xyz` - 3D position.
/// * `q`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::rotate;
///
/// let xyz = [1.0, 1.0, 1.0];
/// let q = [1.0, 0.0, 0.0, 0.0];
///
/// let ret = rotate(&xyz, &q);
///
/// assert_eq!(ret, [1.0, 1.0, 1.0]);
/// ```
pub fn rotate(xyz: &[f64; 3], q: &[f64; 4]) -> [f64; 3] {
    let rot = quaternion2rotation(q);
    let position = PositionMatrix::new(xyz[0], xyz[1], xyz[2]) * rot;
    let row = position.row(0);
    [row[0], row[1], row[2]]
}

/// Inverse rotate `xyz` with input quaternion `q`.
///
/// * `xyz` - 3D position.
/// * `q`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::rotate_inv;
///
/// let xyz = [1.0, 1.0, 1.0];
/// let q = [1.0, 0.0, 0.0, 0.0];
///
/// let ret = rotate_inv(&xyz, &q);
///
/// assert_eq!(ret, [1.0, 1.0, 1.0]);
/// ```
pub fn rotate_inv(xyz: &[f64; 3], q: &[f64; 4]) -> [f64; 3] {
    let q_inv = inverse_quaternion(q);
    rotate(xyz, &q_inv)
}

/// Rotate `q1` with input `q2`.
///
/// * `q1`   - Quaternion, [w, x, y, z] order.
/// * `q2`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::rotate_q;
///
/// let q1 = [1.0, 0.0, 0.0, 0.0];
/// let q2 = [1.0, 0.0, 0.0, 0.0];
///
/// let ret = rotate_q(&q1, &q2);
///
/// assert_eq!(ret, [1.0, 0.0, 0.0, 0.0]);
/// ```
pub fn rotate_q(q1: &[f64; 4], q2: &[f64; 4]) -> [f64; 4] {
    let mut ret = q1.to_owned();
    for i in 0..4 {
        ret[i] *= q2[i];
    }
    ret
}

/// Inverse rotate `q1` with input `q2`.
///
/// * `q1`   - Quaternion, [w, x, y, z] order.
/// * `q2`   - Quaternion, [w, x, y, z] order.
///
/// # Examples
/// ```
/// use perception_eval::math::rotate_q_inv;
///
/// let q1 = [1.0, 0.0, 0.0, 0.0];
/// let q2 = [1.0, 0.0, 0.0, 0.0];
///
/// let ret = rotate_q_inv(&q1, &q2);
///
/// assert_eq!(ret, [1.0, 0.0, 0.0, 0.0]);
/// ```
pub fn rotate_q_inv(q1: &[f64; 4], q2: &[f64; 4]) -> [f64; 4] {
    let q2_inv = inverse_quaternion(q2);
    rotate_q(q1, &q2_inv)
}
