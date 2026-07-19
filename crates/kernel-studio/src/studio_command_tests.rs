use crate::studio_test_support::{
    correlation_id, later_time_reference, studio_api_version, studio_audit_reference,
    studio_selection_context, studio_view_reference, workflow_id,
};
use crate::{StudioCommandRequest, StudioCommandResponse, StudioViewKind};
use kernel_domain::{MemoryCaptureDecision, MemoryCaptureRequest};
use kernel_gateway::{GatewayCommandPayload, GatewayCommandRequest, GatewayRequestEnvelope};

#[test]
fn studio_command_request_requires_command_console_k11_008() {
    let memory_record = crate::studio_test_support::memory_record();
    let gateway_envelope = GatewayRequestEnvelope::command(
        kernel_gateway::GatewayRequestContext::new(
            crate::studio_test_support::gateway_api_version(),
            kernel_gateway::GatewayOperationReference::new(
                kernel_gateway::GatewayOperationKind::Command,
                "memory.capture",
            )
            .expect("operation"),
            crate::studio_test_support::gateway_authentication_context(),
            crate::studio_test_support::authorization_binding(
                "memory",
                crate::studio_test_support::memory_record_reference()
                    .memory_record_id()
                    .as_str(),
            ),
            crate::studio_test_support::ownership_path(),
            correlation_id(),
            crate::studio_test_support::time_reference(),
            None,
            crate::studio_test_support::gateway_audit_reference(),
        )
        .expect("context"),
        GatewayCommandRequest::new(GatewayCommandPayload::MemoryCapture(Box::new(
            MemoryCaptureRequest::new(
                memory_record,
                crate::studio_test_support::authorization_binding(
                    "memory",
                    crate::studio_test_support::memory_record_reference()
                        .memory_record_id()
                        .as_str(),
                )
                .authorization_decision_reference()
                .clone(),
                "capture",
            )
            .expect("capture"),
        ))),
    )
    .expect("command");

    let request = StudioCommandRequest::new(
        studio_api_version(),
        studio_view_reference(StudioViewKind::CommandConsole),
        studio_selection_context(),
        correlation_id(),
        crate::studio_test_support::time_reference(),
        gateway_envelope,
        studio_audit_reference(),
    )
    .expect("request");

    let response = StudioCommandResponse::new(
        &request,
        correlation_id(),
        kernel_gateway::GatewayCommandResponse::MemoryCapture(Box::new(
            MemoryCaptureDecision::Accepted(Box::new(crate::studio_test_support::memory_record())),
        )),
        studio_audit_reference(),
        later_time_reference(),
    )
    .expect("response");
    assert_eq!(response.correlation_id().as_str(), "CX-COR-000001");
    let _ = workflow_id();
}
