use super::super::{
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
        self.refer(&self.dataset.scene_map[&self.inner.scene_token])
    }

    pub fn prev(&self) -> Option<WithDataset<'a, SampleInternal>> {
        self.inner
            .prev
            .as_ref()
            .map(|token| self.refer(&self.dataset.sample_map[token]))
    }

    pub fn next(&self) -> Option<WithDataset<'a, SampleInternal>> {
        self.inner
            .next
            .as_ref()
            .map(|token| self.refer(&self.dataset.sample_map[token]))
    }
}

impl<'a, It> Iterator for Iter<'a, SampleInternal, It>
where
    It: Iterator<Item = &'a LongToken>,
{
    type Item = WithDataset<'a, SampleInternal>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.sample_map[&token]))
    }
}
