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
                pairs.insert("car", Label::Car);
                pairs.insert("truck", Label::Truck);
                pairs.insert("bus", Label::Bus);
                pairs.insert("bicycle", Label::Bicycle);
                pairs.insert("motorbike", Label::Motorbike);
                pairs.insert("pedestrian", Label::Pedestrian);
                pairs.insert("animal", Label::Animal);
                pairs.insert("unknown", Label::Unknown);
            }
            _ => Err(LabelError::ValueError(prefix.to_string()))?,
        }
        let ret = Self { pairs: pairs };
        Ok(ret)
    }

    pub fn convert(&self, name: &str) -> LabelResult<Label> {
        match self.pairs.contains_key(name) {
            true => Ok(self.pairs[name].clone()),
            false => Err(LabelError::ValueError(name.to_string())),
        }
    }
}

pub fn convert_labels(
    target_labels: Vec<&str>,
    converter: &LabelConverter,
) -> LabelResult<Vec<Label>> {
    let mut ret = Vec::new();
    for name in target_labels {
        let label = converter.convert(name)?;
        ret.push(label);
    }
    Ok(ret)
}
