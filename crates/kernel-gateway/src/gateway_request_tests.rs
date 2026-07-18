use kernel_domain::{AuthorizationDecisionOutcome, CorrelationId};

use crate::gateway_test_support::{
    authentication_context, authorization_binding, gateway_api_version, gateway_audit_reference,
    gateway_rate_governance_reference, mismatched_authentication_context, operation_memory_capture,
    ownership_path, project_scope_mismatch_path, requested_at, workspace_scope_mismatch_path,
};
use crate::GatewayRequestContext;

#[test]
fn gateway_request_context_constructs_with_matching_auth_scope_and_audit_k10_004() {
    let context = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect("valid context");
    assert_eq!(
        context.gateway_operation_reference().resource_segment(),
        "memory"
    );
}

#[test]
fn gateway_request_context_rejects_authentication_authorization_mismatch_k10_004() {
    let error = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        mismatched_authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect_err("principal mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::AuthorizationDenied);
}

#[test]
fn gateway_request_context_rejects_denied_authorization_k10_004() {
    let error = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            ownership_path(),
            AuthorizationDecisionOutcome::Deny,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect_err("denied authorization must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::AuthorizationDenied);
}

#[test]
fn gateway_request_context_rejects_workspace_scope_mismatch_k10_004() {
    let error = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            workspace_scope_mismatch_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect_err("workspace mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::ScopeMismatch);
}

#[test]
fn gateway_request_context_rejects_project_scope_mismatch_k10_004() {
    let error = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            project_scope_mismatch_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect_err("project mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::ScopeMismatch);
}

#[test]
fn gateway_request_context_rejects_correlation_mismatch_in_audit_k10_004() {
    let audit = crate::GatewayAuditReference::new(
        kernel_domain::EventTraceReference::new("gateway.audit.trace.000009").expect("trace"),
        Some(CorrelationId::new("CX-COR-000009").expect("correlation")),
        vec![kernel_domain::AuditEvidenceId::new("CX-AUD-000009").expect("audit")],
    )
    .expect("audit");
    let error = GatewayRequestContext::new(
        gateway_api_version(),
        operation_memory_capture(),
        authentication_context(),
        authorization_binding(
            "memory",
            "gateway.memory.record.000001",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path(),
        crate::gateway_test_support::correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        audit,
    )
    .expect_err("audit correlation mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidRequest);
}
