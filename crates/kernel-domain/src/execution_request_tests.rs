use crate::authorization::AuthorizationDecisionOutcome;
use crate::execution_test_support::{
    authorization_reference, execution_session_id, readiness_blocked, task_instance_reference,
    task_state_snapshot,
};
use crate::{ExecutionRequest, TaskState};

#[test]
fn execution_request_constructs_with_ready_in_progress_task_k8_001() {
    let request = crate::execution_test_support::execution_request();
    assert_eq!(
        request.task_instance_reference(),
        &task_instance_reference()
    );
    assert_eq!(request.execution_session_id(), &execution_session_id());
}

#[test]
fn execution_request_rejects_blocked_readiness_k8_001() {
    let error = ExecutionRequest::new(
        execution_session_id(),
        task_instance_reference(),
        task_state_snapshot(TaskState::InProgress),
        readiness_blocked(),
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        crate::request::TimeReference::new("2026-07-18T00:20:00Z").expect("time"),
    )
    .expect_err("blocked readiness must fail");
    assert!(error.to_string().contains("ready task readiness decision"));
}

#[test]
fn execution_request_rejects_denied_authorization_k8_001() {
    let error = ExecutionRequest::new(
        execution_session_id(),
        task_instance_reference(),
        task_state_snapshot(TaskState::InProgress),
        crate::execution_test_support::readiness_ready(),
        authorization_reference(AuthorizationDecisionOutcome::Deny),
        crate::request::TimeReference::new("2026-07-18T00:20:00Z").expect("time"),
    )
    .expect_err("denied auth must fail");
    assert!(error.to_string().contains("allowed authorization decision"));
}

#[test]
fn execution_request_rejects_non_in_progress_snapshot_k8_001() {
    let error = ExecutionRequest::new(
        execution_session_id(),
        task_instance_reference(),
        task_state_snapshot(TaskState::Pending),
        crate::execution_test_support::readiness_ready(),
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        crate::request::TimeReference::new("2026-07-18T00:20:00Z").expect("time"),
    )
    .expect_err("pending state must fail");
    assert!(error.to_string().contains("InProgress"));
}
