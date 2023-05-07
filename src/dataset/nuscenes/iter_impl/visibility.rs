use super::super::{iter::Iter, schema::Visibility, WithDataset};

impl<'a, It> Iterator for Iter<'a, Visibility, It>
where
    It: Iterator<Item = String>,
{
    type Item = WithDataset<'a, Visibility>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.visibility_map[&token]))
    }
}
