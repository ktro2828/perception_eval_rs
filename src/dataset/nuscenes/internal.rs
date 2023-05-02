use std::collections::HashMap;

use chrono::NaiveDateTime;

use super::{
    error::{NuScenesError, NuScenesResult},
    schema::{Instance, LongToken, Sample, SampleAnnotation, Scene},
};

#[derive(Debug, Clone)]
pub struct SampleInternal {
    pub token: LongToken,
    pub next: Option<LongToken>,
    pub prev: Option<LongToken>,
    pub timestamp: NaiveDateTime,
    pub scene_token: LongToken,
    pub annotation_tokens: Vec<LongToken>,
    pub sample_data_tokens: Vec<LongToken>,
}

impl SampleInternal {
    pub fn from(
        sample: Sample,
        annotation_tokens: Vec<LongToken>,
        sample_data_tokens: Vec<LongToken>,
    ) -> Self {
        let Sample {
            token,
            next,
            prev,
            scene_token,
            timestamp,
        } = sample;

        Self {
            token: token,
            next: next,
            prev: prev,
            timestamp: timestamp,
            scene_token: scene_token,
            annotation_tokens: annotation_tokens,
            sample_data_tokens: sample_data_tokens,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstanceInternal {
    pub token: LongToken,
    pub category_token: LongToken,
    pub annotation_tokens: Vec<LongToken>,
}

impl InstanceInternal {
    pub fn from(
        instance: Instance,
        sample_annotations: &HashMap<LongToken, SampleAnnotation>,
    ) -> NuScenesResult<Self> {
        let Instance {
            token,
            nbr_annotations,
            category_token,
            first_annotation_token,
            last_annotation_token,
        } = instance;

        let mut annotation_token_opt = Some(&first_annotation_token);
        let mut annotation_tokens = Vec::new();

        while let Some(annotation_token) = annotation_token_opt {
            let annotation = &sample_annotations
                .get(annotation_token)
                .ok_or(NuScenesError::InternalError)?;
            if annotation_token != &annotation.token {
                return Err(NuScenesError::InternalError);
            }
            annotation_tokens.push(annotation_token.clone());
            annotation_token_opt = annotation.next.as_ref();
        }

        if annotation_tokens.len() != nbr_annotations {
            let msg = format!(
                "the instance with token {} assures nbr_annotations = {}, but got {}",
                token,
                nbr_annotations,
                annotation_tokens.len()
            );
            return Err(NuScenesError::CorruptedDataset(msg));
        }

        if annotation_tokens.last().unwrap() != &last_annotation_token {
            let msg = format!(
                "the instance with token {} assures last_annotation_token = {}, but got {}",
                token,
                last_annotation_token,
                annotation_tokens.last().unwrap()
            );
            return Err(NuScenesError::CorruptedDataset(msg));
        }

        let ret = Self {
            token: token,
            category_token: category_token,
            annotation_tokens: annotation_tokens,
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone)]
pub struct SceneInternal {
    pub token: LongToken,
    pub name: String,
    pub description: String,
    pub log_token: LongToken,
    pub sample_tokens: Vec<LongToken>,
}

impl SceneInternal {
    pub fn from(scene: Scene, samples: &HashMap<LongToken, Sample>) -> NuScenesResult<Self> {
        let Scene {
            token,
            name,
            description,
            log_token,
            nbr_samples,
            first_sample_token,
            last_sample_token,
        } = scene;

        let mut sample_token_opt = Some(&first_sample_token);
        let mut sample_tokens = Vec::new();

        while let Some(sample_token) = sample_token_opt {
            let sample = &samples[sample_token];
            if &sample.token != sample_token {
                return Err(NuScenesError::InternalError);
            }
            sample_tokens.push(sample_token.clone());
            sample_token_opt = sample.next.as_ref();
        }

        if sample_tokens.len() != nbr_samples {
            let msg = format!(
                "the sample with token {} assures nbr_samples = {}, but got {}",
                token,
                nbr_samples,
                sample_tokens.len()
            );
            return Err(NuScenesError::CorruptedDataset(msg));
        }

        if sample_tokens.last().unwrap() != &last_sample_token {
            let msg = format!(
                "the sample with token {} assures last_sample_token = {}, but got {}",
                token,
                last_sample_token,
                sample_tokens.last().unwrap()
            );
            return Err(NuScenesError::CorruptedDataset(msg));
        }

        let ret = Self {
            token: token,
            name: name,
            description: description,
            log_token: log_token,
            sample_tokens: sample_tokens,
        };

        Ok(ret)
    }
}
