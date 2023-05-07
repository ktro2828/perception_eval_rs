pub mod nuscenes;

use self::nuscenes::{error::NuScenesResult, internal::SampleInternal, NuScenes, WithDataset};
use crate::{
    evaluation_task::EvaluationTask,
    frame_id::FrameID,
    label::{LabelConverter, LabelResult},
    object::object3d::DynamicObject,
};
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
    for sample in nusc.sample_iter() {
        let frame = sample_to_frame(&nusc, &sample, frame_id).unwrap();
        datasets.push(frame);
    }
    Ok(datasets)
}

fn sample_to_frame(
    nusc: &NuScenes,
    sample: &WithDataset<SampleInternal>,
    frame_id: &FrameID,
) -> LabelResult<FrameGroundTruth> {
    let mut objects: Vec<DynamicObject> = Vec::new();

    // TODO
    // === update objects container ===
    let label_converter = LabelConverter::new(Some("autoware"))?;
    for sample_annotation in sample.sample_annotation_iter() {
        let instance = nusc.instance_map[&sample_annotation.instance_token].clone();
        let label =
            label_converter.convert(&nusc.category_map[&instance.category_token].clone().name);
        let object = DynamicObject {
            frame_id: frame_id.clone(),
            position: sample_annotation.translation,
            orientation: sample_annotation.rotation,
            size: sample_annotation.size,
            confidence: 1.0,
            label: label,
            velocity: None,
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
