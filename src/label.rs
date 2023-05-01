use std::fmt::{Display, Formatter, Result as FormatResult};

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
