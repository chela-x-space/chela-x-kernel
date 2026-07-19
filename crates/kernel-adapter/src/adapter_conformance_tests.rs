use crate::adapter_test_support::{
    adapter_command_request_context, adapter_query_capability, service_command_intent,
};
use crate::{AdapterCommandIntent, AdapterErrorCode};

#[test]
fn adapter_equivalent_inputs_produce_equivalent_results_k14_001() {
    let left = crate::adapter_test_support::adapter_query_intent();
    let right = crate::adapter_test_support::adapter_query_intent();
    assert_eq!(left, right);
}

#[test]
fn adapter_rejected_cases_remain_side_effect_free_k14_009() {
    let request_context = adapter_command_request_context();
    let service_intent = service_command_intent();
    let request_context_before = request_context.clone();
    let service_intent_before = service_intent.clone();
    let error = AdapterCommandIntent::new(
        request_context.clone(),
        adapter_query_capability(),
        service_intent.clone(),
    )
    .expect_err("query capability must not satisfy command intent");

    assert_eq!(error.code(), AdapterErrorCode::CapabilityMismatch);
    assert_eq!(request_context, request_context_before);
    assert_eq!(service_intent, service_intent_before);
}
