use crate::dataset::nuscenes::{
    internal::{SampleInternal, SceneInternal},
    iter::Iter,
    schema::{LongToken, SampleAnnotation, SampleData},
    WithDataset,
};
use std::slice::Iter as SliceIter;

impl<'a> WithDataset<'a, SampleInternal> {
    pub fn sample_annotation_iter(&self) -> Iter<'a, SampleAnnotation, SliceIter<'a, LongToken>> {
        self.refer_iter(self.inner.annotation_tokens.iter())
    }

    pub fn sample_data_iter(&self) -> Iter<'a, SampleData, SliceIter<'a, LongToken>> {
        self.refer_iter(self.inner.sample_data_tokens.iter())
    }

    pub fn scene(&self) -> WithDataset<'a, SceneInternal> {
        self.refer(&self.dataset.scenes[&self.inner.scene_token])
    }
}
