use super::{
    TaskInstanceReference, TaskReadiness, TaskReadinessBlocker, TaskReadinessEvidence,
    TaskReadinessRejectionReason,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReadinessReady {
    task_instance_reference: TaskInstanceReference,
    task_readiness: TaskReadiness,
    validated_evidence: Vec<TaskReadinessEvidence>,
}

impl TaskReadinessReady {
    pub(crate) fn new(
        task_instance_reference: TaskInstanceReference,
        validated_evidence: Vec<TaskReadinessEvidence>,
    ) -> Self {
        Self {
            task_instance_reference,
            task_readiness: TaskReadiness::Ready,
            validated_evidence,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_readiness(&self) -> TaskReadiness {
        self.task_readiness
    }
    pub fn validated_evidence(&self) -> &[TaskReadinessEvidence] {
        &self.validated_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReadinessBlocked {
    task_instance_reference: TaskInstanceReference,
    task_readiness: TaskReadiness,
    blockers: Vec<TaskReadinessBlocker>,
}

impl TaskReadinessBlocked {
    pub(crate) fn new(
        task_instance_reference: TaskInstanceReference,
        blockers: Vec<TaskReadinessBlocker>,
    ) -> Self {
        Self {
            task_instance_reference,
            task_readiness: TaskReadiness::Blocked,
            blockers,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_readiness(&self) -> TaskReadiness {
        self.task_readiness
    }
    pub fn blockers(&self) -> &[TaskReadinessBlocker] {
        &self.blockers
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReadinessRejection {
    task_instance_reference: TaskInstanceReference,
    reason: TaskReadinessRejectionReason,
}

impl TaskReadinessRejection {
    pub(crate) fn new(
        task_instance_reference: TaskInstanceReference,
        reason: TaskReadinessRejectionReason,
    ) -> Self {
        Self {
            task_instance_reference,
            reason,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn reason(&self) -> TaskReadinessRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskReadinessDecision {
    Ready(TaskReadinessReady),
    Blocked(TaskReadinessBlocked),
    Rejected(TaskReadinessRejection),
}
