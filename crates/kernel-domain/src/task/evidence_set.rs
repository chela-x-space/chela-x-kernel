use crate::errors::{DomainError, DomainResult};

use super::{TaskEvidence, TaskInstanceReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidenceSet {
    task_instance_reference: TaskInstanceReference,
    task_evidences: Vec<TaskEvidence>,
}

impl TaskEvidenceSet {
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_evidences: Vec<TaskEvidence>,
    ) -> DomainResult<Self> {
        for (index, evidence) in task_evidences.iter().enumerate() {
            if evidence.subject_task_instance_reference() != &task_instance_reference {
                return Err(DomainError::InvalidTaskEvidence(
                    "task evidence subject must match evidence set task instance",
                ));
            }
            if task_evidences[..index]
                .iter()
                .any(|prior| prior.task_evidence_reference() == evidence.task_evidence_reference())
            {
                return Err(DomainError::InvalidTaskEvidence(
                    "duplicate task evidence identity",
                ));
            }
        }

        Ok(Self {
            task_instance_reference,
            task_evidences,
        })
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_evidences(&self) -> &[TaskEvidence] {
        &self.task_evidences
    }
}
