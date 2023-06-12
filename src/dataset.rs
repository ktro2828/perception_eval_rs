pub mod nuscenes;

use self::nuscenes::schema::Modality;
use self::nuscenes::{internal::SampleInternal, NuScenes, WithDataset};
use crate::{
    evaluation_task::EvaluationTask, frame_id::FrameID, label::LabelConverter,
    object::object3d::DynamicObject,
};
use chrono::naive::NaiveDateTime;
use indicatif::{ProgressBar, ProgressIterator};
use std::path::PathBuf;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FormatResult},
};

pub type DatasetResult<T> = Result<T, Box<dyn Error>>;

/// A struct to contain ground truth objects at one frame.
///
/// * `timestamp`   - Timestamp of the frame.
/// * `objects`     - List of ground truth objects.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameGroundTruth {
    pub timestamp: NaiveDateTime,
    pub objects: Vec<DynamicObject>,
}

impl Display for FrameGroundTruth {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f,
            "timestamp: {:?}, num objects: {}",
            self.timestamp,
            self.objects.len()
        )
    }
}

/// Returns list of `FrameGroundTruth` including whole frames.
///
/// * `version`         - NuScenes version of dataset.
/// * `data_root`       - Root directory path of dataset.
/// * `evaluation_task` - Task to evaluate.
/// * `frame_id`        - Frame id where objects are with respect to.
pub fn load_dataset(
    version: &str,
    data_root: &PathBuf,
    evaluation_task: &EvaluationTask,
    frame_id: &FrameID,
) -> DatasetResult<Vec<FrameGroundTruth>> {
    log::info!(
        "config: evaluation_task: {}, frame_id: {}",
        evaluation_task,
        frame_id,
    );

    let nusc = NuScenes::load(version, data_root)?;
    let bar = ProgressBar::new(nusc.sample_map.len() as u64);
    let datasets = nusc
        .sample_iter()
        .progress_with(bar)
        .map(|sample| sample_to_frame(&nusc, &sample, frame_id))
        .collect::<DatasetResult<Vec<FrameGroundTruth>>>()?;
    Ok(datasets)
}

/// Convert NuScenes sample into `FrameGroundTruth` instance.
///
/// TODO: Transform position and rotation into BaseLin
///
/// * `nusc`        - NuScenes instance.
/// * `sample`      - Sample annotated in meta data.
/// * `frame_id`    - FrameID instance.
fn sample_to_frame(
    nusc: &NuScenes,
    sample: &WithDataset<SampleInternal>,
    frame_id: &FrameID,
) -> DatasetResult<FrameGroundTruth> {
    let mut objects: Vec<DynamicObject> = Vec::new();

    // TODO
    // === update objects container ===
    let label_converter = LabelConverter::new("autoware")?;
    for sample_data in sample.sample_data_iter() {
        let cs_record = nusc
            .calibrated_sensor_map
            .get(&sample_data.calibrated_sensor_token)
            .unwrap();
        if nusc
            .sensor_map
            .get(&cs_record.sensor_token)
            .unwrap()
            .modality
            != Modality::Lidar
            || sample_data.timestamp != sample.timestamp
        {
            continue;
        }
        let (_, boxes) = nusc.get_sample_data(&sample_data.token, &false)?;
        boxes.iter().for_each(|nusc_box| {
            let label = label_converter.convert(&nusc_box.name);
            objects.push(DynamicObject {
                timestamp: sample.timestamp.to_owned(),
                position: nusc_box.position,
                orientation: nusc_box.orientation,
                size: nusc_box.size,
                confidence: 1.0,
                label: label,
                velocity: None,
                frame_id: frame_id.to_owned(),
                pointcloud_num: Some(nusc_box.num_lidar_pts),
                uuid: Some(nusc_box.instance.to_string()),
            });
        });
    }

    let ret = FrameGroundTruth {
        timestamp: sample.timestamp,
        objects,
    };
    Ok(ret)
}

/// Extract `FrameGroundTruth` instance which has nearest timestamp with input timestamp.
///
/// * `frame_ground_truths` - List of FrameGroundTruth instances.
/// * `timestamp`           - Target timestamp.
pub fn get_current_frame(
    frame_ground_truths: &[FrameGroundTruth],
    timestamp: &NaiveDateTime,
) -> Option<FrameGroundTruth> {
    const TIME_THRESHOLD: i64 = 75; // [ms]

    // TODO: update timestamp computation
    let target_time = timestamp.timestamp_millis();
    let (min_index, min_diff_time) = frame_ground_truths.iter().enumerate().fold(
        (usize::MAX, i64::MAX),
        |(a_idx, a), (b_idx, b)| {
            let diff = (b.timestamp.timestamp_millis() - target_time).abs();
            if diff < a {
                (b_idx, diff)
            } else {
                (a_idx, a)
            }
        },
    );

    match min_diff_time < TIME_THRESHOLD {
        true => Some(frame_ground_truths[min_index].to_owned()),
        false => {
            log::warn!(
                "Could not find corresponding FrameGroundTruth for timestamp: {}, because {} [ms] > {} [ms]",
                timestamp,
                min_diff_time,
                TIME_THRESHOLD
            );
            None
        }
    }
}
