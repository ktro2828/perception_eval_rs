use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;
use thiserror::Error as ThisError;

pub type EvaluationTaskResult<T> = Result<T, EvaluationTaskError>;

/// Errors that can occur while constructing `EvaluationTask` instance.
#[derive(Debug, ThisError)]
pub enum EvaluationTaskError {
    #[error("internal error")]
    InternalError,
    #[error("value error")]
    ValueError,
}
/// Represents type of evaluation tasks.
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

impl AsRef<EvaluationTask> for EvaluationTask {
    fn as_ref(&self) -> &EvaluationTask {
        self
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
    /// Returns whether current task is for 3D evaluation.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::evaluation_task::EvaluationTask;
    ///
    /// let task = EvaluationTask::Detection;
    ///
    /// assert_eq!(task.is_3d(), true);
    /// ```
    pub fn is_3d(&self) -> bool {
        matches!(
            self,
            EvaluationTask::Detection | EvaluationTask::Tracking | EvaluationTask::Prediction
        )
    }

    /// Returns whether current task is for 2D evaluation.
    ///
    /// # Examples
    /// ```
    /// use perception_eval::evaluation_task::EvaluationTask;
    ///
    /// let task = EvaluationTask::Detection;
    ///
    /// assert_eq!(task.is_2d(), false);
    /// ```
    pub fn is_2d(&self) -> bool {
        !self.is_3d()
    }
}

/// Convert string task name into `EvaluationTask` instance.
/// If unexpected task name is input, returns `EvaluationTaskError::ValueError`.
///
/// * `task_name`   - Name of task in string.
///
/// # Examples
/// ```
/// use perception_eval::evaluation_task::{EvaluationTask, set_task};
///
/// let task = set_task("detection").unwrap();
///
/// assert_eq!(task, EvaluationTask::Detection);
/// ```
pub fn set_task(task_name: &str) -> EvaluationTaskResult<EvaluationTask> {
    EvaluationTask::from_str(task_name)
}
