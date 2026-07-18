use crate::state::{
    StateSequence, TransitionAuthorityReference, TransitionEvidenceReference,
    TransitionReasonReference,
};

use super::{
    TaskAssignment, TaskFailureCategory, TaskFailureCode, TaskLifecycleGuards,
    TaskReadinessControl, TaskReadinessDecision, TaskReadinessEvidence, TaskReadinessInput,
    TaskReadinessRequirement, TaskState, TaskStateSnapshot, TaskTransitionRequest,
};

pub(super) fn task_state_snapshot(task_state: TaskState) -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        super::TaskInstanceReference::new(
            crate::task::instance_tests::minimal_task_instance_for_shared_tests()
                .task_instance_id()
                .clone(),
        ),
        task_state,
        StateSequence::new(1).expect("sequence"),
    )
}

pub(super) fn accepted_assignment() -> TaskAssignment {
    super::readiness_test_support::accepted_assignment()
}

pub(super) fn ready_decision() -> TaskReadinessDecision {
    let input = TaskReadinessInput::new(
        task_state_snapshot(TaskState::Pending)
            .task_instance_reference()
            .clone(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(accepted_assignment()),
        vec![
            TaskReadinessRequirement::OwnershipRequired,
            TaskReadinessRequirement::AcceptedAssignmentRequired,
            TaskReadinessRequirement::RequiredInputAvailable,
            TaskReadinessRequirement::DependenciesComplete,
            TaskReadinessRequirement::AuthorizationAllowed,
            TaskReadinessRequirement::EvidencePrerequisitesAvailable,
        ],
        vec![
            TaskReadinessEvidence::RequiredInputAvailable,
            TaskReadinessEvidence::DependenciesComplete,
            TaskReadinessEvidence::AuthorizationAllowed,
            TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
        Some(crate::authorization::AuthorizationDecisionOutcome::Allow),
    );

    TaskReadinessControl::evaluate(&input)
}

pub(super) fn readiness_blocked_decision() -> TaskReadinessDecision {
    let input = TaskReadinessInput::new(
        task_state_snapshot(TaskState::Pending)
            .task_instance_reference()
            .clone(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        None,
        vec![TaskReadinessRequirement::AssignmentRequired],
        vec![],
        None,
    );

    TaskReadinessControl::evaluate(&input)
}

pub(super) fn lifecycle_guards() -> TaskLifecycleGuards {
    TaskLifecycleGuards::new(
        Some(StateSequence::new(1).expect("sequence")),
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        Some(TaskFailureCode::new("FAIL-CODE").expect("failure code")),
        Some(TaskFailureCategory::new("Operational").expect("failure category")),
    )
}

pub(super) fn transition_request(current: TaskState, target: TaskState) -> TaskTransitionRequest {
    TaskTransitionRequest::new(
        task_state_snapshot(current),
        target,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("transition.evidence").expect("evidence")],
        Some(ready_decision()),
        Some(accepted_assignment()),
        lifecycle_guards(),
    )
    .expect("request")
}
