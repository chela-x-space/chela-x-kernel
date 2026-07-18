use crate::errors::{DomainError, DomainResult};
use crate::state::{
    TransitionAuthorityReference, TransitionEvidenceReference, TransitionReasonReference,
};

use super::{
    TaskAssignment, TaskLifecycleGuards, TaskReadinessDecision, TaskState, TaskStateSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTransitionRequest {
    current_task_state_snapshot: TaskStateSnapshot,
    requested_target_task_state: TaskState,
    transition_reason_reference: Option<TransitionReasonReference>,
    transition_authority_reference: Option<TransitionAuthorityReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
    task_readiness_decision: Option<TaskReadinessDecision>,
    task_assignment: Option<TaskAssignment>,
    task_lifecycle_guards: TaskLifecycleGuards,
}

impl TaskTransitionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        current_task_state_snapshot: TaskStateSnapshot,
        requested_target_task_state: TaskState,
        transition_reason_reference: Option<TransitionReasonReference>,
        transition_authority_reference: Option<TransitionAuthorityReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
        task_readiness_decision: Option<TaskReadinessDecision>,
        task_assignment: Option<TaskAssignment>,
        task_lifecycle_guards: TaskLifecycleGuards,
    ) -> DomainResult<Self> {
        for (index, evidence) in transition_evidence_references.iter().enumerate() {
            if transition_evidence_references[..index]
                .iter()
                .any(|prior| prior == evidence)
            {
                return Err(DomainError::InvalidTaskLifecycle(
                    "duplicate task transition evidence reference",
                ));
            }
        }

        Ok(Self {
            current_task_state_snapshot,
            requested_target_task_state,
            transition_reason_reference,
            transition_authority_reference,
            transition_evidence_references,
            task_readiness_decision,
            task_assignment,
            task_lifecycle_guards,
        })
    }

    pub fn current_task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.current_task_state_snapshot
    }
    pub fn requested_target_task_state(&self) -> TaskState {
        self.requested_target_task_state
    }
    pub fn transition_reason_reference(&self) -> Option<&TransitionReasonReference> {
        self.transition_reason_reference.as_ref()
    }
    pub fn transition_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.transition_authority_reference.as_ref()
    }
    pub fn transition_evidence_references(&self) -> &[TransitionEvidenceReference] {
        &self.transition_evidence_references
    }
    pub fn task_readiness_decision(&self) -> Option<&TaskReadinessDecision> {
        self.task_readiness_decision.as_ref()
    }
    pub fn task_assignment(&self) -> Option<&TaskAssignment> {
        self.task_assignment.as_ref()
    }
    pub fn task_lifecycle_guards(&self) -> &TaskLifecycleGuards {
        &self.task_lifecycle_guards
    }
}
