use crate::adapter_test_support::{
    adapter_command_capability, adapter_query_capability, adapter_query_request_context,
    service_command_intent,
};
use crate::{AdapterCommandIntent, AdapterErrorCode};

#[test]
fn adapter_command_intent_accepts_valid_service_command_k14_004() {
    let intent = crate::adapter_test_support::adapter_command_intent();
    assert_eq!(
        intent
            .adapter_request_context()
            .adapter_identity()
            .adapter_identifier(),
        "adapter.integration.primary"
    );
}

#[test]
fn adapter_command_intent_rejects_capability_mismatch_k14_009() {
    let error = AdapterCommandIntent::new(
        crate::adapter_test_support::adapter_command_request_context(),
        adapter_query_capability(),
        service_command_intent(),
    )
    .expect_err("query capability must not satisfy command intent");
    assert_eq!(error.code(), AdapterErrorCode::CapabilityMismatch);
}

#[test]
fn adapter_command_intent_rejects_request_mismatch_k14_010() {
    let error = AdapterCommandIntent::new(
        adapter_query_request_context(),
        adapter_command_capability(),
        service_command_intent(),
    )
    .expect_err("mismatched request contexts must fail");
    assert_eq!(
        error.code(),
        AdapterErrorCode::AdapterRequestIdentityMismatch
    );
}
