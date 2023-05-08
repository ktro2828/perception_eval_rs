use crate::{config::MetricsParams, evaluation_task::EvaluationTask};

pub(crate) struct MetricsConfig<'a> {
    evaluation_task: EvaluationTask,
    params: &'a MetricsParams,
}
