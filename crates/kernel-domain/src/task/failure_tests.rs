use super::{TaskFailureControl, TaskFailureOutcome, TaskFailureRejectionReason};

#[test]
fn task_failure_preserves_code_category_policy_and_evidence() {
    let failure = super::outcome_test_support::failure(
        super::outcome_test_support::failure_evidence_set(),
        Some(
            super::TaskFailurePolicyReference::new("task.failure.policy.demo").expect("policy"),
        ),
    );

    assert_eq!(failure.task_failure_code().as_str(), "task.failure.timeout");
    assert_eq!(
        failure.task_failure_category().as_str(),
        "task.failure_category.execution"
    );
    assert_eq!(
        failure.task_failure_policy_reference(),
        Some(
            &super::TaskFailurePolicyReference::new("task.failure.policy.demo").expect("policy")
        )
    );
    assert_eq!(failure.task_failure_evidence_set().task_evidences().len(), 1);
}

#[test]
fn task_failure_accepts_valid_failure_record() {
    let decision = TaskFailureControl::evaluate(&super::outcome_test_support::failure_request(
        None,
    ));

    assert!(matches!(decision, TaskFailureOutcome::Accepted(_)));
}

#[test]
fn task_failure_rejects_policy_mismatch() {
    let request = super::TaskFailureValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(super::TaskState::InProgress),
        super::outcome_test_support::failure(
            super::outcome_test_support::failure_evidence_set(),
            Some(
                super::TaskFailurePolicyReference::new("task.failure.policy.other")
                    .expect("policy"),
            ),
        ),
        None,
    );

    let decision = TaskFailureControl::evaluate(&request);
    assert!(matches!(
        decision,
        TaskFailureOutcome::Rejected(rejected)
            if rejected.reason() == TaskFailureRejectionReason::FailurePolicyMismatch
    ));
}

#[test]
fn task_failure_rejects_completion_failure_conflict() {
    let completion = match super::TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(super::TaskState::InProgress),
    ) {
        super::TaskCompletionOutcome::Accepted(completion) => completion,
        _ => panic!("expected accepted completion"),
    };
    let decision =
        TaskFailureControl::evaluate(&super::outcome_test_support::failure_request(Some(
            completion,
        )));

    assert!(matches!(
        decision,
        TaskFailureOutcome::Rejected(rejected)
            if rejected.reason() == TaskFailureRejectionReason::CompletionFailureConflict
    ));
}

#[test]
fn task_failure_rejects_task_instance_mismatch() {
    let other_evidence = super::TaskEvidenceSet::new(
        super::TaskInstanceReference::new(
            super::TaskInstanceId::new("task.instance.other").expect("instance id"),
        ),
        vec![super::outcome_test_support::task_evidence("task.evidence.other", None)],
    );
    let request = super::TaskFailureValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(super::TaskState::InProgress),
        super::outcome_test_support::failure(
            other_evidence.expect("evidence set"),
            Some(
                super::TaskFailurePolicyReference::new("task.failure.policy.demo")
                    .expect("policy"),
            ),
        ),
        None,
    );

    let decision = TaskFailureControl::evaluate(&request);
    assert!(matches!(
        decision,
        TaskFailureOutcome::Rejected(rejected)
            if rejected.reason() == TaskFailureRejectionReason::TaskInstanceMismatch
    ));
}
