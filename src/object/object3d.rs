use nalgebra::SMatrix;

use crate::{
    frame_id::FrameID,
    label::Label,
    point::{distance_points, distance_points_bev},
};
use std::fmt::{Display, Formatter, Result as FormatResult};

pub type RotationMatrix<T> = SMatrix<T, 3, 3>;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectState {
    position: [f64; 3],
    orientation: [f64; 4],
    size: [f64; 3],
    velocity: Option<[f64; 3]>,
}

impl Display for ObjectState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f,
            "position: {:?}\norientation: {:?}\nsize: {:?}\nvelocity: {:?}",
            self.position, self.orientation, self.size, self.velocity
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicObject {
    pub frame_id: FrameID,
    pub position: [f64; 3],
    pub orientation: [f64; 4],
    pub size: [f64; 3],
    pub velocity: Option<[f64; 3]>,
    pub confidence: f64,
    pub label: Label,
    pub pointcloud_num: Option<isize>,
    pub uuid: Option<String>,
}

impl Display for DynamicObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f, 
            "frame_id: {:?}\nposition: {:?}\norientation: {:?}\nsize: {:?}\nvelocity: {:?}\nconfidence: {}\nlabel: {:?}\nuuid: {:?}", 
            self.frame_id, 
            self.position, 
            self.orientation,
            self.size, 
            self.velocity, 
            self.confidence, 
            self.label, 
            self.uuid
        )
    }
}

impl DynamicObject {
    /// Returns `ObjectState` instance.
    pub fn state(&self) -> ObjectState {
        ObjectState {
            position: self.position,
            orientation: self.orientation,
            size: self.size,
            velocity: self.velocity,
        }
    }

    /// Returns name of label in string.
    pub fn label_name(&self) -> String {
        self.label.to_string()
    }

    /// Returns area of box in BEV.
    pub fn area(&self) -> f64 {
        self.size[0] * self.size[1]
    }

    /// Returns volume of box.
    pub fn volume(&self) -> f64 {
        self.area() * self.size[2]
    }

    /// Returns distance from origin where the object is with respect to.
    pub fn distance(&self) -> f64 {
        distance_points(&self.position, &[0.0; 3])
    }

    /// Returns distance in BEV from origin where the object is with respect to.
    pub fn distance_bev(&self) -> f64 {
        distance_points_bev(&self.position, &[0.0; 3])
    }

    /// Returns distance from the other point.
    /// 
    /// * `point`   - 3D coordinates position.
    pub fn distance_from(&self, point: &[f64; 3]) -> f64 {
        distance_points(&self.position, point)
    }

    /// Returns distance in BEV from the other point.
    /// 
    /// * `point`   - 3D coordinates position.
    pub fn distance_bev_from(&self, point: &[f64; 3]) -> f64 {
        distance_points_bev(&self.position, point)
    }

    /// Returns 3x3 rotation matrix.
    pub fn rotation_matrix(&self) -> RotationMatrix<f64> {
        let [q0, q1, q2, q3] = self.orientation;
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

    /// Returns footprint of object's box.
    pub fn footprint(&self) -> Vec<[f64; 3]> {
        let mut center2corners = Vec::new();
        center2corners.push([self.size[1] * 0.5, self.size[0] * 0.5, 0.0]);
        center2corners.push([-self.size[1] * 0.5, self.size[0] * 0.5, 0.0]);
        center2corners.push([-self.size[1] * 0.5, -self.size[0] * 0.5, 0.0]);
        center2corners.push([self.size[1] * 0.5, -self.size[0] * 0.5, 0.0]);

        let mut footprint = Vec::new();
        let rot = self.rotation_matrix();
        for corner in center2corners {}

        footprint
    }
}
