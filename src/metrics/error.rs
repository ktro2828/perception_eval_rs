use thiserror::Error as ThisError;

use crate::evaluation_task::EvaluationTask;

pub type MetricsResult<T> = Result<T, MetricsError>;

/// Represents error that occurs while computing metrics score.
#[derive(Debug, ThisError)]
pub enum MetricsError {
    #[error("internal error, please report bug")]
    InternalBug,
    #[error("not implemented error: {0}")]
    NotImplementedError(EvaluationTask),
}
