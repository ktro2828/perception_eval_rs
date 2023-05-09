use crate::math::rotate;

#[derive(Debug)]
pub(crate) struct NuScenesBox {
    pub(crate) position: [f64; 3],
    pub(crate) orientation: [f64; 4],
    pub(crate) size: [f64; 3],
    pub(crate) label: String,
}

impl NuScenesBox {
    pub(crate) fn translate(&mut self, xyz: &[f64; 3]) {
        for i in 0..3 {
            self.position[i] += xyz[i];
        }
    }

    pub(crate) fn rotate(&mut self, orientation: &[f64; 4]) {
        self.position = rotate(&self.position, &orientation);

        for i in 0..4 {
            self.orientation[i] *= orientation[i];
        }
    }
}
