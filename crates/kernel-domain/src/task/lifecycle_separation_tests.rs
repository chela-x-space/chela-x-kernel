use crate::state::{
    TransitionAuthorityReference, TransitionEvidenceReference, TransitionReasonReference,
};

use super::{
    TaskAssignmentStatus, TaskLifecycleGuards, TaskState, TaskTransitionControl,
    TaskTransitionDecision, TaskTransitionRequest,
};

#[test]
fn task_lifecycle_transition_does_not_mutate_assignment() {
    let assignment = super::lifecycle_test_support::accepted_assignment();
    let assignment_before = assignment.clone();
    let request = super::lifecycle_test_support::transition_request(
        TaskState::Pending,
        TaskState::InProgress,
    );

    let _decision = TaskTransitionControl::evaluate(&request);

    assert_eq!(assignment, assignment_before);
    assert_eq!(
        assignment.task_assignment_status(),
        TaskAssignmentStatus::Accepted
    );
}

#[test]
fn task_lifecycle_transition_does_not_mutate_ownership_priority_or_readiness() {
    let ownership = super::readiness_test_support::ownership();
    let ownership_before = ownership.clone();
    let readiness = super::lifecycle_test_support::ready_decision();
    let readiness_before = readiness.clone();
    let priority = super::TaskPriority::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending)
            .task_instance_reference()
            .clone(),
        super::TaskPriorityClass::new("Explicit").expect("class"),
        super::TaskPriorityValue::new(5).expect("value"),
    );
    let priority_before = priority.clone();

    let _decision =
        TaskTransitionControl::evaluate(&super::lifecycle_test_support::transition_request(
            TaskState::Pending,
            TaskState::InProgress,
        ));

    assert_eq!(ownership, ownership_before);
    assert_eq!(readiness, readiness_before);
    assert_eq!(priority, priority_before);
}

#[test]
fn task_lifecycle_transition_does_not_infer_assignment_from_assigned_status() {
    let assigned_only = super::TaskAssignment::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending)
            .task_instance_reference()
            .clone(),
        super::lifecycle_test_support::accepted_assignment()
            .task_assignee()
            .cloned(),
        TaskAssignmentStatus::Assigned,
        Some(TransitionAuthorityReference::new("assignment.authority").expect("authority")),
        None,
    )
    .expect("assignment");
    let request = TaskTransitionRequest::new(
        super::lifecycle_test_support::task_state_snapshot(TaskState::Pending),
        TaskState::InProgress,
        Some(TransitionReasonReference::new("transition.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("transition.evidence").expect("evidence")],
        Some(super::lifecycle_test_support::ready_decision()),
        Some(assigned_only),
        TaskLifecycleGuards::new(
            Some(crate::StateSequence::new(1).expect("sequence")),
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            Some(super::TaskFailureCode::new("FAIL-CODE").expect("failure code")),
            Some(super::TaskFailureCategory::new("Operational").expect("failure category")),
        ),
    )
    .expect("request");

    match TaskTransitionControl::evaluate(&request) {
        TaskTransitionDecision::Rejected(rejected) => {
            assert_eq!(
                rejected.reason(),
                super::TaskTransitionRejectionReason::AssignmentRequired
            );
        }
        other => panic!("expected rejected transition, got {other:?}"),
    }
}

#[test]
fn task_lifecycle_vocabulary_remains_separate_from_readiness_and_assignment() {
    assert_eq!(TaskState::Pending.as_str(), "Pending");
    assert_eq!(TaskState::Archived.as_str(), "Archived");
}
