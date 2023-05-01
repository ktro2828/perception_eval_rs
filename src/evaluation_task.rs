use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;

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
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Detection" | "detection" => Ok(EvaluationTask::Detection),
            "Tracking" | "tracking" => Ok(EvaluationTask::Tracking),
            "Prediction" | "prediction" => Ok(EvaluationTask::Prediction),
            _ => Err(()),
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

pub fn set_task(task_name: &str) -> Result<EvaluationTask, ()> {
    EvaluationTask::from_str(task_name)
}
