use crate::errors::{DomainError, DomainResult};
use crate::request::TimeReference;
use crate::state::TransitionReasonReference;
use crate::{ExecutionSession, TaskCompletion, TaskFailure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionTermination {
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionOutcome {
    Succeeded {
        execution_session: ExecutionSession,
        task_completion: TaskCompletion,
        ended_at: TimeReference,
    },
    Failed {
        execution_session: ExecutionSession,
        task_failure: TaskFailure,
        ended_at: TimeReference,
    },
    Terminated {
        execution_session: ExecutionSession,
        execution_termination: ExecutionTermination,
        termination_reason: TransitionReasonReference,
        ended_at: TimeReference,
    },
}

impl ExecutionOutcome {
    pub fn succeeded(
        execution_session: ExecutionSession,
        task_completion: TaskCompletion,
        ended_at: TimeReference,
    ) -> DomainResult<Self> {
        if task_completion
            .task_completion_result()
            .task_instance_reference()
            != execution_session
                .execution_request()
                .task_instance_reference()
        {
            return Err(DomainError::InvalidExecution(
                "successful execution outcome must preserve task instance continuity",
            ));
        }
        Ok(Self::Succeeded {
            execution_session,
            task_completion,
            ended_at,
        })
    }

    pub fn failed(
        execution_session: ExecutionSession,
        task_failure: TaskFailure,
        ended_at: TimeReference,
    ) -> DomainResult<Self> {
        if task_failure.task_instance_reference()
            != execution_session
                .execution_request()
                .task_instance_reference()
        {
            return Err(DomainError::InvalidExecution(
                "failed execution outcome must preserve task instance continuity",
            ));
        }
        Ok(Self::Failed {
            execution_session,
            task_failure,
            ended_at,
        })
    }

    pub fn terminated(
        execution_session: ExecutionSession,
        execution_termination: ExecutionTermination,
        termination_reason: TransitionReasonReference,
        ended_at: TimeReference,
    ) -> DomainResult<Self> {
        if matches!(
            execution_termination,
            ExecutionTermination::Succeeded | ExecutionTermination::Failed
        ) {
            return Err(DomainError::InvalidExecution(
                "succeeded or failed execution outcomes require explicit completion or failure facts",
            ));
        }
        Ok(Self::Terminated {
            execution_session,
            execution_termination,
            termination_reason,
            ended_at,
        })
    }

    pub fn execution_session(&self) -> &ExecutionSession {
        match self {
            Self::Succeeded {
                execution_session, ..
            }
            | Self::Failed {
                execution_session, ..
            }
            | Self::Terminated {
                execution_session, ..
            } => execution_session,
        }
    }

    pub fn execution_termination(&self) -> ExecutionTermination {
        match self {
            Self::Succeeded { .. } => ExecutionTermination::Succeeded,
            Self::Failed { .. } => ExecutionTermination::Failed,
            Self::Terminated {
                execution_termination,
                ..
            } => *execution_termination,
        }
    }
}
