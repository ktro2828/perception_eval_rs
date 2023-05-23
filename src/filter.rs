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
    objects: &Vec<DynamicObject>,
    is_gt: bool,
    filter_params: &FilterParams,
) -> Vec<DynamicObject> {
    let mut ret = Vec::new();
    for object in objects {
        let is_target;
        if is_gt {
            is_target = is_target_object(
                object,
                &filter_params.target_labels,
                &filter_params.max_x_positions,
                &filter_params.max_y_positions,
                &filter_params.min_point_numbers,
                &filter_params.target_uuids,
            );
        } else {
            is_target = is_target_object(
                object,
                &filter_params.target_labels,
                &filter_params.max_x_positions,
                &filter_params.max_y_positions,
                &None,
                &None,
            );
        }

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
    max_x_positions: &Vec<f64>,
    max_y_positions: &Vec<f64>,
    min_point_numbers: &Option<Vec<usize>>,
    target_uuids: &Option<Vec<String>>,
) -> bool {
    let label_threshold = LabelThreshold::new(&object.label, target_labels);

    let mut is_target = true;

    // target_labels
    is_target = is_target && target_labels.contains(&object.label);

    // max_x_positions
    is_target = {
        let max_x_position = label_threshold.get_threshold(max_x_positions);
        is_target
            && object.position[0].abs()
                < max_x_position.unwrap_or_else(|| {
                    log::error!("There is no corresponding max_x_position");
                    panic!("There is no corresponding max_x_position")
                })
    };

    // max_y_positions
    is_target = {
        let max_y_position = label_threshold.get_threshold(max_y_positions);
        is_target
            && object.position[1].abs()
                < max_y_position.unwrap_or_else(|| {
                    log::error!("There is no corresponding max_y_position");
                    panic!("There is no corresponding max_y_position")
                })
    };

    // min_point_numbers
    is_target = {
        match min_point_numbers {
            Some(thresholds) => match &object.pointcloud_num {
                Some(pt_num) => {
                    let min_point_number = label_threshold.get_threshold(thresholds);
                    is_target
                        && min_point_number.unwrap_or_else(|| {
                            log::warn!("There is no corresponding min_point_number, use 0");
                            0
                        }) <= *pt_num
                }
                None => is_target,
            },
            None => is_target,
        }
    };

    // target_uuids
    is_target = {
        match target_uuids {
            Some(thresholds) => match &object.uuid {
                Some(uuid) => is_target && thresholds.contains(&uuid),
                None => false,
            },
            None => is_target,
        }
    };

    is_target
}

/// Returns hashmap that key is label name and value is list of objects that have same label.
///
/// * `objects`         - List of objects.
/// * `target_labels`   - List of target labels.
#[allow(unused)]
pub(crate) fn divide_objects(
    objects: &Vec<DynamicObject>,
    target_labels: &Vec<Label>,
) -> HashMap<String, Vec<DynamicObject>> {
    let mut ret: HashMap<String, Vec<DynamicObject>> = HashMap::new();

    for label in target_labels {
        ret.insert(label.to_string(), Vec::new());
    }

    for obj in objects {
        match ret.get_mut(&obj.label_name()) {
            Some(v) => v.push(obj.clone()),
            None => (),
        }
    }
    ret
}

/// Returns hashmap that key is label name and value is the number objects that have same label.
///
/// * `objects`         - List of objects.
/// * `target_labels`   - List of target labels.
pub(crate) fn divide_objects_to_num(
    objects: &Vec<DynamicObject>,
    target_labels: &Vec<Label>,
) -> HashMap<String, usize> {
    let mut ret: HashMap<String, usize> = HashMap::new();

    for label in target_labels {
        ret.insert(label.to_string(), 0);
    }

    for obj in objects {
        match ret.get_mut(&obj.label_name()) {
            Some(v) => *v += 1,
            None => (),
        }
    }
    ret
}

/// Returns hashmap that key is label name and value is list of results that estimated object have same label.
///
/// * `results`         - List of results.
/// * `target_labels`   - List of target labels.
pub(crate) fn divide_results(
    results: &Vec<PerceptionResult>,
    target_labels: &Vec<Label>,
) -> HashMap<String, Vec<PerceptionResult>> {
    let mut ret: HashMap<String, Vec<PerceptionResult>> = HashMap::new();

    for label in target_labels {
        ret.insert(label.to_string(), Vec::new());
    }

    for result in results {
        match ret.get_mut(&result.estimated_object.label_name()) {
            Some(v) => v.push(result.clone()),
            None => (),
        }
    }
    ret
}

/// Returns hashmap that key is label name and value is the number of results that estimated object have same label.
///
/// * `results`         - List of results.
/// * `target_labels`   - List of target labels.
#[allow(unused)]
pub(crate) fn divide_results_to_num(
    results: &Vec<PerceptionResult>,
    target_labels: &Vec<Label>,
) -> HashMap<String, usize> {
    let mut ret: HashMap<String, usize> = HashMap::new();

    for label in target_labels {
        ret.insert(label.to_string(), 0);
    }

    for result in results {
        match ret.get_mut(&result.estimated_object.label_name()) {
            Some(v) => *v += 1,
            None => (),
        }
    }
    ret
}

#[cfg(test)]

mod tests {
    use crate::{
        filter::{divide_objects, divide_objects_to_num, is_target_object},
        frame_id::FrameID,
        label::Label,
        object::object3d::DynamicObject,
    };
    use chrono::NaiveDateTime;

    #[test]
    fn test_divide_objects() {
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

        let object_map =
            divide_objects(&vec![object.clone()], &vec![Label::Car, Label::Pedestrian]);
        assert_eq!(*object_map.get("Car").unwrap(), vec![object]);
        assert_eq!(*object_map.get("Pedestrian").unwrap(), vec![]);
    }

    #[test]
    fn test_divide_objects_to_num() {
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

        let object_num_map =
            divide_objects_to_num(&vec![object], &vec![Label::Car, Label::Pedestrian]);
        assert_eq!(*object_num_map.get("Car").unwrap(), 1);
        assert_eq!(*object_num_map.get("Pedestrian").unwrap(), 0);
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
