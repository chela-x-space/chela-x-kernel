use crate::execution_test_support::{
    execution_audit_reference, execution_context, execution_evidence_binding, execution_request,
    execution_session_id, task_instance_reference,
};
use crate::{ExecutionAuditReference, ExecutionSession};

#[test]
fn execution_session_constructs_with_continuous_identities_k8_003() {
    let session = crate::execution_test_support::execution_session();
    assert_eq!(session.execution_session_id(), &execution_session_id());
    assert_eq!(
        session.execution_request().task_instance_reference(),
        &task_instance_reference()
    );
}

#[test]
fn execution_session_rejects_audit_identity_mismatch_k8_003() {
    let error = ExecutionSession::new(
        execution_request(),
        execution_context(),
        execution_evidence_binding(),
        ExecutionAuditReference::new(
            crate::ExecutionSessionId::new("execution.session-mismatch").expect("id"),
            None,
            vec![crate::AuditEvidenceId::new("CX-AUD-000009").expect("audit")],
        )
        .expect("audit"),
        crate::request::TimeReference::new("2026-07-18T00:21:00Z").expect("time"),
    )
    .expect_err("mismatch must fail");
    assert!(error
        .to_string()
        .contains("identifiers must remain continuous"));
}

#[test]
fn execution_session_preserves_evidence_and_audit_snapshots_k8_003() {
    let session = crate::execution_test_support::execution_session();
    assert_eq!(
        session
            .execution_evidence_binding()
            .task_instance_reference(),
        &task_instance_reference()
    );
    assert_eq!(
        session.execution_audit_reference(),
        &execution_audit_reference()
    );
}
