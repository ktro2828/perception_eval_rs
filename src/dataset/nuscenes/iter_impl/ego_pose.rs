use crate::dataset::nuscenes::{
    iter::Iter,
    schema::{EgoPose, LongToken},
    WithDataset,
};

impl<'a, It> Iterator for Iter<'a, EgoPose, It>
where
    It: Iterator<Item = &'a LongToken>,
{
    type Item = WithDataset<'a, EgoPose>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.ego_poses[&token]))
    }
}
