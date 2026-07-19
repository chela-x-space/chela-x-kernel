use crate::service_test_support::{
    application_query_intent, command_application_request_context, query_service_request_context,
    service_command_capability, service_query_capability,
};
use crate::{ServiceErrorCode, ServiceQueryIntent, ServiceRequestContext};

#[test]
fn service_query_intent_accepts_valid_application_query_k13_005() {
    let intent = crate::service_test_support::service_query_intent();
    assert_eq!(
        intent
            .application_query_intent()
            .application_request_context()
            .application_request_id()
            .as_str(),
        "application.request.000001"
    );
}

#[test]
fn service_query_intent_rejects_capability_mismatch_k13_005() {
    let error = ServiceQueryIntent::new(
        query_service_request_context(),
        service_command_capability(),
        application_query_intent(),
    )
    .expect_err("command capability must not satisfy query intent");
    assert_eq!(error.code(), ServiceErrorCode::CapabilityMismatch);
}

#[test]
fn service_query_intent_rejects_application_request_mismatch_k13_010() {
    let mismatched_context = ServiceRequestContext::new(
        crate::service_test_support::service_api_version(),
        crate::service_test_support::service_request_id(),
        crate::service_test_support::service_identity(),
        crate::service_test_support::service_capability_declaration(),
        command_application_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect("context");
    let error = ServiceQueryIntent::new(
        mismatched_context,
        service_query_capability(),
        application_query_intent(),
    )
    .expect_err("application request mismatch must fail");
    assert_eq!(error.code(), ServiceErrorCode::ApplicationRequestMismatch);
}
