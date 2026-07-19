use crate::application_test_support::{
    application_audit_reference, application_query_intent, command_application_request_context,
    command_capability, later_time_reference, studio_command_request, studio_command_response,
    studio_view_response,
};
use crate::ApplicationErrorCode;
use crate::{
    ApplicationCommandIntent, ApplicationRequestEnvelope, ApplicationResponseEnvelope,
    ApplicationResponsePayload, ApplicationResponseStatusReference,
};

#[test]
fn application_response_preserves_request_correlation_k12_010() {
    let response = ApplicationResponseEnvelope::new(
        &ApplicationRequestEnvelope::query(application_query_intent()),
        ApplicationResponsePayload::View(Box::new(studio_view_response())),
        ApplicationResponseStatusReference::new("application.response.valid").expect("status"),
        application_audit_reference(),
        later_time_reference(),
    )
    .expect("response");
    assert_eq!(response.correlation_id().as_str(), "CX-COR-000001");
}

#[test]
fn application_response_rejects_request_response_mismatch_k12_010() {
    let command_intent = ApplicationCommandIntent::new(
        command_application_request_context(),
        command_capability(),
        studio_command_request(),
    )
    .expect("command");
    let error = ApplicationResponseEnvelope::new(
        &ApplicationRequestEnvelope::query(application_query_intent()),
        ApplicationResponsePayload::Command(Box::new(studio_command_response())),
        ApplicationResponseStatusReference::new("application.response.valid").expect("status"),
        application_audit_reference(),
        later_time_reference(),
    )
    .expect_err("request/response mismatch must fail");
    assert_eq!(error.code(), ApplicationErrorCode::ResponseRequestMismatch);
    let _ = command_intent;
}
