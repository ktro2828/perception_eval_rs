pub mod nuscenes;

use self::nuscenes::{error::NuScenesResult, schema::Sample, NuScenes};
use crate::{evaluation_task::EvaluationTask, frame_id::FrameID, object::object3d::DynamicObject};
use chrono::naive::NaiveDateTime;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FrameGroundTruth {
    timestamp: NaiveDateTime,
    objects: Vec<DynamicObject>,
}

pub fn load_dataset(
    version: String,
    data_root: PathBuf,
    evaluation_task: EvaluationTask,
    frame_id: FrameID,
    load_raw_data: bool,
) -> NuScenesResult<Vec<FrameGroundTruth>> {
    log::info!(
        "config: load_raw_data: {}, evaluation_task: {}, frame_id: {}",
        load_raw_data,
        evaluation_task,
        frame_id,
    );

    let nusc = NuScenes::load(version, data_root)?;
    let mut datasets: Vec<FrameGroundTruth> = Vec::new();
    for sample in nusc.sample_iter() {
        datasets.push(sample_to_frame(
            nusc,
            sample,
            evaluation_task,
            frame_id,
            load_raw_data,
        ));
    }
    Ok(datasets)
}

fn sample_to_frame(
    nusc: NuScenes,
    sample: Sample,
    evaluation_task: EvaluationTask,
    frame_id: FrameID,
    load_raw_data: bool,
) -> FrameGroundTruth {
    let mut objects: Vec<DynamicObject> = Vec::new();

    // TODO
    // === update objects container ===
    // DO SOMETHING

    FrameGroundTruth {
        timestamp: sample.timestamp,
        objects: objects,
    }
}

pub fn get_current_frame(
    frame_ground_truths: &Vec<FrameGroundTruth>,
    timestamp: &NaiveDateTime,
) -> Option<FrameGroundTruth> {
    const time_threshold: i64 = 75; // [ms]

    // TODO: update timestamp computation
    let target_time = timestamp.timestamp_millis();
    let (min_index, min_diff_time) = frame_ground_truths.iter().enumerate().fold(
        (usize::MAX, i64::MAX),
        |(a_idx, a), (b_idx, &b)| {
            let diff = (b.timestamp.timestamp_millis() - target_time).abs();
            if diff < a {
                (b_idx, diff)
            } else {
                (a_idx, a)
            }
        },
    );

    match min_diff_time < time_threshold {
        true => Some(frame_ground_truths[min_index]),
        false => {
            log::warn!(
                "Could not find corresponding FrameGroundTruth for timestamp: {}, because {} [ms] > {} [ms]",
                timestamp,
                min_diff_time,
                time_threshold
            );
            None
        }
    }
}
