use crate::application_test_support::{
    application_api_version, application_audit_reference, application_capability_declaration,
    application_identity, application_request_id, application_session_reference, correlation_id,
    gateway_authentication_context, mismatched_gateway_authentication_context, ownership_path,
    project_scope_mismatch_path, query_application_request_context, studio_selection_context,
    time_reference, workspace_scope_mismatch_path,
};
use crate::{ApplicationErrorCode, ApplicationRequestContext};
use kernel_domain::{AuthorizationDecisionOutcome, EventTraceReference};

#[test]
fn application_request_context_preserves_causation_and_audit_order_k12_006() {
    let context = query_application_request_context();
    assert_eq!(
        context.causation_reference().expect("causation").as_str(),
        "application.cause.trace.000001"
    );
    let evidence = context.application_audit_reference().audit_evidence_ids();
    assert_eq!(evidence[0].as_str(), "CX-AUD-000003");
    assert_eq!(evidence[1].as_str(), "CX-AUD-000004");
}

#[test]
fn application_request_context_rejects_workspace_scope_mismatch_k12_007() {
    let error = ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        crate::application_test_support::authorization_binding(
            "gateway",
            "gateway.status",
            workspace_scope_mismatch_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        studio_selection_context(),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect_err("workspace mismatch must fail");
    assert_eq!(error.code(), ApplicationErrorCode::ScopeMismatch);
}

#[test]
fn application_request_context_rejects_project_scope_mismatch_k12_007() {
    let error = ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        crate::application_test_support::authorization_binding(
            "gateway",
            "gateway.status",
            project_scope_mismatch_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        studio_selection_context(),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect_err("project mismatch must fail");
    assert_eq!(error.code(), ApplicationErrorCode::ScopeMismatch);
}

#[test]
fn application_request_context_rejects_authorization_evidence_mismatch_k12_006() {
    let error = ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        mismatched_gateway_authentication_context(),
        crate::application_test_support::authorization_binding(
            "gateway",
            "gateway.status",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        studio_selection_context(),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect_err("auth mismatch must fail");
    assert_eq!(
        error.code(),
        ApplicationErrorCode::AuthorizationEvidenceMismatch
    );
}

#[test]
fn application_request_context_rejects_session_application_mismatch_k12_006() {
    let wrong_session = crate::ApplicationSessionReference::new(
        "application.session.000001",
        kernel_domain::EnglishNamespace::new(
            "application_identifier",
            "application.integration.secondary",
        )
        .expect("id"),
        ownership_path(),
        correlation_id(),
        time_reference(),
        crate::application_test_support::later_time_reference(),
        crate::ApplicationSessionStatusReference::new("application.session.active")
            .expect("status"),
    )
    .expect("session");
    let error = ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        crate::application_test_support::authorization_binding(
            "gateway",
            "gateway.status",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        studio_selection_context(),
        Some(wrong_session),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect_err("session mismatch must fail");
    assert_eq!(
        error.code(),
        ApplicationErrorCode::SessionApplicationMismatch
    );
}

#[test]
fn application_request_context_rejects_gateway_authorization_denial_k12_006() {
    let error = ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        crate::application_test_support::authorization_binding(
            "gateway",
            "gateway.status",
            ownership_path(),
            AuthorizationDecisionOutcome::Deny,
        ),
        studio_selection_context(),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect_err("denial cannot be overridden");
    assert_eq!(
        error.code(),
        ApplicationErrorCode::GatewayAuthorizationDenied
    );
}
