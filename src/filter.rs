use std::collections::HashMap;

use crate::{
    config::FilterParams, label::Label, object::object3d::DynamicObject,
    result::object::PerceptionResult, threshold::LabelThreshold,
};

/// Filter objects with `FilterParams`. Returns list of kept objects.
///
/// * `objects`         - List of `DynamicObject` instances.
/// * `is_gt`           - Whether input objects are ground truth.
/// * `filter_params`   - `FilterParam` instance.
///
/// # Examples
/// ```
/// use chrono::NaiveDateTime;
/// use perception_eval::{config::FilterParams, filter::filter_objects, frame_id::FrameID, label::Label, object::object3d::DynamicObject};
///
/// let object1 = DynamicObject {
///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
///     frame_id: FrameID::BaseLink,
///     position: [1.0, 1.0, 0.0],
///     orientation: [1.0, 0.0, 0.0, 0.0],
///     size: [2.0, 1.0, 1.0],
///     velocity: None,
///     confidence: 1.0,
///     label: Label::Car,
///     pointcloud_num: Some(1000),
///     uuid: Some("111".to_string()),
/// };
///
/// let object2 = DynamicObject {
///     timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
///     frame_id: FrameID::BaseLink,
///     position: [10.0, 10.0, 0.0],
///     orientation: [1.0, 0.0, 0.0, 0.0],
///     size: [2.0, 1.0, 1.0],
///     velocity: None,
///     confidence: 1.0,
///     label: Label::Car,
///     pointcloud_num: Some(1000),
///     uuid: Some("111".to_string()),
/// };
///
///
/// let objects = vec![object1.clone(), object2];
/// let filter_params = FilterParams::new(&vec!["car"], 5.0, 5.0, None, None).unwrap();
/// let ret = filter_objects(&objects, false, &filter_params);
///
/// assert_eq!(ret, vec![object1]);
/// ```
pub fn filter_objects(
    objects: &[DynamicObject],
    is_gt: bool,
    filter_params: &FilterParams,
) -> Vec<DynamicObject> {
    let mut ret = Vec::new();
    for object in objects {
        let is_target = if is_gt {
            is_target_object(
                object,
                &filter_params.target_labels,
                &filter_params.max_x_positions,
                &filter_params.max_y_positions,
                &filter_params.min_point_numbers,
                &filter_params.target_uuids,
            )
        } else {
            is_target_object(
                object,
                &filter_params.target_labels,
                &filter_params.max_x_positions,
                &filter_params.max_y_positions,
                &None,
                &None,
            )
        };

        if is_target {
            ret.push(object.to_owned());
        }
    }
    ret
}

/// Returns whether input object is kept.
///
/// * `object`              - DynamicObject instance.
/// * `target_labels`       - List of `Label` instances.
/// * `max_x_positions`     - List of maximum x positions for corresponding label.
/// * `max_y_positions`     - List of maximum y positions for corresponding label.
/// * `min_point_numbers`   - List of minimum number of points the object's box
///                           must contain for corresponding label.
/// * `target_uuids`        - List of instance IDs to be kept.
fn is_target_object(
    object: &DynamicObject,
    target_labels: &Vec<Label>,
    max_x_positions: &[f64],
    max_y_positions: &[f64],
    min_point_numbers: &Option<Vec<usize>>,
    target_uuids: &Option<Vec<String>>,
) -> bool {
    let label_threshold = LabelThreshold::new(&object.label, target_labels);

    // target_labels
    let mut is_target = target_labels.contains(&object.label);

    // Following filters must satisfy that object's label is included in target_labels
    if !is_target {
        return false;
    }

    // max_x_positions
    is_target &= {
        let max_x_position = label_threshold.get_threshold(max_x_positions);
        object.position[0].abs()
            < max_x_position.unwrap_or_else(|| {
                log::error!("There is no corresponding max_x_position");
                panic!("There is no corresponding max_x_position")
            })
    };

    // max_y_positions
    is_target &= {
        let max_y_position = label_threshold.get_threshold(max_y_positions);
        object.position[1].abs()
            < max_y_position.unwrap_or_else(|| {
                log::error!("There is no corresponding max_y_position");
                panic!("There is no corresponding max_y_position")
            })
    };

    // min_point_numbers
    is_target &= {
        match min_point_numbers {
            Some(thresholds) => match &object.pointcloud_num {
                Some(pt_num) => {
                    let min_point_number = label_threshold.get_threshold(thresholds);
                    min_point_number.unwrap_or_else(|| {
                        log::warn!("There is no corresponding min_point_number, use 0");
                        0
                    }) <= *pt_num
                }
                None => true,
            },
            None => true,
        }
    };

    // target_uuids
    is_target &= {
        match target_uuids {
            Some(thresholds) => match &object.uuid {
                Some(uuid) => thresholds.contains(uuid),
                None => false,
            },
            None => true,
        }
    };

    is_target
}

/// Returns hashmap that key is `Label` and value is list of objects that have same label.
///
/// * `objects`         - List of objects.
/// * `target_labels`   - List of target labels.
#[allow(unused)]
pub(crate) fn hash_objects(
    objects: &[DynamicObject],
    target_labels: &[Label],
) -> HashMap<Label, Vec<DynamicObject>> {
    let mut ret: HashMap<Label, Vec<DynamicObject>> = HashMap::new();

    target_labels.iter().for_each(|label| {
        ret.insert(label.to_owned(), Vec::new());
    });

    objects.iter().for_each(|obj| {
        if let Some(v) = ret.get_mut(&obj.label) {
            v.push(obj.clone())
        }
    });

    ret
}

/// Returns hashmap that key is `Label` and value is the number objects that have same label.
///
/// * `objects`         - List of objects.
/// * `target_labels`   - List of target labels.
pub(crate) fn hash_num_objects(
    objects: &[DynamicObject],
    target_labels: &[Label],
) -> HashMap<Label, usize> {
    let mut ret: HashMap<Label, usize> = HashMap::new();

    target_labels.iter().for_each(|label| {
        ret.insert(label.to_owned(), 0);
    });

    objects.iter().for_each(|obj| {
        if let Some(v) = ret.get_mut(&obj.label) {
            *v += 1
        }
    });

    ret
}

/// Returns hashmap that key is `Label` and value is list of results that estimated object have same label.
///
/// * `results`         - List of results.
/// * `target_labels`   - List of target labels.
pub(crate) fn hash_results(
    results: &[PerceptionResult],
    target_labels: &[Label],
) -> HashMap<Label, Vec<PerceptionResult>> {
    let mut ret: HashMap<Label, Vec<PerceptionResult>> = HashMap::new();

    target_labels.iter().for_each(|label| {
        ret.insert(label.to_owned(), Vec::new());
    });

    results.iter().for_each(|result| {
        if let Some(v) = ret.get_mut(&result.estimated_object.label) {
            v.push(result.clone())
        }
    });

    ret
}

/// Returns hashmap that key is `Label` and value is the number of results that estimated object have same label.
///
/// * `results`         - List of results.
/// * `target_labels`   - List of target labels.
#[allow(unused)]
pub(crate) fn hash_num_results(
    results: &[PerceptionResult],
    target_labels: &[Label],
) -> HashMap<Label, usize> {
    let mut ret: HashMap<Label, usize> = HashMap::new();

    target_labels.iter().for_each(|label| {
        ret.insert(label.to_owned(), 0);
    });

    results.iter().for_each(|result| {
        if let Some(v) = ret.get_mut(&result.estimated_object.label) {
            *v += 1
        }
    });

    ret
}

#[cfg(test)]
mod tests {
    use crate::{
        filter::{hash_num_objects, hash_objects, is_target_object},
        frame_id::FrameID,
        label::Label,
        object::object3d::DynamicObject,
    };
    use chrono::NaiveDateTime;

    #[test]
    fn test_hash_objects() {
        let object = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [1.0, 1.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };

        let object_map = hash_objects(&vec![object.clone()], &vec![Label::Car, Label::Pedestrian]);
        assert_eq!(*object_map.get(&Label::Car).unwrap(), vec![object]);
        assert_eq!(*object_map.get(&Label::Pedestrian).unwrap(), vec![]);
    }

    #[test]
    fn test_hash_num_objects() {
        let object = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [1.0, 1.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };

        let object_num_map = hash_num_objects(&vec![object], &vec![Label::Car, Label::Pedestrian]);
        assert_eq!(*object_num_map.get(&Label::Car).unwrap(), 1);
        assert_eq!(*object_num_map.get(&Label::Pedestrian).unwrap(), 0);
    }

    #[test]
    fn test_is_target_object() {
        let object = DynamicObject {
            timestamp: NaiveDateTime::from_timestamp_micros(10000).unwrap(),
            frame_id: FrameID::BaseLink,
            position: [1.0, 1.0, 0.0],
            orientation: [1.0, 0.0, 0.0, 0.0],
            size: [2.0, 1.0, 1.0],
            velocity: None,
            confidence: 1.0,
            label: Label::Car,
            pointcloud_num: Some(1000),
            uuid: Some("111".to_string()),
        };

        let target_labels = vec![Label::Car, Label::Pedestrian];
        let max_x_positions = vec![20.0, 10.0];
        let max_y_positions = vec![20.0, 10.0];
        let min_point_numbers = Some(vec![100, 100]);
        let target_uuids = None;

        let is_target = is_target_object(
            &object,
            &target_labels,
            &max_x_positions,
            &max_y_positions,
            &min_point_numbers,
            &target_uuids,
        );

        assert_eq!(is_target, true);
    }
}
