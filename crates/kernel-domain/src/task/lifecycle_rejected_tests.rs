use crate::state::{StateSequence, TransitionAuthorityReference, TransitionReasonReference};

use super::{
    TaskFailureCategory, TaskFailureCode, TaskLifecycleGuards, TaskState, TaskTransitionControl,
    TaskTransitionDecision, TaskTransitionRejectionReason, TaskTransitionRequest,
};

#[test]
fn task_lifecycle_pending_to_completed_is_rejected() {
    assert_rejection(
        super::lifecycle_test_support::transition_request(TaskState::Pending, TaskState::Completed),
        TaskTransitionRejectionReason::IllegalTransition,
    );
}

#[test]
fn task_lifecycle_pending_to_failed_is_rejected() {
    assert_rejection(
        super::lifecycle_test_support::transition_request(TaskState::Pending, TaskState::Failed),
        TaskTransitionRejectionReason::IllegalTransition,
    );
}

#[test]
fn task_lifecycle_in_progress_to_pending_is_rejected() {
    assert_rejection(
        super::lifecycle_test_support::transition_request(
            TaskState::InProgress,
            TaskState::Pending,
        ),
        TaskTransitionRejectionReason::IllegalTransition,
    );
}

#[test]
fn task_lifecycle_archived_to_any_state_is_rejected() {
    assert_rejection(
        super::lifecycle_test_support::transition_request(TaskState::Archived, TaskState::Pending),
        TaskTransitionRejectionReason::TerminalState,
    );
}

#[test]
fn task_lifecycle_sequence_mismatch_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending),
        TaskState::InProgress,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        Vec::new(),
        Some(super::lifecycle_test_support::ready_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        TaskLifecycleGuards::new(
            Some(StateSequence::new(2).expect("sequence")),
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            Some(TaskFailureCode::new("FAIL-CODE").expect("failure code")),
            Some(TaskFailureCategory::new("Operational").expect("failure category")),
        ),
    )
    .expect("request");

    assert_rejection(request, TaskTransitionRejectionReason::SequenceMismatch);
}

#[test]
fn task_lifecycle_start_without_ready_readiness_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending),
        TaskState::InProgress,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        Vec::new(),
        Some(super::lifecycle_test_support::readiness_blocked_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        super::lifecycle_test_support::lifecycle_guards(),
    )
    .expect("request");

    assert_rejection(
        request,
        TaskTransitionRejectionReason::ReadinessNotSatisfied,
    );
}

#[test]
fn task_lifecycle_cancel_without_authority_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending),
        TaskState::Cancelled,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        None,
        Vec::new(),
        Some(super::lifecycle_test_support::ready_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        super::lifecycle_test_support::lifecycle_guards(),
    )
    .expect("request");

    assert_rejection(request, TaskTransitionRejectionReason::MissingAuthority);
}

#[test]
fn task_lifecycle_cancel_without_reason_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending),
        TaskState::Cancelled,
        None,
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        Vec::new(),
        Some(super::lifecycle_test_support::ready_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        super::lifecycle_test_support::lifecycle_guards(),
    )
    .expect("request");

    assert_rejection(request, TaskTransitionRejectionReason::MissingReason);
}

#[test]
fn task_lifecycle_complete_without_required_evidence_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::InProgress),
        TaskState::Completed,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        Vec::new(),
        Some(super::lifecycle_test_support::ready_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        TaskLifecycleGuards::new(
            Some(StateSequence::new(1).expect("sequence")),
            true,
            true,
            true,
            true,
            true,
            false,
            true,
            Some(TaskFailureCode::new("FAIL-CODE").expect("failure code")),
            Some(TaskFailureCategory::new("Operational").expect("failure category")),
        ),
    )
    .expect("request");

    assert_rejection(request, TaskTransitionRejectionReason::MissingEvidence);
}

#[test]
fn task_lifecycle_fail_without_failure_code_is_rejected() {
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::InProgress),
        TaskState::Failed,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        Vec::new(),
        Some(super::lifecycle_test_support::ready_decision()),
        Some(super::lifecycle_test_support::accepted_assignment()),
        TaskLifecycleGuards::new(
            Some(StateSequence::new(1).expect("sequence")),
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            None,
            Some(TaskFailureCategory::new("Operational").expect("failure category")),
        ),
    )
    .expect("request");

    assert_rejection(request, TaskTransitionRejectionReason::FailureCodeRequired);
}

fn assert_rejection(request: TaskTransitionRequest, reason: TaskTransitionRejectionReason) {
    let decision = TaskTransitionControl::evaluate(&request);

    match decision {
        TaskTransitionDecision::Rejected(rejected) => {
            assert_eq!(rejected.reason(), reason);
            assert_eq!(
                rejected
                    .current_task_state_snapshot()
                    .state_sequence()
                    .value(),
                1
            );
        }
        other => panic!("expected rejected transition, got {other:?}"),
    }
}
