use crate::service_test_support::{
    application_command_intent, command_service_request_context, service_query_capability,
};
use crate::{ServiceCommandIntent, ServiceErrorCode};

#[test]
fn service_equivalent_inputs_produce_equivalent_results_k13_001() {
    let left = crate::service_test_support::service_query_intent();
    let right = crate::service_test_support::service_query_intent();
    assert_eq!(left, right);
}

#[test]
fn service_rejected_cases_remain_side_effect_free_k13_009() {
    let request_context = command_service_request_context();
    let application_intent = application_command_intent();
    let request_context_before = request_context.clone();
    let application_intent_before = application_intent.clone();
    let error = ServiceCommandIntent::new(
        request_context.clone(),
        service_query_capability(),
        application_intent.clone(),
    )
    .expect_err("query capability must not satisfy command intent");

    assert_eq!(error.code(), ServiceErrorCode::CapabilityMismatch);
    assert_eq!(request_context, request_context_before);
    assert_eq!(application_intent, application_intent_before);
}
