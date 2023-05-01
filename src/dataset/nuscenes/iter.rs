#[derive(Debug, Clone)]
pub struct Iter<'a, Value, It> {
    pub dataset: &'a NuScenes,
    pub token_iter: It,
}

impl<'a, Value, It> Iter<'a, Value, It>
where
    It: Iterator,
{
    pub fn refer(&self, reference: &'a Value) -> WithData<'a, Value> {
        WithData {
            dataset: self.dataset,
            inner: reference,
        }
    }
}
