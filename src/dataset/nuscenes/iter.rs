use crate::dataset::nuscenes::{NuScenes, WithDataset};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Iter<'a, Value, It> {
    pub dataset: &'a NuScenes,
    pub tokens_iter: It,
    pub phantom: PhantomData<Value>,
}

impl<'a, Value, It> Iter<'a, Value, It>
where
    It: Iterator,
{
    pub fn refer(&self, reference: &'a Value) -> WithDataset<'a, Value> {
        WithDataset {
            dataset: self.dataset,
            inner: reference,
        }
    }
}
