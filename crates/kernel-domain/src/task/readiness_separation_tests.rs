use super::{
    TaskPriority, TaskPriorityClass, TaskPriorityValue, TaskReadinessBlocker, TaskReadinessControl,
    TaskReadinessDecision, TaskReadinessInput, TaskReadinessRejectionReason,
    TaskReadinessRequirement, TaskState,
};

use crate::authorization::AuthorizationDecisionOutcome;

#[test]
fn task_readiness_high_priority_does_not_imply_ready() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        Some(TaskPriority::new(
            super::readiness_test_support::task_instance_reference(),
            TaskPriorityClass::new("Explicit").expect("class"),
            TaskPriorityValue::new(9).expect("value"),
        )),
        Some(super::readiness_test_support::ownership()),
        None,
        vec![TaskReadinessRequirement::AssignmentRequired],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::MissingAssignment]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_terminal_task_state_is_blocked() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Completed,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(super::readiness_test_support::accepted_assignment()),
        vec![],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::TerminalTaskState]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_same_input_produces_same_decision() {
    let left = TaskReadinessControl::evaluate(&super::readiness_test_support::ready_input());
    let right = TaskReadinessControl::evaluate(&super::readiness_test_support::ready_input());

    assert_eq!(left, right);
}

#[test]
fn task_readiness_evaluation_does_not_mutate_task_or_assignment() {
    let task_instance = crate::task::instance_tests::minimal_task_instance_for_shared_tests();
    let assignment = super::readiness_test_support::accepted_assignment();
    let assignment_before = assignment.clone();
    let state_before = task_instance.task_state();
    let input = TaskReadinessInput::new(
        super::TaskInstanceReference::new(task_instance.task_instance_id().clone()),
        task_instance.task_state(),
        None,
        Some(super::readiness_test_support::ownership()),
        Some(assignment.clone()),
        vec![TaskReadinessRequirement::AcceptedAssignmentRequired],
        vec![],
        None,
    );

    let _decision = TaskReadinessControl::evaluate(&input);

    assert_eq!(task_instance.task_state(), state_before);
    assert_eq!(assignment, assignment_before);
}

#[test]
fn task_readiness_contradictory_requirements_are_rejected() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        None,
        vec![
            TaskReadinessRequirement::AssignmentRequired,
            TaskReadinessRequirement::LaterAssignmentPermitted,
        ],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Rejected(rejected) => {
            assert_eq!(
                rejected.reason(),
                TaskReadinessRejectionReason::ContradictoryRequirement
            );
        }
        other => panic!("expected rejected decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_authorization_denied_produces_expected_reason() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(super::readiness_test_support::accepted_assignment()),
        vec![TaskReadinessRequirement::AuthorizationAllowed],
        vec![super::TaskReadinessEvidence::AuthorizationAllowed],
        Some(AuthorizationDecisionOutcome::Deny),
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::AuthorizationDenied]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_task_state_contains_no_ready_or_blocked_variant() {
    assert_eq!(TaskState::Pending.as_str(), "Pending");
    assert_eq!(TaskState::Completed.as_str(), "Completed");
}
