use crate::execution_test_support::{
    accepted_failure, execution_audit_reference, execution_context, execution_evidence_binding,
    execution_request, execution_session,
};
use crate::{ExecutionOutcome, ExecutionValidation, TaskState};

#[test]
fn execution_validation_accepts_reference_only_audit_binding_k8_007() {
    ExecutionValidation::validate_reference_only_audit(
        &execution_audit_reference(),
        &execution_evidence_binding(),
    )
    .expect("reference-only audit compatibility");
}

#[test]
fn execution_validation_accepts_failed_snapshot_continuity_k8_007() {
    ExecutionValidation::validate_failed_snapshot(
        &crate::execution_test_support::task_state_snapshot(TaskState::InProgress),
        &accepted_failure(),
        &crate::request::TimeReference::new("2026-07-18T00:36:00Z").expect("time"),
    )
    .expect("continuity");
}

#[test]
fn execution_validation_accepts_request_context_and_session_k8_008() {
    ExecutionValidation::validate_request(&execution_request()).expect("request");
    ExecutionValidation::validate_context(&execution_context()).expect("context");
    ExecutionValidation::validate_session(&execution_session()).expect("session");
}

#[test]
fn execution_failed_outcome_supports_explicit_failure_only_k8_008() {
    let outcome = ExecutionOutcome::failed(
        execution_session(),
        accepted_failure(),
        crate::request::TimeReference::new("2026-07-18T00:37:00Z").expect("time"),
    )
    .expect("failed");
    assert_eq!(
        outcome.execution_termination(),
        crate::ExecutionTermination::Failed
    );
}
