use super::{TaskCompletion, TaskFailure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcomeRejectionReason {
    CompletionFailureConflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskOutcomeDecision {
    Completed(TaskCompletion),
    Failed(TaskFailure),
    Rejected(TaskOutcomeRejectionReason),
}
