use crate::service_test_support::{
    query_application_request_context, service_api_version, service_capability_declaration,
    service_identity, service_request_id,
};
use crate::{ServiceCapabilityDeclaration, ServiceErrorCode, ServiceRequestContext};

#[test]
fn service_request_context_accepts_valid_application_context_k13_006() {
    let context = ServiceRequestContext::new(
        service_api_version(),
        service_request_id(),
        service_identity(),
        service_capability_declaration(),
        query_application_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect("context");
    assert_eq!(
        context
            .application_request_context()
            .application_request_id()
            .as_str(),
        "application.request.000001"
    );
}

#[test]
fn service_request_context_rejects_capability_version_mismatch_k13_006() {
    let mismatched_declaration = ServiceCapabilityDeclaration::new(
        crate::ServiceApiVersion::new("2026.07.18").expect("version"),
        vec![
            crate::service_test_support::service_command_capability(),
            crate::service_test_support::service_query_capability(),
        ],
        true,
        true,
    )
    .expect("capability");
    let error = ServiceRequestContext::new(
        service_api_version(),
        service_request_id(),
        service_identity(),
        mismatched_declaration,
        query_application_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect_err("mismatched capability versions must fail");
    assert_eq!(error.code(), ServiceErrorCode::CapabilityMismatch);
}
