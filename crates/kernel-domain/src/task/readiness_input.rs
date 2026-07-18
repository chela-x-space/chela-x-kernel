use crate::authorization::AuthorizationDecisionOutcome;

use super::{
    TaskAssignment, TaskInstanceReference, TaskOwnership, TaskPriority, TaskReadinessEvidence,
    TaskReadinessRequirement, TaskState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReadinessInput {
    task_instance_reference: TaskInstanceReference,
    task_state: TaskState,
    task_priority: Option<TaskPriority>,
    task_ownership: Option<TaskOwnership>,
    task_assignment: Option<TaskAssignment>,
    task_readiness_requirements: Vec<TaskReadinessRequirement>,
    task_readiness_evidence: Vec<TaskReadinessEvidence>,
    authorization_outcome: Option<AuthorizationDecisionOutcome>,
}

impl TaskReadinessInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_state: TaskState,
        task_priority: Option<TaskPriority>,
        task_ownership: Option<TaskOwnership>,
        task_assignment: Option<TaskAssignment>,
        task_readiness_requirements: Vec<TaskReadinessRequirement>,
        task_readiness_evidence: Vec<TaskReadinessEvidence>,
        authorization_outcome: Option<AuthorizationDecisionOutcome>,
    ) -> Self {
        Self {
            task_instance_reference,
            task_state,
            task_priority,
            task_ownership,
            task_assignment,
            task_readiness_requirements,
            task_readiness_evidence,
            authorization_outcome,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_state(&self) -> TaskState {
        self.task_state
    }
    pub fn task_priority(&self) -> Option<&TaskPriority> {
        self.task_priority.as_ref()
    }
    pub fn task_ownership(&self) -> Option<&TaskOwnership> {
        self.task_ownership.as_ref()
    }
    pub fn task_assignment(&self) -> Option<&TaskAssignment> {
        self.task_assignment.as_ref()
    }
    pub fn task_readiness_requirements(&self) -> &[TaskReadinessRequirement] {
        &self.task_readiness_requirements
    }
    pub fn task_readiness_evidence(&self) -> &[TaskReadinessEvidence] {
        &self.task_readiness_evidence
    }
    pub fn authorization_outcome(&self) -> Option<AuthorizationDecisionOutcome> {
        self.authorization_outcome
    }
}
