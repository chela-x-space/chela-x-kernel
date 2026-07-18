use crate::gateway_test_support::{
    gateway_request_context, memory_query, memory_query_request, operation_gateway_status,
    operation_memory_retrieve, operation_task_query, operation_workflow_query, ownership_path,
    project_scope_mismatch_path, task_instance_reference, workflow_id,
};
use crate::{GatewayQueryPayload, GatewayQueryRequest, GatewayRequestEnvelope};

#[test]
fn gateway_request_envelope_accepts_matching_memory_retrieval_query_k10_006() {
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_retrieve(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        memory_query_request(),
    )
    .expect("matching retrieval");
}

#[test]
fn gateway_request_envelope_rejects_query_scope_mismatch_k10_006() {
    let request = GatewayQueryRequest::new(GatewayQueryPayload::MemoryRetrieval(Box::new(
        kernel_domain::MemoryRetrievalRequest::new(
            crate::gateway_test_support::authorization_binding(
                "memory",
                crate::gateway_test_support::memory_record_id().as_str(),
                ownership_path(),
                kernel_domain::AuthorizationDecisionOutcome::Allow,
            )
            .authorization_decision_reference()
            .clone(),
            project_scope_mismatch_path(),
            memory_query(),
        )
        .expect("retrieval"),
    )));
    let error = GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_retrieve(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        request,
    )
    .expect_err("scope mismatch must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::ScopeMismatch);
}

#[test]
fn gateway_request_envelope_accepts_matching_workflow_query_k10_006() {
    let request = GatewayQueryRequest::new(GatewayQueryPayload::WorkflowState(workflow_id()));
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_workflow_query(),
            "workflow",
            workflow_id().as_str(),
            ownership_path(),
        ),
        request,
    )
    .expect("workflow query");
}

#[test]
fn gateway_request_envelope_accepts_matching_task_query_k10_006() {
    let request =
        GatewayQueryRequest::new(GatewayQueryPayload::TaskState(task_instance_reference()));
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_task_query(),
            "task",
            task_instance_reference().task_instance_id().as_str(),
            ownership_path(),
        ),
        request,
    )
    .expect("task query");
}

#[test]
fn gateway_request_envelope_accepts_gateway_status_query_k10_006() {
    let request = GatewayQueryRequest::new(GatewayQueryPayload::GatewayStatus);
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_gateway_status(),
            "gateway",
            "status",
            ownership_path(),
        ),
        request,
    )
    .expect("status query");
}
