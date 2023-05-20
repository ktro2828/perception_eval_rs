pub mod nuscenes;

use self::nuscenes::{error::NuScenesResult, internal::SampleInternal, NuScenes, WithDataset};
use crate::{
    evaluation_task::EvaluationTask,
    frame_id::FrameID,
    label::{LabelConverter, LabelResult},
    object::object3d::DynamicObject,
};
use chrono::naive::NaiveDateTime;
use indicatif::{ProgressBar, ProgressIterator};
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::path::PathBuf;

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
    version: String,
    data_root: PathBuf,
    evaluation_task: &EvaluationTask,
    frame_id: &FrameID,
) -> NuScenesResult<Vec<FrameGroundTruth>> {
    log::info!(
        "config: evaluation_task: {}, frame_id: {}",
        evaluation_task,
        frame_id,
    );

    let nusc = NuScenes::load(version, data_root)?;
    let mut datasets: Vec<FrameGroundTruth> = Vec::new();
    let bar = ProgressBar::new(nusc.sample_map.len() as u64);
    nusc.sample_iter().progress_with(bar).for_each(|sample| {
        let frame = sample_to_frame(&nusc, &sample, frame_id).unwrap();
        datasets.push(frame);
    });
    Ok(datasets)
}

/// Convert NuScenes sample into `FrameGroundTruth` instance.
///
/// TODO: Transform position and rotation into BaseLink
///
/// let mut position = sample_annotation.translation;
/// let mut orientation = sample_annotation.rotation;
///
/// // How should I get corresponding ego_pose??
/// let ego_position = ego_pose.translation;
/// let ego_orientation = ego_pose.rotation;
///
/// if *frame_id == FrameID::BaseLink {
///     position = rotate(&position, &ego_orientation);
///     orientation = rotate_q(&orientation, &ego_orientation);
/// }
///
///
/// * `nusc`        - NuScenes instance.
/// * `sample`      - Sample annotated in meta data.
/// * `frame_id`    - FrameID instance.
fn sample_to_frame(
    nusc: &NuScenes,
    sample: &WithDataset<SampleInternal>,
    frame_id: &FrameID,
) -> LabelResult<FrameGroundTruth> {
    let mut objects: Vec<DynamicObject> = Vec::new();

    // TODO
    // === update objects container ===
    let label_converter = LabelConverter::new("autoware")?;
    for sample_annotation in sample.sample_annotation_iter() {
        let instance = &nusc.instance_map[&sample_annotation.instance_token];
        let label = label_converter.convert(&nusc.category_map[&instance.category_token].name);
        let object = DynamicObject {
            timestamp: sample.timestamp,
            frame_id: frame_id.to_owned(),
            position: sample_annotation.translation,
            orientation: sample_annotation.rotation,
            size: sample_annotation.size,
            confidence: 1.0,
            label: label,
            velocity: None,
            pointcloud_num: Some(sample_annotation.num_lidar_pts),
            uuid: Some(sample_annotation.instance_token.to_string()),
        };
        objects.push(object);
    }

    let ret = FrameGroundTruth {
        timestamp: sample.timestamp,
        objects: objects,
    };
    Ok(ret)
}

/// Extract `FrameGroundTruth` instance which has nearest timestamp with input timestamp.
///
/// * `frame_ground_truths` - List of FrameGroundTruth instances.
/// * `timestamp`           - Target timestamp.
pub fn get_current_frame(
    frame_ground_truths: &Vec<FrameGroundTruth>,
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
