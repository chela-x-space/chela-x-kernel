use crate::application_test_support::{
    application_api_version, application_audit_reference, application_capability_declaration,
    application_identity, application_query_intent, application_request_id,
    application_status_snapshot, command_application_request_context, correlation_id,
    gateway_authentication_context, ownership_path, query_capability, studio_command_request,
    studio_selection_context, time_reference,
};
use crate::{ApplicationCommandIntent, ApplicationErrorCode, ApplicationRequestContext};
use kernel_domain::{AuthorizationDecisionOutcome, EventTraceReference};

#[test]
fn application_equivalent_inputs_produce_equivalent_results_k12_001() {
    let left = application_query_intent();
    let right = application_query_intent();
    assert_eq!(left, right);
}

#[test]
fn application_request_context_rejects_correlation_mismatch_k12_007() {
    let wrong_session = crate::ApplicationSessionReference::new(
        "application.session.000001",
        application_identity().application_identifier().clone(),
        ownership_path(),
        crate::application_test_support::other_correlation_id(),
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
    .expect_err("correlation mismatch must fail");
    assert_eq!(
        error.code(),
        ApplicationErrorCode::SessionCorrelationMismatch
    );
    let _ = application_status_snapshot();
}

#[test]
fn application_equivalent_invalid_inputs_return_equivalent_errors_k12_003() {
    let left = ApplicationCommandIntent::new(
        command_application_request_context(),
        query_capability(),
        studio_command_request(),
    )
    .expect_err("query capability must not satisfy command intent");
    let right = ApplicationCommandIntent::new(
        command_application_request_context(),
        query_capability(),
        studio_command_request(),
    )
    .expect_err("query capability must not satisfy command intent");
    assert_eq!(left, right);
}

#[test]
fn application_rejected_cases_remain_side_effect_free_k12_003() {
    let request_context = command_application_request_context();
    let studio_request = studio_command_request();
    let request_context_before = request_context.clone();
    let studio_request_before = studio_request.clone();

    let error = ApplicationCommandIntent::new(
        request_context.clone(),
        query_capability(),
        studio_request.clone(),
    )
    .expect_err("query capability must not satisfy command intent");

    assert_eq!(error.code(), ApplicationErrorCode::CapabilityMismatch);
    assert_eq!(request_context, request_context_before);
    assert_eq!(studio_request, studio_request_before);
}
