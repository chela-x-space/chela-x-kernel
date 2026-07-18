use crate::gateway_test_support::{
    gateway_request_context, memory_command_request, memory_record_id, operation_memory_capture,
    operation_workflow_transition, ownership_path, workflow_transition_control_request,
};
use crate::{GatewayCommandPayload, GatewayCommandRequest, GatewayRequestEnvelope};

#[test]
fn gateway_command_request_wraps_memory_capture_payload_k10_005() {
    let request = memory_command_request();
    matches!(
        request.gateway_command_payload(),
        GatewayCommandPayload::MemoryCapture(_)
    );
}

#[test]
fn gateway_request_envelope_accepts_matching_memory_command_k10_005() {
    let envelope = GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_command_request(),
    )
    .expect("matching command");
    assert_eq!(
        envelope
            .gateway_request_context()
            .gateway_operation_reference()
            .resource_segment(),
        "memory"
    );
}

#[test]
fn gateway_request_envelope_rejects_operation_command_mismatch_k10_005() {
    let request = GatewayCommandRequest::new(GatewayCommandPayload::WorkflowTransition(Box::new(
        workflow_transition_control_request(),
    )));
    let error = GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
        ),
        request,
    )
    .expect_err("workflow payload with memory operation must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidCommand);
}

#[test]
fn gateway_request_envelope_accepts_matching_workflow_command_k10_005() {
    let request = GatewayCommandRequest::new(GatewayCommandPayload::WorkflowTransition(Box::new(
        workflow_transition_control_request(),
    )));
    GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_workflow_transition(),
            "workflow",
            crate::gateway_test_support::workflow_id().as_str(),
            ownership_path(),
        ),
        request,
    )
    .expect("matching workflow command");
}
