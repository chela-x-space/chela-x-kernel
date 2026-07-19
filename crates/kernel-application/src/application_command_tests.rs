use crate::application_test_support::{
    command_application_request_context, command_capability, query_capability,
    studio_command_request,
};
use crate::{ApplicationCommandIntent, ApplicationErrorCode};

#[test]
fn application_command_intent_accepts_valid_studio_command_k12_003() {
    let intent = ApplicationCommandIntent::new(
        command_application_request_context(),
        command_capability(),
        studio_command_request(),
    )
    .expect("intent");
    assert_eq!(
        intent
            .application_request_context()
            .correlation_id()
            .as_str(),
        "CX-COR-000001"
    );
}

#[test]
fn application_command_intent_rejects_command_query_capability_mismatch_k12_003() {
    let error = ApplicationCommandIntent::new(
        command_application_request_context(),
        query_capability(),
        studio_command_request(),
    )
    .expect_err("query capability must not satisfy command intent");
    assert_eq!(error.code(), ApplicationErrorCode::CapabilityMismatch);
}
