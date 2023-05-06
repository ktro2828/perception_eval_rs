use crate::dataset::nuscenes::{
    iter::Iter,
    schema::{Category, LongToken},
    WithDataset,
};

impl<'a, It> Iterator for Iter<'a, Category, It>
where
    It: Iterator<Item = &'a LongToken>,
{
    type Item = WithDataset<'a, Category>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.categories[&token]))
    }
}
