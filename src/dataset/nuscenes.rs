pub mod error;
pub mod iter;
pub mod schema;

use std::{
    collections::{hash_map::Keys as HashMapKeys, HashMap},
    fs::File,
    io::BufReader,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use error::{NuScenesError, NuScenesResult};
use iter::Iter;
use schema::{
    Attribute, CalibratedSensor, Category, EgoPose, Instance, Log, LongToken, Map, Sample,
    SampleAnnotation, SampleData, Scene, Sensor, Visibility,
};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct NuScenes {
    pub version: String,
    pub data_root: PathBuf,
    pub attributes: HashMap<LongToken, Attribute>,
    pub calibrated_sensors: HashMap<LongToken, CalibratedSensor>,
    pub categories: HashMap<LongToken, Category>,
    pub ego_poses: HashMap<LongToken, EgoPose>,
    pub instances: HashMap<LongToken, Instance>,
    pub logs: HashMap<LongToken, Log>,
    pub maps: HashMap<LongToken, Map>,
    pub scenes: HashMap<LongToken, Scene>,
    pub samples: HashMap<LongToken, Sample>,
    pub sample_annotations: HashMap<LongToken, SampleAnnotation>,
    pub sample_data: HashMap<LongToken, SampleData>,
    pub sensors: HashMap<LongToken, Sensor>,
    pub visibilities: HashMap<LongToken, Visibility>,
}

impl NuScenes {
    // access to the properties
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn data_root(&self) -> &Path {
        &self.data_root
    }

    pub fn load<S, P>(version: S, data_root: P) -> NuScenesResult<Self>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        let dataset_dir: &Path = data_root.as_ref();
        let meta_dir: PathBuf = dataset_dir.join(version.as_ref());

        // load JSON
        let attribute_list: Vec<Attribute> = {
            let attribute_path: PathBuf = meta_dir.join("attribute.json");
            load_json(attribute_path)?
        };

        let calibrated_sensor_list: Vec<CalibratedSensor> = {
            let calibrated_sensor_path: PathBuf = meta_dir.join("calibrated_sensor.json");
            load_json(calibrated_sensor_path)?
        };

        let category_list: Vec<Category> = {
            let category_path: PathBuf = meta_dir.join("category.json");
            load_json(category_path)?
        };

        let ego_pose_list: Vec<EgoPose> = {
            let ego_pose_path: PathBuf = meta_dir.join("ego_pose.json");
            load_json(ego_pose_path)?
        };

        let instance_list: Vec<Instance> = {
            let instance_path: PathBuf = meta_dir.join("instance.json");
            load_json(instance_path)?
        };

        let log_list: Vec<Log> = {
            let log_path: PathBuf = meta_dir.join("log.json");
            load_json(log_path)?
        };

        let map_list: Vec<Map> = {
            let map_path: PathBuf = meta_dir.join("map.json");
            load_json(map_path)?
        };

        let sample_list: Vec<Sample> = {
            let sample_path: PathBuf = meta_dir.join("sample.json");
            load_json(sample_path)?
        };

        let sample_annotation_list: Vec<SampleAnnotation> = {
            let sample_annotation_path: PathBuf = meta_dir.join("sample_annotation.json");
            load_json(sample_annotation_path)?
        };

        let sample_data_list: Vec<SampleData> = {
            let sample_data_path: PathBuf = meta_dir.join("sample_data.json");
            load_json(sample_data_path)?
        };

        let scene_list: Vec<Scene> = {
            let scene_path: PathBuf = meta_dir.join("scene.json");
            load_json(scene_path)?
        };

        let sensor_list: Vec<Sensor> = {
            let sensor_path: PathBuf = meta_dir.join("sensor.json");
            load_json(sensor_path)?
        };

        let visibility_list: Vec<Visibility> = {
            let visibility_path: PathBuf = meta_dir.join("visibility.json");
            load_json(visibility_path)?
        };

        // index items by tokens
        let attributes: HashMap<LongToken, Attribute> = attribute_list
            .into_iter()
            .map(|attribute| (attribute.token.clone(), attribute))
            .collect::<HashMap<_, _>>();

        let calibrated_sensors: HashMap<LongToken, CalibratedSensor> = calibrated_sensor_list
            .into_iter()
            .map(|calibrated_sensor| (calibrated_sensor.token.clone(), calibrated_sensor))
            .collect::<HashMap<_, _>>();

        let categories: HashMap<LongToken, Category> = category_list
            .into_iter()
            .map(|category| (category.token.clone(), category))
            .collect::<HashMap<_, _>>();

        let ego_poses: HashMap<LongToken, EgoPose> = ego_pose_list
            .into_iter()
            .map(|ego_pose| (ego_pose.token.clone(), ego_pose))
            .collect::<HashMap<_, _>>();

        let instances: HashMap<LongToken, Instance> = instance_list
            .into_iter()
            .map(|instance| (instance.token.clone(), instance))
            .collect::<HashMap<_, _>>();

        let logs: HashMap<LongToken, Log> = log_list
            .into_iter()
            .map(|log| (log.token.clone(), log))
            .collect::<HashMap<_, _>>();

        let maps: HashMap<LongToken, Map> = map_list
            .into_iter()
            .map(|map| (map.token.clone(), map))
            .collect::<HashMap<_, _>>();

        let scenes: HashMap<LongToken, Scene> = scene_list
            .into_iter()
            .map(|scene| (scene.token.clone(), scene))
            .collect::<HashMap<_, _>>();

        let samples: HashMap<LongToken, Sample> = sample_list
            .into_iter()
            .map(|sample| (sample.token.clone(), sample))
            .collect::<HashMap<_, _>>();

        let sample_annotations: HashMap<LongToken, SampleAnnotation> = sample_annotation_list
            .into_iter()
            .map(|sample_annotation| (sample_annotation.token.clone(), sample_annotation))
            .collect::<HashMap<_, _>>();

        let sample_data: HashMap<LongToken, SampleData> = sample_data_list
            .into_iter()
            .map(|sample_data| (sample_data.token.clone(), sample_data))
            .collect::<HashMap<_, _>>();

        let sensors: HashMap<LongToken, Sensor> = sensor_list
            .into_iter()
            .map(|sensor| (sensor.token.clone(), sensor))
            .collect::<HashMap<_, _>>();

        let visibilities: HashMap<LongToken, Visibility> = visibility_list
            .into_iter()
            .map(|visibility| (visibility.token.clone(), visibility))
            .collect::<HashMap<_, _>>();

        // TODO: validate integrity
        //
        // --- Do something ---
        //

        let ret: NuScenes = Self {
            version: version.as_ref().to_owned(),
            data_root: dataset_dir.to_owned(),
            attributes: attributes,
            calibrated_sensors: calibrated_sensors,
            categories: categories,
            ego_poses: ego_poses,
            instances: instances,
            logs: logs,
            maps: maps,
            scenes: scenes,
            samples: samples,
            sample_annotations: sample_annotations,
            sample_data: sample_data,
            sensors: sensors,
            visibilities: visibilities,
        };

        Ok(ret)
    }

    // access to the iterators
    pub fn attribute_iter<'a>(
        &'a self,
    ) -> Iter<'a, Attribute, HashMapKeys<'a, LongToken, Attribute>> {
        self.refer_iter(self.attributes.keys())
    }

    pub fn calibrated_sensor_iter<'a>(
        &'a self,
    ) -> Iter<'a, CalibratedSensor, HashMapKeys<'a, LongToken, CalibratedSensor>> {
        self.refer_iter(self.calibrated_sensors.keys())
    }

    fn refer_iter<'a, Value, It>(&'a self, tokens_iter: It) -> Iter<'a, Value, It> {
        Iter {
            dataset: self,
            tokens_iter: tokens_iter,
            phantom: PhantomData,
        }
    }
}

// A wrapper struct that wraps around a base type with a reference to dataset.
#[derive(Debug, Clone)]
pub struct WithDataset<'a, T> {
    pub dataset: &'a NuScenes,
    pub inner: &'a T,
}

impl<'a, T> WithDataset<'a, T> {
    pub fn refer<S>(&self, reference: &'a S) -> WithDataset<'a, S> {
        WithDataset {
            dataset: self.dataset,
            inner: reference,
        }
    }

    pub fn refer_iter<Value, It>(&self, tokens_iter: It) -> Iter<'a, Value, It> {
        Iter {
            dataset: self.dataset,
            tokens_iter: tokens_iter,
            phantom: PhantomData,
        }
    }
}

fn load_json<'de, T, P>(path: P) -> NuScenesResult<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let reader = BufReader::new(File::open(path.as_ref())?);
    let value = serde_json::from_reader(reader).map_err(|err| {
        let msg = format!("failed to load file {}: {:?}", path.as_ref().display(), err);
        NuScenesError::CorruptedDataset(msg)
    })?;

    Ok(value)
}
