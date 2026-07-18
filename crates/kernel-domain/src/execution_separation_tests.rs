use crate::execution_test_support::{accepted_completion, accepted_failure, execution_session};
use crate::{ExecutionOutcome, TaskState};

#[test]
fn execution_outcome_does_not_mutate_task_completion_or_failure_k8_008() {
    let completion = accepted_completion();
    let failure = accepted_failure();
    let original_task = completion
        .task_completion_result()
        .task_instance_reference()
        .clone();
    ExecutionOutcome::succeeded(
        execution_session(),
        completion.clone(),
        crate::request::TimeReference::new("2026-07-18T00:38:00Z").expect("time"),
    )
    .expect("success");
    ExecutionOutcome::failed(
        execution_session(),
        failure.clone(),
        crate::request::TimeReference::new("2026-07-18T00:39:00Z").expect("time"),
    )
    .expect("failure");
    assert_eq!(
        completion
            .task_completion_result()
            .task_instance_reference(),
        &original_task
    );
    assert_eq!(failure.task_instance_reference(), &original_task);
}

#[test]
fn execution_session_does_not_mutate_task_lifecycle_facts_k8_008() {
    let snapshot = crate::execution_test_support::task_state_snapshot(TaskState::InProgress);
    let sequence = snapshot.state_sequence();
    let state = snapshot.task_state();
    let _session = execution_session();
    assert_eq!(snapshot.state_sequence(), sequence);
    assert_eq!(snapshot.task_state(), state);
}

#[test]
fn execution_concern_separation_keeps_runtime_and_task_distinct_k8_008() {
    let session = execution_session();
    assert_ne!(
        session
            .execution_context()
            .runtime_state_snapshot()
            .runtime_id()
            .as_str(),
        session
            .execution_request()
            .task_instance_reference()
            .task_instance_id()
            .as_str()
    );
}
