use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;
use thiserror::Error as ThisError;

pub type FrameIdResult<T> = Result<T, FrameIdError>;

/// Errors that can occur while constructing `FrameID` instance.
#[derive(Debug, ThisError)]
pub enum FrameIdError {
    #[error("internal error")]
    InternalError,
    #[error("value error: {0}")]
    ValueError(String),
}

/// Represents type of sensor frames.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameID {
    // 3D
    BaseLink,
    Map,

    // 2D
    CamBack,
    CamBackLeft,
    CamBackRight,
    CamFront,
    CamFrontLeft,
    CamFrontRight,
    CamTrafficLightNear,
    CamTrafficLightFar,
}

impl Display for FrameID {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{:?}", self)
    }
}

impl FromStr for FrameID {
    type Err = FrameIdError;

    fn from_str(input: &str) -> FrameIdResult<Self> {
        match input {
            "BaseLink" | "base_link" => Ok(FrameID::BaseLink),
            "Map" | "map" => Ok(FrameID::Map),
            "CamBack" | "cam_back" => Ok(FrameID::CamBack),
            "CamBackLeft" | "cam_back_left" => Ok(FrameID::CamBackLeft),
            "CamBackRight" | "cam_back_right" => Ok(FrameID::CamBackRight),
            "CamFront" | "cam_front" => Ok(FrameID::CamFront),
            "CamFrontLeft" | "cam_front_left" => Ok(FrameID::CamFrontLeft),
            "CamFrontRight" | "cam_front_right" => Ok(FrameID::CamFrontRight),
            "CamTrafficLightNear" | "cam_traffic_light_near" => Ok(FrameID::CamTrafficLightNear),
            "CamTrafficLightFar" | "cam_traffic_light_far" => Ok(FrameID::CamTrafficLightFar),
            _ => Err(FrameIdError::ValueError(input.to_string())),
        }
    }
}
