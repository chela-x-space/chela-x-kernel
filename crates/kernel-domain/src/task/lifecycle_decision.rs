use crate::state::{
    TransitionAuthorityReference, TransitionEvidenceReference, TransitionReasonReference,
};

use super::{TaskState, TaskStateSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskTransitionRejectionReason {
    IllegalTransition,
    TerminalState,
    MissingAuthority,
    MissingEvidence,
    MissingReason,
    SequenceMismatch,
    ReadinessNotSatisfied,
    AssignmentRequired,
    AuthorizationNotAllowed,
    DependenciesNotSatisfied,
    CompletionConditionsRequired,
    RequiredOutputsMissing,
    FailureCodeRequired,
    FailureCategoryRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAllowedTransition {
    previous_task_state_snapshot: TaskStateSnapshot,
    current_task_state_snapshot: TaskStateSnapshot,
    transition_reason_reference: Option<TransitionReasonReference>,
    transition_authority_reference: Option<TransitionAuthorityReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
}

impl TaskAllowedTransition {
    pub(crate) fn new(
        previous_task_state_snapshot: TaskStateSnapshot,
        current_task_state_snapshot: TaskStateSnapshot,
        transition_reason_reference: Option<TransitionReasonReference>,
        transition_authority_reference: Option<TransitionAuthorityReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
    ) -> Self {
        Self {
            previous_task_state_snapshot,
            current_task_state_snapshot,
            transition_reason_reference,
            transition_authority_reference,
            transition_evidence_references,
        }
    }

    pub fn previous_task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.previous_task_state_snapshot
    }
    pub fn current_task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.current_task_state_snapshot
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRejectedTransition {
    current_task_state_snapshot: TaskStateSnapshot,
    requested_target_task_state: TaskState,
    reason: TaskTransitionRejectionReason,
}

impl TaskRejectedTransition {
    pub(crate) fn new(
        current_task_state_snapshot: TaskStateSnapshot,
        requested_target_task_state: TaskState,
        reason: TaskTransitionRejectionReason,
    ) -> Self {
        Self {
            current_task_state_snapshot,
            requested_target_task_state,
            reason,
        }
    }

    pub fn current_task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.current_task_state_snapshot
    }
    pub fn requested_target_task_state(&self) -> TaskState {
        self.requested_target_task_state
    }
    pub fn reason(&self) -> TaskTransitionRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskNoOpTransition {
    current_task_state_snapshot: TaskStateSnapshot,
}

impl TaskNoOpTransition {
    pub(crate) fn new(current_task_state_snapshot: TaskStateSnapshot) -> Self {
        Self {
            current_task_state_snapshot,
        }
    }

    pub fn current_task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.current_task_state_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskTransitionDecision {
    Allowed(TaskAllowedTransition),
    Rejected(TaskRejectedTransition),
    NoOp(TaskNoOpTransition),
}
