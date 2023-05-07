use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;
use thiserror::Error as ThisError;

pub type EvaluationTaskResult<T> = Result<T, EvaluationTaskError>;

#[derive(Debug, ThisError)]
pub enum EvaluationTaskError {
    #[error("internal error")]
    InternalError,
    #[error("value error")]
    ValueError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationTask {
    Detection,
    Tracking,
    Prediction,
}

impl Display for EvaluationTask {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{:?}", self)
    }
}

impl FromStr for EvaluationTask {
    type Err = EvaluationTaskError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Detection" | "detection" => Ok(EvaluationTask::Detection),
            "Tracking" | "tracking" => Ok(EvaluationTask::Tracking),
            "Prediction" | "prediction" => Ok(EvaluationTask::Prediction),
            _ => Err(EvaluationTaskError::ValueError),
        }
    }
}

impl EvaluationTask {
    pub fn is_3d(&self) -> bool {
        matches!(
            self,
            EvaluationTask::Detection | EvaluationTask::Tracking | EvaluationTask::Prediction
        )
    }

    pub fn is_2d(&self) -> bool {
        !self.is_3d()
    }
}

pub fn set_task(task_name: &str) -> EvaluationTaskResult<EvaluationTask> {
    EvaluationTask::from_str(task_name)
}
