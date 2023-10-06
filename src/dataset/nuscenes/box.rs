use crate::utils::math::{rotate, rotate_inv, rotate_q, rotate_q_inv, translate, translate_inv};

use super::schema::LongToken;

#[derive(Debug)]
pub struct NuScenesBox {
    pub position: [f64; 3],
    pub orientation: [f64; 4],
    pub size: [f64; 3],
    pub name: String,
    pub instance: LongToken,
    pub num_lidar_pts: usize,
    pub token: LongToken,
}

impl NuScenesBox {
    /// Translates own position with input vector.
    /// This method is the destructive operation.
    ///
    /// * `xyz` - A translation 3d-vector, ordering (x, y, z).
    pub fn translate(&mut self, xyz: &[f64; 3]) {
        self.position = translate(&self.position, xyz);
    }

    /// Returns the translated position with input vector.
    /// This method is the nondestructive operation.
    ///
    /// * `xyz` - A translation 3d-vector, ordering (x, y, z).
    pub fn get_translate(&self, xyz: &[f64; 3]) -> [f64; 3] {
        translate(&self.position, xyz)
    }

    /// Inverse-translates own position with input vector.
    /// This method is the destructive operation.
    ///
    /// * `xyz` - A translation 3d-vector, ordering (x, y, z).
    pub fn translate_inv(&mut self, xyz: &[f64; 3]) {
        self.position = translate_inv(&self.position, xyz);
    }

    /// Returns the inverse-translated position with input vector.
    ///
    /// * `xyz` - A translation 3d-vector, ordering (x, y, z).
    pub fn get_translate_inv(&self, xyz: &[f64; 3]) -> [f64; 3] {
        translate_inv(&self.position, xyz)
    }

    pub fn rotate(&mut self, orientation: &[f64; 4]) {
        self.position = rotate(&self.position, orientation);
        self.orientation = rotate_q(&self.orientation, orientation);
    }

    pub fn rotate_inv(&mut self, orientation: &[f64; 4]) {
        self.position = rotate_inv(&self.position, orientation);
        self.orientation = rotate_q_inv(&self.orientation, orientation);
    }
}
