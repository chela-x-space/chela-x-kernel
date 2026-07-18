use super::{TaskEvidenceSet, TaskInstance, TaskInstanceReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskEvidenceRejectionReason {
    TaskInstanceMismatch,
    UndeclaredEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidenceValidationRequest {
    task_instance: TaskInstance,
    task_evidence_set: TaskEvidenceSet,
}

impl TaskEvidenceValidationRequest {
    pub fn new(task_instance: TaskInstance, task_evidence_set: TaskEvidenceSet) -> Self {
        Self {
            task_instance,
            task_evidence_set,
        }
    }

    pub fn task_instance(&self) -> &TaskInstance {
        &self.task_instance
    }
    pub fn task_evidence_set(&self) -> &TaskEvidenceSet {
        &self.task_evidence_set
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidenceRejected {
    task_evidence_set: TaskEvidenceSet,
    reason: TaskEvidenceRejectionReason,
}

impl TaskEvidenceRejected {
    fn new(task_evidence_set: TaskEvidenceSet, reason: TaskEvidenceRejectionReason) -> Self {
        Self {
            task_evidence_set,
            reason,
        }
    }

    pub fn task_evidence_set(&self) -> &TaskEvidenceSet {
        &self.task_evidence_set
    }
    pub fn reason(&self) -> TaskEvidenceRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEvidenceValidation {
    Accepted(TaskEvidenceSet),
    Rejected(TaskEvidenceRejected),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskEvidenceControl;

impl TaskEvidenceControl {
    pub fn validate(request: &TaskEvidenceValidationRequest) -> TaskEvidenceValidation {
        let task_instance_reference =
            TaskInstanceReference::new(request.task_instance().task_instance_id().clone());
        if request.task_evidence_set().task_instance_reference() != &task_instance_reference {
            return TaskEvidenceValidation::Rejected(TaskEvidenceRejected::new(
                request.task_evidence_set().clone(),
                TaskEvidenceRejectionReason::TaskInstanceMismatch,
            ));
        }
        if request
            .task_evidence_set()
            .task_evidences()
            .iter()
            .any(|evidence| {
                evidence
                    .task_evidence_metadata()
                    .task_evidence_requirement()
                    .is_some_and(|requirement| {
                        !request
                            .task_instance()
                            .task_definition()
                            .task_evidence_requirements()
                            .contains(requirement)
                    })
            })
        {
            return TaskEvidenceValidation::Rejected(TaskEvidenceRejected::new(
                request.task_evidence_set().clone(),
                TaskEvidenceRejectionReason::UndeclaredEvidence,
            ));
        }

        TaskEvidenceValidation::Accepted(request.task_evidence_set().clone())
    }
}
