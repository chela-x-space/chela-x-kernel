use crate::errors::{DomainError, DomainResult};

use super::{TaskEvidenceReference, TaskOutputContract, TaskStateSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyFact {
    task_state_snapshot: TaskStateSnapshot,
    task_evidence_references: Vec<TaskEvidenceReference>,
    task_output_contracts: Vec<TaskOutputContract>,
}

impl TaskDependencyFact {
    pub fn new(
        task_state_snapshot: TaskStateSnapshot,
        task_evidence_references: Vec<TaskEvidenceReference>,
        task_output_contracts: Vec<TaskOutputContract>,
    ) -> DomainResult<Self> {
        if has_duplicate(&task_evidence_references) {
            return Err(DomainError::InvalidTaskDependency(
                "duplicate task dependency evidence reference",
            ));
        }
        if has_duplicate(&task_output_contracts) {
            return Err(DomainError::InvalidTaskDependency(
                "duplicate task dependency output contract",
            ));
        }

        Ok(Self {
            task_state_snapshot,
            task_evidence_references,
            task_output_contracts,
        })
    }

    pub fn task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.task_state_snapshot
    }
    pub fn task_evidence_references(&self) -> &[TaskEvidenceReference] {
        &self.task_evidence_references
    }
    pub fn task_output_contracts(&self) -> &[TaskOutputContract] {
        &self.task_output_contracts
    }
}

fn has_duplicate<T: PartialEq>(values: &[T]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
}
