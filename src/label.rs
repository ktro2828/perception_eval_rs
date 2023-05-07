use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
};

use thiserror::Error as ThisError;

pub type LabelResult<T> = Result<T, LabelError>;

#[derive(Debug, ThisError)]
pub enum LabelError {
    #[error("internal error")]
    InternalError,
    #[error("value error: {0}")]
    ValueError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    Unknown,
    Car,
    Truck,
    Bus,
    Bicycle,
    Motorbike,
    Pedestrian,
    Animal,
}

impl Display for Label {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct LabelConverter<'a> {
    pairs: HashMap<&'a str, Label>,
}

impl<'a> LabelConverter<'a> {
    pub fn new(label_prefix: Option<&str>) -> LabelResult<Self> {
        let mut pairs = HashMap::new();

        let prefix = {
            match label_prefix {
                Some(value) => value,
                None => "autoware",
            }
        };

        match prefix {
            "autoware" => {
                // car
                pairs.insert("car", Label::Car);
                pairs.insert("vehicle.car", Label::Car);
                pairs.insert("vehicle.construction", Label::Car);
                pairs.insert("vehicle.emergency (ambulance & police)", Label::Car);
                pairs.insert("vehicle.police", Label::Car);
                pairs.insert("vehicle.fire", Label::Car);
                pairs.insert("vehicle.ambulance", Label::Car);
                // truck
                pairs.insert("truck", Label::Truck);
                pairs.insert("vehicle.truck", Label::Truck);
                pairs.insert("trailer", Label::Truck);
                pairs.insert("vehicle.trailer", Label::Truck);
                // bus
                pairs.insert("bus", Label::Bus);
                pairs.insert("vehicle.bus", Label::Bus);
                pairs.insert("vehicle.bus (bendy & rigid)", Label::Bus);
                pairs.insert("vehicle.bus.rigid", Label::Bus);
                pairs.insert("vehicle.bus.bendy", Label::Bus);
                // bicycle
                pairs.insert("bicycle", Label::Bicycle);
                pairs.insert("vehicle.bicycle", Label::Bicycle);
                // motorbike
                pairs.insert("motorbike", Label::Motorbike);
                pairs.insert("vehicle.motorcycle", Label::Motorbike);
                // pedestrian
                pairs.insert("pedestrian", Label::Pedestrian);
                pairs.insert("pedestrian.adult", Label::Pedestrian);
                pairs.insert("pedestrian.child", Label::Pedestrian);
                pairs.insert("pedestrian.construction_worker", Label::Pedestrian);
                pairs.insert("pedestrian.personal_mobility", Label::Pedestrian);
                pairs.insert("pedestrian.police_officer", Label::Pedestrian);
                pairs.insert("pedestrian.stroller", Label::Pedestrian);
                pairs.insert("pedestrian.wheelchair", Label::Pedestrian);
                pairs.insert("human.pedestrian.adult", Label::Pedestrian);
                pairs.insert("human.pedestrian.child", Label::Pedestrian);
                pairs.insert("human.pedestrian.construction_worker", Label::Pedestrian);
                pairs.insert("human.pedestrian.personal_mobility", Label::Pedestrian);
                pairs.insert("human.pedestrian.police_officer", Label::Pedestrian);
                pairs.insert("human.pedestrian.stroller", Label::Pedestrian);
                pairs.insert("human.pedestrian.wheelchair", Label::Pedestrian);
                // animal
                pairs.insert("animal", Label::Animal);
                // unknown
                pairs.insert("unknown", Label::Unknown);
                pairs.insert("movable_object.barrier", Label::Unknown);
                pairs.insert("movable_object.debris", Label::Unknown);
                pairs.insert("movable_object.pushable_pullable", Label::Unknown);
                pairs.insert("movable_object.trafficcone", Label::Unknown);
                pairs.insert("movable_object.traffic_cone", Label::Unknown);
                pairs.insert("static_object.bicycle_rack", Label::Unknown);
                pairs.insert("static_object.bollard", Label::Unknown);
            }
            _ => Err(LabelError::ValueError(prefix.to_string()))?,
        }
        let ret = Self { pairs: pairs };
        Ok(ret)
    }

    pub fn convert(&self, name: &str) -> Label {
        match self.pairs.contains_key(name) {
            true => self.pairs[name].clone(),
            false => {
                log::warn!("unexpected label name: {}, set as Label::Unknown", name);
                Label::Unknown
            }
        }
    }
}

pub fn convert_labels(
    target_labels: &Vec<&str>,
    converter: &LabelConverter,
) -> LabelResult<Vec<Label>> {
    let mut ret = Vec::new();
    for name in target_labels {
        let label = converter.convert(name);
        ret.push(label);
    }
    Ok(ret)
}
