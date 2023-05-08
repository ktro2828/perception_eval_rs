use crate::result::PerceptionResult;

use super::config::MetricsConfig;

pub(crate) struct MetricsScore<'a> {
    pub(crate) config: &'a MetricsConfig<'a>,
}

impl<'a> MetricsScore<'a> {
    pub(crate) fn evaluate_detection(&self, results: &Vec<PerceptionResult>) {}
}
