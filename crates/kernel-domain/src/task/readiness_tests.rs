use super::{
    TaskReadinessBlocker, TaskReadinessControl, TaskReadinessDecision, TaskReadinessInput,
    TaskReadinessRequirement, TaskState,
};

#[test]
fn task_readiness_valid_ready_result_is_returned() {
    let decision = TaskReadinessControl::evaluate(&super::readiness_test_support::ready_input());

    match decision {
        TaskReadinessDecision::Ready(ready) => {
            assert_eq!(ready.validated_evidence().len(), 4);
        }
        other => panic!("expected ready decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_missing_required_input_produces_expected_reason() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(super::readiness_test_support::accepted_assignment()),
        vec![TaskReadinessRequirement::RequiredInputAvailable],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::MissingRequiredInput]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_assigned_but_not_otherwise_ready_remains_blocked() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(super::readiness_test_support::accepted_assignment()),
        vec![TaskReadinessRequirement::DependenciesComplete],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::DependencyIncomplete]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_accepted_assignment_does_not_automatically_imply_ready() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        Some(super::readiness_test_support::ownership()),
        Some(super::readiness_test_support::accepted_assignment()),
        vec![TaskReadinessRequirement::EvidencePrerequisitesAvailable],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[TaskReadinessBlocker::MissingRequiredEvidence]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_missing_owner_produces_expected_reason() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        None,
        Some(super::readiness_test_support::accepted_assignment()),
        vec![TaskReadinessRequirement::OwnershipRequired],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(blocked.blockers(), &[TaskReadinessBlocker::MissingOwner]);
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}

#[test]
fn task_readiness_reason_order_is_preserved() {
    let input = TaskReadinessInput::new(
        super::readiness_test_support::task_instance_reference(),
        TaskState::Pending,
        None,
        None,
        None,
        vec![
            TaskReadinessRequirement::OwnershipRequired,
            TaskReadinessRequirement::AssignmentRequired,
            TaskReadinessRequirement::RequiredInputAvailable,
        ],
        vec![],
        None,
    );

    match TaskReadinessControl::evaluate(&input) {
        TaskReadinessDecision::Blocked(blocked) => {
            assert_eq!(
                blocked.blockers(),
                &[
                    TaskReadinessBlocker::MissingOwner,
                    TaskReadinessBlocker::MissingAssignment,
                    TaskReadinessBlocker::MissingRequiredInput,
                ]
            );
        }
        other => panic!("expected blocked decision, got {other:?}"),
    }
}
