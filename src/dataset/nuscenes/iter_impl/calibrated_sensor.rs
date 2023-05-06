use crate::dataset::nuscenes::{
    iter::Iter,
    schema::{CalibratedSensor, LongToken, Sensor},
    WithDataset,
};

impl<'a> WithDataset<'a, CalibratedSensor> {
    pub fn sensor(&self) -> WithDataset<'a, Sensor> {
        self.refer(&self.dataset.sensors[&self.inner.sensor_token])
    }
}

impl<'a, It> Iterator for Iter<'a, CalibratedSensor, It>
where
    It: Iterator<Item = &'a LongToken>,
{
    type Item = WithDataset<'a, CalibratedSensor>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens_iter
            .next()
            .map(|token| self.refer(&self.dataset.calibrated_sensors[&token]))
    }
}
