use nalgebra::SMatrix;

use crate::{
    frame_id::FrameID,
    label::Label,
    point::{distance_points, distance_points_bev},
};

pub type RotationMatrix<T> = SMatrix<T, 3, 3>;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectState {
    position: [f64; 3],
    orientation: [f64; 4],
    size: [f64; 3],
    velocity: Option<[f64; 3]>,
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
    pub uuid: Option<String>,
}

impl DynamicObject {
    pub fn state(&self) -> ObjectState {
        ObjectState {
            position: self.position,
            orientation: self.orientation,
            size: self.size,
            velocity: self.velocity,
        }
    }

    pub fn label_name(&self) -> String {
        self.label.to_string()
    }

    pub fn area(&self) -> f64 {
        self.size[0] * self.size[1]
    }

    pub fn volume(&self) -> f64 {
        self.area() * self.size[2]
    }

    pub fn distance(&self, other: &DynamicObject) -> f64 {
        distance_points(&self.position, &other.position)
    }

    pub fn distance_bev(&self, other: &DynamicObject) -> f64 {
        distance_points_bev(&self.position, &other.position)
    }

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
