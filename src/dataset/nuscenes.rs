pub mod error;
pub mod internal;
pub mod iter;
pub mod iter_impl;
pub mod schema;

use error::{NuScenesError, NuScenesResult};
use image::DynamicImage;
use iter::Iter;
use itertools::Itertools;
use nalgebra::{Dyn, Matrix, VecStorage, U5};
use schema::{
    Attribute, CalibratedSensor, Category, EgoPose, Instance, Log, LongToken, Map, Sample,
    SampleAnnotation, SampleData, Scene, Sensor, ShortToken, Visibility,
};
use serde::de::DeserializeOwned;
use std::{
    collections::{hash_map::Keys as HashMapKeys, HashMap},
    fs::File,
    io::BufReader,
    marker::PhantomData,
    path::{Path, PathBuf},
    slice::Iter as SliceIter,
};

use self::internal::{InstanceInternal, SampleInternal, SceneInternal};

pub type PointCloudMatrix = Matrix<f32, Dyn, U5, VecStorage<f32, Dyn, U5>>;

#[derive(Clone)]
pub enum RawData {
    PointCloud(PointCloudMatrix),
    Image(DynamicImage),
}

#[derive(Debug, Clone)]
pub struct NuScenes {
    pub(super) version: String,
    pub(super) data_root: PathBuf,
    pub(super) attributes: HashMap<LongToken, Attribute>,
    pub(super) calibrated_sensors: HashMap<LongToken, CalibratedSensor>,
    pub(super) categories: HashMap<LongToken, Category>,
    pub(super) ego_poses: HashMap<LongToken, EgoPose>,
    pub(super) instances: HashMap<LongToken, Instance>,
    pub(super) logs: HashMap<LongToken, Log>,
    pub(super) maps: HashMap<ShortToken, Map>,
    pub(super) scenes: HashMap<LongToken, Scene>,
    pub(super) samples: HashMap<LongToken, Sample>,
    pub(super) sample_annotations: HashMap<LongToken, SampleAnnotation>,
    pub(super) sample_data: HashMap<LongToken, SampleData>,
    pub(super) sensors: HashMap<LongToken, Sensor>,
    pub(super) visibilities: HashMap<LongToken, Visibility>,
    pub(super) sorted_ego_pose_tokens: Vec<LongToken>,
    pub(super) sorted_sample_tokens: Vec<LongToken>,
    pub(super) sorted_sample_data_tokens: Vec<LongToken>,
    pub(super) sorted_scene_tokens: Vec<LongToken>,
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

        let maps: HashMap<ShortToken, Map> = map_list
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

        let mut sample_to_annotation_groups = sample_annotations
            .iter()
            .map(|(sample_annotation_token, sample_annotation)| {
                (
                    sample_annotation.sample_token.clone(),
                    sample_annotation_token.clone(),
                )
            })
            .into_group_map();

        let mut sample_to_sample_data_groups = sample_data
            .iter()
            .map(|(sample_data_token, sample_data)| {
                (sample_data.sample_token.clone(), sample_data_token.clone())
            })
            .into_group_map();

        let instance_internal_map = instances
            .into_iter()
            .map(|(instance_token, instance)| {
                let ret = InstanceInternal::from(instance, &sample_annotations)?;
                Ok((instance_token, ret))
            })
            .collect::<NuScenesResult<HashMap<_, _>>>()?;

        let scene_internal_map = scenes
            .into_iter()
            .map(|(scene_token, scene)| {
                let internal = SceneInternal::from(scene, &samples)?;
                Ok((scene_token, internal))
            })
            .collect::<NuScenesResult<HashMap<_, _>>>()?;

        let sample_internal_map = samples
            .into_iter()
            .map(|(sample_token, sample)| {
                let sample_data_tokens = sample_to_sample_data_groups
                    .remove(&sample_token)
                    .ok_or(NuScenesError::InternalError)?;
                let annotation_tokens = sample_to_annotation_groups
                    .remove(&sample_token)
                    .ok_or(NuScenesError::InternalError)?;
                let internal = SampleInternal::from(sample, annotation_tokens, sample_data_tokens);
                Ok((sample_token, internal))
            })
            .collect::<NuScenesResult<HashMap<_, _>>>()?;

        let sorted_ego_pose_tokens = {
            let mut sorted_pairs = ego_poses
                .iter()
                .map(|(sample_token, sample)| (sample_token, sample.timestamp))
                .collect::<Vec<_>>();
            sorted_pairs.sort_by_cached_key(|(_, timestamp)| timestamp.clone());

            sorted_pairs
                .into_iter()
                .map(|(token, _)| token.clone())
                .collect::<Vec<_>>()
        };

        let sorted_sample_tokens = {
            let mut sorted_pairs = sample_internal_map
                .iter()
                .map(|(sample_token, sample)| (sample_token, sample.timestamp))
                .collect::<Vec<_>>();
            sorted_pairs.sort_by_cached_key(|(_, timestamp)| timestamp.clone());

            sorted_pairs
                .into_iter()
                .map(|(token, _)| token.clone())
                .collect::<Vec<_>>()
        };

        let sorted_sample_data_tokens = {
            let mut sorted_pairs = sample_data
                .iter()
                .map(|(sample_token, sample)| (sample_token, sample.timestamp))
                .collect::<Vec<_>>();
            sorted_pairs.sort_by_cached_key(|(_, timestamp)| timestamp.clone());

            sorted_pairs
                .into_iter()
                .map(|(token, _)| token.clone())
                .collect::<Vec<_>>()
        };

        let sorted_scene_tokens = {
            let mut sorted_pairs = scene_internal_map
                .iter()
                .map(|(scene_token, scene)| {
                    let timestamp = scene
                        .sample_tokens
                        .iter()
                        .map(|sample_token| {
                            let sample = sample_internal_map
                                .get(&sample_token)
                                .ok_or(NuScenesError::InternalError)?;
                            Ok(sample.timestamp)
                        })
                        .collect::<NuScenesResult<Vec<_>>>()?
                        .into_iter()
                        .min()
                        .ok_or(NuScenesError::InternalError)?;

                    Ok((scene_token, timestamp))
                })
                .collect::<NuScenesResult<Vec<_>>>()?;
            sorted_pairs.sort_by_cached_key(|(_, timestamp)| timestamp.clone());

            sorted_pairs
                .into_iter()
                .map(|(token, _)| token.clone())
                .collect::<Vec<_>>()
        };

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
            sorted_ego_pose_tokens: sorted_ego_pose_tokens,
            sorted_scene_tokens: sorted_scene_tokens,
            sorted_sample_tokens: sorted_sample_tokens,
            sorted_sample_data_tokens: sorted_sample_data_tokens,
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

    pub fn category_iter<'a>(&'a self) -> Iter<'a, Category, HashMapKeys<'a, LongToken, Category>> {
        self.refer_iter(self.categories.keys())
    }

    pub fn ego_pose_iter<'a>(&'a self) -> Iter<'a, EgoPose, SliceIter<'a, LongToken>> {
        self.refer_iter(self.sorted_ego_pose_tokens.iter())
    }

    pub fn instance_iter<'a>(&'a self) -> Iter<'a, Instance, HashMapKeys<'a, LongToken, Instance>> {
        self.refer_iter(self.instances.keys())
    }

    pub fn log_iter<'a>(&'a self) -> Iter<'a, Log, HashMapKeys<'a, LongToken, Log>> {
        self.refer_iter(self.logs.keys())
    }

    pub fn map_iter<'a>(&'a self) -> Iter<'a, Map, HashMapKeys<'a, ShortToken, Map>> {
        self.refer_iter(self.maps.keys())
    }

    pub fn scene_iter<'a>(&'a self) -> Iter<'a, Scene, SliceIter<'a, LongToken>> {
        self.refer_iter(self.sorted_scene_tokens.iter())
    }

    pub fn sample_iter<'a>(&'a self) -> Iter<'a, SampleInternal, SliceIter<'a, LongToken>> {
        self.refer_iter(self.sorted_sample_tokens.iter())
    }

    pub fn sample_annotation_iter<'a>(
        &'a self,
    ) -> Iter<'a, SampleAnnotation, HashMapKeys<'a, LongToken, SampleAnnotation>> {
        self.refer_iter(self.sample_annotations.keys())
    }

    pub fn sample_data_iter<'a>(&'a self) -> Iter<'a, SampleData, SliceIter<'a, LongToken>> {
        self.refer_iter(self.sorted_sample_data_tokens.iter())
    }

    pub fn sensor_iter<'a>(&'a self) -> Iter<'a, Sensor, HashMapKeys<'a, LongToken, Sensor>> {
        self.refer_iter(self.sensors.keys())
    }

    pub fn visibility_iter<'a>(
        &'a self,
    ) -> Iter<'a, Visibility, HashMapKeys<'a, LongToken, Visibility>> {
        self.refer_iter(self.visibilities.keys())
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
