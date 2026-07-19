use crate::adapter_test_support::{
    adapter_command_capability, alternate_service_query_intent, service_query_intent,
};
use crate::{AdapterErrorCode, AdapterQueryIntent};

#[test]
fn adapter_query_intent_accepts_valid_service_query_k14_005() {
    let intent = crate::adapter_test_support::adapter_query_intent();
    assert_eq!(
        intent
            .adapter_request_context()
            .adapter_request_id()
            .as_str(),
        "adapter.request.query"
    );
}

#[test]
fn adapter_query_intent_rejects_capability_mismatch_k14_009() {
    let error = AdapterQueryIntent::new(
        crate::adapter_test_support::adapter_query_request_context(),
        adapter_command_capability(),
        service_query_intent(),
    )
    .expect_err("command capability must not satisfy query intent");
    assert_eq!(error.code(), AdapterErrorCode::CapabilityMismatch);
}

#[test]
fn adapter_query_intent_rejects_service_request_mismatch_k14_010() {
    let error = AdapterQueryIntent::new(
        crate::adapter_test_support::adapter_query_request_context(),
        crate::adapter_test_support::adapter_query_capability(),
        alternate_service_query_intent(),
    )
    .expect_err("mismatched service request contexts must fail");
    assert_eq!(
        error.code(),
        AdapterErrorCode::AdapterRequestIdentityMismatch
    );
}
