use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NuScenes {
    pub version: String,
    pub data_root: String,
    // // TODO
    // pub attributes: HashMap<LongToken, Attribute>,
    // pub calibrated_sensors: HashMap<LongToken, CalibratedSensor>,
    // pub categories: HashMap<LongToken, Category>,
    // pub ego_pose: HashMap<LongToken, EgoPose>,
    // pub instances: HashMap<LongToken, Instance>,
    // pub logs: HashMap<LongToken, Log>,
    // pub maps: HashMap<LongToken, Map>,
    // pub scenes: HashMap<LongToken, Scene>,
    // pub samples: HashMap<LongToken, Sample>,
    // pub sample_annotations: HashMap<LongToken, SampleAnnotation>,
    // pub sample_data: HashMap<LongToken, SampleData>,
    // pub sensors: HashMap<LongToken, Sensor>,
    // pub visibilities: HashMap<LongToken, Visibility>,
}

impl NuScenes {
    // access to the properties
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn data_root(&self) -> &str {
        &self.data_root
    }

    // access to the iterators
}
