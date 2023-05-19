use chrono::NaiveDateTime;
use nalgebra::SMatrix;

use crate::{
    frame_id::FrameID,
    label::Label,
    math::{quaternion2euler, quaternion2rotation, PositionMatrix, RotationMatrix},
    point::{distance_points, distance_points_bev},
};
use std::{
    f64::consts::PI,
    fmt::{Display, Formatter, Result as FormatResult},
};

pub(crate) type CornerMatrix = SMatrix<f64, 4, 3>;

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

#[derive(Debug, Clone)]
pub struct DynamicObject {
    pub timestamp: NaiveDateTime,
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

impl PartialEq for DynamicObject {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.frame_id == other.frame_id
            && self.position == other.position
            && self.orientation == other.orientation
            && self.label == other.label
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
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
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let name = object.label_name();
    ///
    /// assert_eq!(name, "Car");
    /// ```
    pub fn label_name(&self) -> String {
        self.label.to_string()
    }

    /// Returns area of box in BEV.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let area = object.area();
    ///
    /// assert_eq!(area, 2.0);
    /// ```
    pub fn area(&self) -> f64 {
        self.size[0] * self.size[1]
    }

    /// Returns volume of box.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let volume = object.volume();
    ///
    /// assert_eq!(volume, 2.0);
    /// ```
    pub fn volume(&self) -> f64 {
        self.area() * self.size[2]
    }

    /// Returns distance from origin where the object is with respect to.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let distance = object.distance();
    ///
    /// assert_eq!(distance, 2.0_f64.sqrt());
    /// ```
    pub fn distance(&self) -> f64 {
        distance_points(&self.position, &[0.0; 3])
    }

    /// Returns distance in BEV from origin where the object is with respect to.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let distance_bev = object.distance_bev();
    ///
    /// assert_eq!(distance_bev, 2.0_f64.sqrt());
    /// ```
    pub fn distance_bev(&self) -> f64 {
        distance_points_bev(&self.position, &[0.0; 3])
    }

    /// Returns distance from the other point.
    ///
    /// * `point`   - 3D coordinates position.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let distance = object.distance_from(&[1.0, 1.0, 1.0]);
    ///
    /// assert_eq!(distance, 1.0);
    /// ```
    pub fn distance_from(&self, point: &[f64; 3]) -> f64 {
        distance_points(&self.position, point)
    }

    /// Returns distance in BEV from the other point.
    ///
    /// * `point`   - 3D coordinates position.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let distance_bev = object.distance_bev_from(&[1.0, 1.0, 1.0]);
    ///
    /// assert_eq!(distance_bev, 0.0);
    /// ```
    pub fn distance_bev_from(&self, point: &[f64; 3]) -> f64 {
        distance_points_bev(&self.position, point)
    }

    /// Returns object's heading angle.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let heading = object.heading();
    ///
    /// assert_eq!(heading, 0.0);
    /// ```
    pub fn heading(&self) -> f64 {
        let [_, _, yaw] = self.euler();

        // yaw = -yaw - 0.5 * PI;

        if PI < yaw {
            yaw - 2.0 * PI
        } else if yaw < -PI {
            yaw + 2.0 * PI
        } else {
            yaw
        }
    }

    /// Returns 3x3 rotation matrix.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, math::RotationMatrix, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let rot = object.rotation_matrix();
    ///
    /// let eye = RotationMatrix::new(
    ///     1.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0,
    ///     0.0, 0.0, 1.0
    /// );
    ///
    /// assert_eq!(rot, eye);
    /// ```
    pub fn rotation_matrix(&self) -> RotationMatrix<f64> {
        quaternion2rotation(&self.orientation)
    }

    /// Returns euler angles in [roll, pitch yaw] order.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 1.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let euler = object.euler();
    ///
    /// assert_eq!(euler, [0.0, 0.0, 0.0]);
    /// ```
    pub fn euler(&self) -> [f64; 3] {
        quaternion2euler(&self.orientation)
    }

    /// Returns footprint of object's box.
    ///
    /// # Examples
    /// ```
    /// use chrono::NaiveDateTime;
    /// use perception_eval::{frame_id::FrameID, label::Label, object::object3d::DynamicObject};
    ///
    /// let object = DynamicObject {
    ///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
    ///     frame_id: FrameID::BaseLink,
    ///     position: [1.0, 1.0, 0.0],
    ///     orientation: [1.0, 0.0, 0.0, 0.0],
    ///     size: [2.0, 2.0, 1.0],
    ///     velocity: None,
    ///     confidence: 1.0,
    ///     label: Label::Car,
    ///     pointcloud_num: Some(1000),
    ///     uuid: Some("111".to_string()),
    /// };
    ///
    /// let footprint = object.footprint();
    ///
    /// assert_eq!(footprint[0], [2.0, 2.0, 0.0]);
    /// assert_eq!(footprint[1], [0.0, 2.0, 0.0]);
    /// assert_eq!(footprint[2], [0.0, 0.0, 0.0]);
    /// assert_eq!(footprint[3], [2.0, 0.0, 0.0])
    /// ```
    pub fn footprint(&self) -> Vec<[f64; 3]> {
        let center2corners = CornerMatrix::new(
            self.size[1] * 0.5,
            self.size[0] * 0.5,
            0.0,
            -self.size[1] * 0.5,
            self.size[0] * 0.5,
            0.0,
            -self.size[1] * 0.5,
            -self.size[0] * 0.5,
            0.0,
            self.size[1] * 0.5,
            -self.size[0] * 0.5,
            0.0,
        );

        let rot = self.rotation_matrix();
        let position: SMatrix<f64, 1, 3> =
            PositionMatrix::new(self.position[0], self.position[1], self.position[2]);

        center2corners
            .row_iter()
            .map(|corner| {
                let mat = corner * rot + position;
                [mat[(0, 0)], mat[(0, 1)], mat[(0, 2)]]
            })
            .collect()
    }
}
