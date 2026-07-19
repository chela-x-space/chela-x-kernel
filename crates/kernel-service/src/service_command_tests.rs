use crate::service_test_support::{
    application_command_intent, command_service_request_context, query_application_request_context,
    service_command_capability, service_query_capability,
};
use crate::{ServiceCommandIntent, ServiceErrorCode, ServiceRequestContext};

#[test]
fn service_command_intent_accepts_valid_application_command_k13_004() {
    let intent = crate::service_test_support::service_command_intent();
    assert_eq!(
        intent
            .application_command_intent()
            .application_request_context()
            .application_request_id()
            .as_str(),
        "application.request.000001"
    );
}

#[test]
fn service_command_intent_rejects_capability_mismatch_k13_004() {
    let error = ServiceCommandIntent::new(
        command_service_request_context(),
        service_query_capability(),
        application_command_intent(),
    )
    .expect_err("query capability must not satisfy command intent");
    assert_eq!(error.code(), ServiceErrorCode::CapabilityMismatch);
}

#[test]
fn service_command_intent_rejects_application_request_mismatch_k13_010() {
    let mismatched_context = ServiceRequestContext::new(
        crate::service_test_support::service_api_version(),
        crate::service_test_support::service_request_id(),
        crate::service_test_support::service_identity(),
        crate::service_test_support::service_capability_declaration(),
        query_application_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect("context");
    let error = ServiceCommandIntent::new(
        mismatched_context,
        service_command_capability(),
        application_command_intent(),
    )
    .expect_err("application request mismatch must fail");
    assert_eq!(error.code(), ServiceErrorCode::ApplicationRequestMismatch);
}
