use crate::{
    config::FilterParams, label::Label, object::object3d::DynamicObject, threshold::LabelThreshold,
};

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
                &filter_params.target_uuids,
            );
        } else {
            is_target = is_target_object(
                object,
                &filter_params.target_labels,
                &filter_params.max_x_positions,
                &filter_params.max_y_positions,
                &None,
            );
        }

        if is_target {
            ret.push(object.to_owned());
        }
    }
    ret
}

fn is_target_object(
    object: &DynamicObject,
    target_labels: &Vec<Label>,
    max_x_positions: &Vec<f64>,
    max_y_positions: &Vec<f64>,
    // min_point_numbers: &Option<Vec<u64>>,
    target_uuids: &Option<Vec<String>>,
) -> bool {
    let label_threshold = LabelThreshold::new(&object.label, target_labels);

    let mut is_target = true;

    is_target = is_target && target_labels.contains(&object.label);

    is_target = {
        let max_x_position = label_threshold.get_threshold(max_x_positions);
        is_target && object.position[0].abs() < max_x_position.unwrap()
    };

    is_target = {
        let max_y_position = label_threshold.get_threshold(max_y_positions);
        is_target && object.position[0].abs() < max_y_position.unwrap()
    };

    is_target = {
        match target_uuids {
            Some(thresholds) => match &object.uuid {
                Some(uuid) => is_target && thresholds.contains(&uuid),
                None => is_target,
            },
            None => is_target,
        }
    };

    is_target
}
