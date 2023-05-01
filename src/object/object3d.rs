use crate::label::Label;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectState {
    position: [f64; 3],
    orientation: [f64; 4],
    size: [f64; 3],
    velocity: Option<[f64; 3]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicObject {
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
}
