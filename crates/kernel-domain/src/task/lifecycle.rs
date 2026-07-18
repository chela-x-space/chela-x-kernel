use crate::errors::DomainResult;
use crate::identifier::NonEmptyText;
use crate::state::StateSequence;

use super::{TaskInstanceReference, TaskState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStateSnapshot {
    task_instance_reference: TaskInstanceReference,
    task_state: TaskState,
    state_sequence: StateSequence,
}

impl TaskStateSnapshot {
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_state: TaskState,
        state_sequence: StateSequence,
    ) -> Self {
        Self {
            task_instance_reference,
            task_state,
            state_sequence,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_state(&self) -> TaskState {
        self.task_state
    }
    pub fn state_sequence(&self) -> StateSequence {
        self.state_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureCode(NonEmptyText);

impl TaskFailureCode {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_failure_code", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureCategory(NonEmptyText);

impl TaskFailureCategory {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_failure_category", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
