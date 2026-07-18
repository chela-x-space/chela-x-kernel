use crate::gateway_test_support::{
    accepted_memory_capture_decision, correlation_id, gateway_audit_reference,
    gateway_request_context, gateway_status_snapshot, memory_command_request, memory_query,
    memory_query_request, memory_record, memory_retrieval_request, operation_gateway_status,
    operation_memory_capture, operation_memory_query, operation_memory_retrieve, ownership_path,
    responded_at, task_state_snapshot,
};
use crate::{
    gateway_response::GatewayEnvelopeResponse, GatewayCommandResponse, GatewayQueryPayload,
    GatewayQueryRequest, GatewayQueryResponse, GatewayRequestEnvelope, GatewayResponseEnvelope,
};

#[test]
fn gateway_response_envelope_accepts_matching_command_response_k10_007() {
    let request = GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_command_request(),
    )
    .expect("request");
    let response = GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Command(Box::new(GatewayCommandResponse::MemoryCapture(
            Box::new(accepted_memory_capture_decision()),
        ))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect("response");
    matches!(response, GatewayResponseEnvelope::Command { .. });
}

#[test]
fn gateway_response_envelope_rejects_request_response_correlation_mismatch_k10_007() {
    let request = GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_command_request(),
    )
    .expect("request");
    let error = GatewayResponseEnvelope::new(
        &request,
        kernel_domain::CorrelationId::new("CX-COR-000099").expect("correlation"),
        GatewayEnvelopeResponse::Command(Box::new(GatewayCommandResponse::MemoryCapture(
            Box::new(accepted_memory_capture_decision()),
        ))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect_err("correlation mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidRequest);
}

#[test]
fn gateway_response_envelope_rejects_command_query_kind_mismatch_k10_007() {
    let request = GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_retrieve(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_query_request(),
    )
    .expect("request");
    let error = GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Command(Box::new(GatewayCommandResponse::MemoryCapture(
            Box::new(accepted_memory_capture_decision()),
        ))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect_err("query request cannot carry command response");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidQuery);
}

#[test]
fn gateway_response_envelope_accepts_matching_memory_retrieval_response_k10_007() {
    let request = GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_retrieve(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_query_request(),
    )
    .expect("request");
    let retrieval_result = kernel_domain::MemoryRetrievalResult::new(
        memory_retrieval_request(),
        vec![memory_record(ownership_path())],
    )
    .expect("result");
    GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Query(Box::new(GatewayQueryResponse::MemoryRetrieval(Box::new(
            retrieval_result,
        )))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect("response");
}

#[test]
fn gateway_response_envelope_accepts_matching_status_response_k10_007() {
    let request = GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_gateway_status(),
            "gateway",
            "status",
            ownership_path(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::GatewayStatus),
    )
    .expect("request");
    GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Query(Box::new(GatewayQueryResponse::StatusSnapshot(
            gateway_status_snapshot(),
        ))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect("status response");
}

#[test]
fn gateway_response_envelope_accepts_matching_task_query_response_k10_007() {
    let request = GatewayRequestEnvelope::query(
        gateway_request_context(
            crate::gateway_test_support::operation_task_query(),
            "task",
            crate::gateway_test_support::task_instance_reference()
                .task_instance_id()
                .as_str(),
            ownership_path(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::TaskState(
            crate::gateway_test_support::task_instance_reference(),
        )),
    )
    .expect("request");
    GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Query(Box::new(GatewayQueryResponse::TaskStateSnapshots(vec![
            task_state_snapshot(),
        ]))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect("task response");
}

#[test]
fn gateway_response_envelope_rejects_memory_query_result_mismatch_k10_008() {
    let request = GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_query(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::MemoryQuery(memory_query())),
    )
    .expect("request");
    let mismatched_query_result = kernel_domain::MemoryQueryResult::new(
        kernel_domain::MemoryQuery::by_classification(
            kernel_domain::MemoryClassification::new("INTERNAL").expect("classification"),
        ),
        vec![kernel_domain::MemoryProjection::new(&memory_record(
            ownership_path(),
        ))],
    )
    .expect("query result");
    let error = GatewayResponseEnvelope::new(
        &request,
        correlation_id(),
        GatewayEnvelopeResponse::Query(Box::new(GatewayQueryResponse::MemoryQuery(Box::new(
            mismatched_query_result,
        )))),
        gateway_audit_reference(),
        responded_at(),
    )
    .expect_err("query mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidQuery);
}
