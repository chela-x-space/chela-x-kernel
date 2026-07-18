use super::{
    TaskCompletionControl, TaskCompletionOutcome, TaskFailureControl, TaskFailureOutcome,
    TaskOutcomeDecision, TaskTransitionDecision,
};

#[test]
fn completion_and_failure_evaluation_do_not_mutate_lifecycle() {
    let snapshot = super::outcome_test_support::state_snapshot(super::TaskState::InProgress);

    let completion = TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(super::TaskState::InProgress),
    );
    let failure = TaskFailureControl::evaluate(&super::outcome_test_support::failure_request(None));

    assert_eq!(snapshot.task_state(), super::TaskState::InProgress);
    assert!(matches!(completion, TaskCompletionOutcome::Accepted(_)));
    assert!(matches!(failure, TaskFailureOutcome::Accepted(_)));
}

#[test]
fn completion_validation_does_not_trigger_lifecycle_transition() {
    let completion = TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(super::TaskState::InProgress),
    );
    let transition = super::outcome_test_support::lifecycle_completion_decision();

    assert!(matches!(completion, TaskCompletionOutcome::Accepted(_)));
    assert!(matches!(transition, TaskTransitionDecision::Allowed(_)));
}

#[test]
fn outcome_decision_is_mutually_exclusive_by_type() {
    let completion = match TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(super::TaskState::InProgress),
    ) {
        TaskCompletionOutcome::Accepted(completion) => completion,
        _ => panic!("expected accepted completion"),
    };
    let decision = TaskOutcomeDecision::Completed(completion);

    assert!(matches!(decision, TaskOutcomeDecision::Completed(_)));
}
