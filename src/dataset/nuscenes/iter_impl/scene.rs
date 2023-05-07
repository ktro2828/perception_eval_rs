use super::super::{
    internal::{SampleInternal, SceneInternal},
    iter::Iter,
    schema::{Log, LongToken},
    WithDataset,
};
use std::slice::Iter as SliceIter;

impl<'a> WithDataset<'a, SceneInternal> {
    pub fn log(&self) -> WithDataset<'a, Log> {
        self.refer(&self.dataset.log_map[&self.inner.log_token])
    }

    pub fn sample_iter(&self) -> Iter<'a, SampleInternal, SliceIter<'a, LongToken>> {
        self.refer_iter(self.inner.sample_tokens.iter())
    }
}

impl<'a, It> Iterator for Iter<'a, SceneInternal, It>
where
    It: Iterator<Item = &'a LongToken>,
{
    type Item = WithDataset<'a, SceneInternal>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.scene_map[&token]))
    }
}
