use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{evaluation_task::EvaluationTask, frame_id::FrameID};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Scenario {
    #[serde(rename = "ScenarioFormatVersion")]
    pub(super) version: String,
    #[serde(rename = "ScenarioName")]
    pub(super) name: String,
    #[serde(rename = "ScenarioDescription")]
    pub(super) description: String,
    #[serde(rename = "SensorModel")]
    pub(super) sensor_model: String,
    #[serde(rename = "VehicleModel")]
    pub(super) vehicle_model: String,
    #[serde(rename = "Evaluation")]
    pub(super) evaluation: Evaluation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Evaluation {
    #[serde(rename = "UseCaseName")]
    pub(super) usecase: UseCase,
    #[serde(rename = "UseCaseFormatVersion")]
    pub(super) version: String,
    #[serde(rename = "Datasets")]
    pub(super) datasets: Vec<HashMap<String, Dataset>>,
    #[serde(rename = "Conditions")]
    pub(super) conditions: Conditions,
    #[serde(rename = "PerceptionEvaluationConfig")]
    pub(super) config: EvaluationConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) enum UseCase {
    #[serde(rename = "perception")]
    Perception,
    #[serde(rename = "perception_2d")]
    Perception2D,
    #[serde(rename = "traffic_light")]
    TrafficLight,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Dataset {
    #[serde(rename = "Version")]
    pub(super) version: String,
    #[serde(rename = "VehicleId")]
    pub(super) vehicle_id: String,
    #[serde(rename = "LaunchSensing")]
    pub(super) launch_sensing: bool,
    #[serde(rename = "LocalMapPath")]
    pub(super) local_map_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Conditions {
    #[serde(rename = "PassRate")]
    pub(super) pass_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct EvaluationConfig {
    #[serde(rename = "evaluation_config_dict")]
    pub(super) params: ConfigParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct ConfigParams {
    #[serde(with = "evaluation_task_serde")]
    pub(super) evaluation_task: EvaluationTask,
    #[serde(with = "frame_id_serde")]
    pub(super) frame_id: FrameID,
    pub(super) target_labels: Vec<String>,
    pub(super) max_x_position: f64,
    pub(super) max_y_position: f64,
    pub(super) min_point_number: Option<usize>,
    pub(super) target_uuids: Option<Vec<String>>,
    pub(super) center_distance_threshold: f64,
    pub(super) plane_distance_threshold: f64,
    pub(super) iou_2d_threshold: f64,
    pub(super) iou_3d_threshold: f64,
}

mod evaluation_task_serde {
    use std::str::FromStr;

    use crate::evaluation_task::EvaluationTask;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &EvaluationTask, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let task_str = value.to_string();
        serializer.serialize_str(&task_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<EvaluationTask, D::Error>
    where
        D: Deserializer<'de>,
    {
        let task_str = String::deserialize(deserializer)?;
        let task = EvaluationTask::from_str(&task_str).unwrap(); // TODO: update result type
        Ok(task)
    }
}

mod frame_id_serde {
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serializer};

    use crate::frame_id::FrameID;

    pub fn serialize<S>(value: &FrameID, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let frame_id_str = value.to_string();
        serializer.serialize_str(&frame_id_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FrameID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let frame_id_str = String::deserialize(deserializer)?;
        let frame_id = FrameID::from_str(&frame_id_str).unwrap(); // TODO
        Ok(frame_id)
    }
}
