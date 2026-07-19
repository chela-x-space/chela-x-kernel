use crate::service_test_support::{
    application_query_response_envelope, command_service_request_context,
};
use crate::{ServiceErrorCode, ServiceResponseEnvelope, ServiceResponseKind, ServiceResponseStatusReference};

#[test]
fn service_response_envelope_preserves_application_response_k13_007() {
    let response = crate::service_test_support::service_response_envelope();
    assert_eq!(response.service_response_kind(), ServiceResponseKind::View);
}

#[test]
fn service_response_envelope_rejects_request_response_mismatch_k13_010() {
    let error = ServiceResponseEnvelope::new(
        &command_service_request_context(),
        application_query_response_envelope(),
        ServiceResponseStatusReference::new("service.response.complete").expect("status"),
        "2026-07-19T00:10:00Z",
    )
    .expect_err("mismatched responses must fail");
    assert_eq!(error.code(), ServiceErrorCode::ResponseRequestMismatch);
}
