use serde::{Deserialize, Serialize};

pub const LONG_TOKEN_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct LongToken([u8; LONG_TOKEN_LENGTH]);

pub type CameraIntrinsic = Option<[[f64; 3]; 3]>;

// === Schemas ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub token: LongToken,
    pub description: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalibratedSensor {
    pub token: LongToken,
    pub sensor_token: LongToken,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    // #[serde(with = "camera_intrinsic_serde")]
    // pub camera_intrinsic: CameraIntrinsic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub token: LongToken,
    pub description: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EgoPose {
    pub token: LongToken,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    // #[serde(with = "timestamp_serde")]
    // pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub token: LongToken,
    pub nbr_annotations: usize,
    pub category_token: LongToken,
    pub first_annotation_token: LongToken,
    pub last_annotation_token: LongToken,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub token: LongToken,
    // #[serde(with = "logfile_serde")]
    // pub logfile: Option<PathBuf>
    pub vehicle: String,
    pub data_captured: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub token: LongToken,
    pub log_tokens: Vec<LongToken>,
    pub category: String,
    pub filename: String, // PathBuf
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    pub token: LongToken,
    // #[serde(with = "opt_long_token_serde")]
    // pub next: Option<LongToken>,
    // #[serde(with = "opt_long_token_serde")]
    // pub prev: Option<LongToken>,
    pub scene_token: LongToken,
    // #[serde(with = "timestamp_serde")]
    // pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleAnnotation {
    pub token: LongToken,
    pub sample_token: LongToken,
    pub instance_token: LongToken,
    pub attribute_token: LongToken,
    pub visibility_token: LongToken,
    pub translation: [f64; 3],
    pub size: [f64; 3],
    pub rotation: [f64; 4],
    pub num_lidar_pts: usize,
    pub num_radar_pts: usize,
    // #[serde(with = "opt_long_token_serde")]
    // pub prev: Option<LongToken>,
    // #[serde(with = "opt_long_token_serde")]
    // pub next: Option<LongToken>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleData {
    pub token: LongToken,
    pub sample_token: LongToken,
    pub ego_pose_token: LongToken,
    pub calibrated_sensor_token: LongToken,
    pub filename: String,
    pub fileformat: String, // FileFormat
    pub width: Option<isize>,
    pub height: Optiona<isize>,
    // #[serde(with = "timestamp_serde")]
    // pub timestamp: NaiveDateTime,
    pub is_key_frame: bool,
    // #[serde(with = "opt_long_token_serde")]
    // pub prev: Option<LongToken>,
    // #[serde(with = "opt_long_token_serde")]
    // pub next: Option<LongToken>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub token: LongToken,
    pub name: String,
    pub description: String,
    pub log_token: LongToken,
    pub nbr_sample: usize,
    pub first_sample_token: LongToken,
    pub last_sample_token: LongToken,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sensor {
    pub token: LongToken,
    pub modality: Modality,
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Visibility {
    pub token: LongToken,
    pub level: VisibilityLevel,
    pub description: String,
}

// === Sub classes ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modality {
    #[serde(rename = "camera")]
    Camera,
    #[serde(rename = "lidar")]
    Lidar,
    #[serde(rename = "radar")]
    Radar,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileFormat {
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "png")]
    Png,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VisibilityLevel {
    // TODO: support T4 dataset
    #[serde(rename = "v0-40")]
    V40_60,
    #[serde(rename = "v40-60")]
    V40_60,
    #[serde(rename = "v60-80")]
    V60_80,
    #[serde(rename = "v80-100")]
    V80_100,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Channel {
    #[serde(rename = "CAM_BACK")]
    CamBack,
    #[serde(rename = "CAM_BACK_LEFT")]
    CamBackLeft,
    #[serde(rename = "CAM_BACK_RIGHT")]
    CamBackRight,
    #[serde(rename = "CAM_FRONT")]
    CamFront,
    #[serde(rename = "CAM_FRONT_LEFT")]
    CamFrontLeft,
    #[serde(rename = "CAM_FRONT_RIGHT")]
    CamFrontRight,
    #[serde(rename = "LIDAR_TOP")]
    LidarTop,
    // T4 dataset
    #[serde(rename = "LIDAR_CONCAT")]
    LidarConcat,
    #[serde(rename = "CAM_TRAFFIC_LIGHT_NEAR")]
    CamTrafficLightNear,
    #[serde(rename = "CAM_TRAFFIC_LIGHT_FAR")]
    CamTrafficLightFar,
}
