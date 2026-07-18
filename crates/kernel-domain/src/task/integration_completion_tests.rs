use super::{
    TaskCompletionOutcome, TaskState, TaskTransitionDecision, TaskTransitionRejectionReason,
};

#[test]
fn integration_definition_to_instance_binding_is_preserved() {
    let task_instance = super::integration_flow_support::completion_happy_path().task_instance;

    assert_eq!(task_instance.task_state(), TaskState::Pending);
    assert_eq!(
        task_instance.task_definition_snapshot_reference(),
        super::outcome_test_support::task_instance().task_definition_snapshot_reference()
    );
}

#[test]
fn integration_full_completion_happy_path_reaches_archived() {
    let flow = super::integration_flow_support::completion_happy_path();

    assert!(matches!(
        flow.readiness_decision,
        super::TaskReadinessDecision::Ready(_)
    ));
    assert!(matches!(
        flow.start_decision,
        TaskTransitionDecision::Allowed(_)
    ));
    assert!(matches!(
        flow.completion_outcome,
        TaskCompletionOutcome::Accepted(_)
    ));
    assert!(matches!(
        flow.completed_decision,
        TaskTransitionDecision::Allowed(_)
    ));
    assert!(matches!(
        flow.archived_decision,
        TaskTransitionDecision::Allowed(_)
    ));
    assert_eq!(
        super::integration_test_support::allowed_snapshot(&flow.archived_decision).task_state(),
        TaskState::Archived
    );
}

#[test]
fn integration_completion_accepted_does_not_mutate_lifecycle() {
    let snapshot = super::integration_test_support::allowed_snapshot(
        &super::integration_flow_support::completion_happy_path().start_decision,
    );
    let outcome = TaskCompletionOutcome::Accepted(
        match super::TaskCompletionControl::evaluate(&super::TaskCompletionValidationRequest::new(
            super::outcome_test_support::task_instance(),
            snapshot.clone(),
            super::outcome_test_support::completion_result(
                super::outcome_test_support::valid_outputs(),
                super::outcome_test_support::required_evidence_set(
                    super::outcome_test_support::evidence_requirement(),
                ),
                vec![super::outcome_test_support::completion_requirement()],
            ),
            None,
        )) {
            TaskCompletionOutcome::Accepted(completion) => completion,
            _ => panic!("expected accepted completion"),
        },
    );

    assert_eq!(snapshot.task_state(), TaskState::InProgress);
    assert!(matches!(outcome, TaskCompletionOutcome::Accepted(_)));
}

#[test]
fn integration_completion_acceptance_supports_explicit_completed_transition() {
    let flow = super::integration_flow_support::completion_happy_path();

    assert!(matches!(
        flow.completion_outcome,
        TaskCompletionOutcome::Accepted(_)
    ));
    assert!(matches!(
        flow.completed_decision,
        TaskTransitionDecision::Allowed(_)
    ));
}

#[test]
fn integration_completed_archives_correctly() {
    let flow = super::integration_flow_support::completion_happy_path();
    let archived_snapshot =
        super::integration_test_support::allowed_snapshot(&flow.archived_decision);

    assert_eq!(archived_snapshot.task_state(), TaskState::Archived);
    assert_eq!(archived_snapshot.state_sequence().value(), 4);
}

#[test]
fn integration_missing_completion_evidence_prevents_valid_completion_outcome() {
    let start_snapshot = super::integration_test_support::allowed_snapshot(
        &super::integration_flow_support::completion_happy_path().start_decision,
    );
    let outcome =
        super::TaskCompletionControl::evaluate(&super::TaskCompletionValidationRequest::new(
            super::outcome_test_support::task_instance(),
            start_snapshot,
            super::outcome_test_support::completion_result(
                super::outcome_test_support::valid_outputs(),
                super::TaskEvidenceSet::new(
                    super::integration_test_support::task_instance_reference(),
                    Vec::new(),
                )
                .expect("evidence set"),
                vec![super::outcome_test_support::completion_requirement()],
            ),
            None,
        ));

    assert!(matches!(
        outcome,
        TaskCompletionOutcome::Rejected(rejected)
            if rejected.reason() == super::TaskCompletionRejectionReason::MissingRequiredEvidence
    ));
}

#[test]
fn integration_blocked_readiness_rejects_start_transition() {
    let blocked = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![super::TaskReadinessEvidence::RequiredInputAvailable],
    );
    let decision =
        super::TaskTransitionControl::evaluate(&super::integration_test_support::start_request(
            super::integration_test_support::pending_snapshot(),
            blocked,
            super::integration_test_support::accepted_assignment(),
            false,
        ));

    assert!(matches!(
        decision,
        TaskTransitionDecision::Rejected(rejected)
            if rejected.reason() == TaskTransitionRejectionReason::ReadinessNotSatisfied
    ));
}
