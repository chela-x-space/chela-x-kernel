use super::{TaskFailureOutcome, TaskState, TaskTransitionDecision};

#[test]
fn integration_full_failure_happy_path_reaches_archived() {
    let flow = super::integration_flow_support::failure_happy_path();

    assert!(matches!(
        flow.readiness_decision,
        super::TaskReadinessDecision::Ready(_)
    ));
    assert!(matches!(
        flow.start_decision,
        TaskTransitionDecision::Allowed(_)
    ));
    assert!(matches!(
        flow.failure_outcome,
        TaskFailureOutcome::Accepted(_)
    ));
    assert!(matches!(
        flow.failed_decision,
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
fn integration_failure_accepted_does_not_mutate_lifecycle() {
    let start_snapshot = super::integration_test_support::allowed_snapshot(
        &super::integration_flow_support::failure_happy_path().start_decision,
    );
    let outcome = super::TaskFailureControl::evaluate(&super::TaskFailureValidationRequest::new(
        super::outcome_test_support::task_instance(),
        start_snapshot.clone(),
        super::outcome_test_support::failure(
            super::outcome_test_support::failure_evidence_set(),
            Some(
                super::TaskFailurePolicyReference::new("task.failure.policy.demo").expect("policy"),
            ),
        ),
        None,
    ));

    assert_eq!(start_snapshot.task_state(), TaskState::InProgress);
    assert!(matches!(outcome, TaskFailureOutcome::Accepted(_)));
}

#[test]
fn integration_failure_acceptance_supports_explicit_failed_transition() {
    let flow = super::integration_flow_support::failure_happy_path();

    assert!(matches!(
        flow.failure_outcome,
        TaskFailureOutcome::Accepted(_)
    ));
    assert!(matches!(
        flow.failed_decision,
        TaskTransitionDecision::Allowed(_)
    ));
}

#[test]
fn integration_failed_archives_correctly() {
    let flow = super::integration_flow_support::failure_happy_path();
    let archived_snapshot =
        super::integration_test_support::allowed_snapshot(&flow.archived_decision);

    assert_eq!(archived_snapshot.task_state(), TaskState::Archived);
    assert_eq!(archived_snapshot.state_sequence().value(), 4);
}

#[test]
fn integration_failure_record_preserves_code_category_evidence_and_policy() {
    let flow = super::integration_flow_support::failure_happy_path();
    let failure = match flow.failure_outcome {
        TaskFailureOutcome::Accepted(failure) => failure,
        _ => panic!("expected accepted failure"),
    };

    assert_eq!(failure.task_failure_code().as_str(), "task.failure.timeout");
    assert_eq!(
        failure.task_failure_category().as_str(),
        "task.failure_category.execution"
    );
    assert_eq!(
        failure.task_failure_evidence_set().task_evidences().len(),
        1
    );
    assert_eq!(
        failure
            .task_failure_policy_reference()
            .expect("policy")
            .as_str(),
        "task.failure.policy.demo"
    );
}
