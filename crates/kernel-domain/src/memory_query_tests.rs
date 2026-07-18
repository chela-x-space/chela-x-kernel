use crate::authorization::AuthorizationDecisionOutcome;
use crate::memory_test_support::{
    authorization_reference, memory_projection, memory_record, memory_record_reference,
    memory_record_with, retrieval_request, runtime_id, scoped_ownership, task_instance_reference,
    workflow_id, workflow_memory_provenance, workspace_ownership,
};
use crate::{
    DomainError, MemoryQuery, MemoryQueryResult, MemoryRetrievalRequest, MemoryRetrievalResult,
};

#[test]
fn memory_query_by_record_references_rejects_empty_input_k9_006() {
    let error = MemoryQuery::by_record_references(vec![]).expect_err("empty query must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory query by record references requires at least one reference"
        )
    );
}

#[test]
fn memory_query_by_record_references_rejects_duplicates_k9_006() {
    let reference = memory_record_reference("memory.record-duplicate");
    let error = MemoryQuery::by_record_references(vec![reference.clone(), reference])
        .expect_err("duplicate references must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory("duplicate memory record reference in memory query")
    );
}

#[test]
fn memory_retrieval_request_requires_allowed_authorization_k9_006() {
    let error = MemoryRetrievalRequest::new(
        authorization_reference(AuthorizationDecisionOutcome::Deny),
        workspace_ownership("CX-WS-000001"),
        MemoryQuery::by_workflow(workflow_id()),
    )
    .expect_err("denied authorization must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory("memory operation requires an allowed authorization decision")
    );
}

#[test]
fn memory_retrieval_result_preserves_supplied_order_k9_006() {
    let query = MemoryQuery::by_record_references(vec![
        memory_record_reference("memory.record-0002"),
        memory_record_reference("memory.record-0001"),
    ])
    .expect("query");
    let request = retrieval_request(query);
    let result = MemoryRetrievalResult::new(
        request,
        vec![
            memory_record("memory.record-0002"),
            memory_record("memory.record-0001"),
        ],
    )
    .expect("result");
    assert_eq!(
        result.memory_records()[0]
            .memory_record_reference()
            .memory_record_id()
            .as_str(),
        "memory.record-0002"
    );
}

#[test]
fn memory_retrieval_result_rejects_query_mismatch_k9_006() {
    let request = retrieval_request(MemoryQuery::by_workflow(workflow_id()));
    let error = MemoryRetrievalResult::new(
        request,
        vec![memory_record_with(
            "memory.record-0001",
            workspace_ownership("CX-WS-000001"),
            workflow_memory_provenance(crate::WorkflowId::new("CX-WF-000002").expect("workflow")),
        )],
    )
    .expect_err("record outside requested scope must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory retrieval result contains a record outside the requested scope or query"
        )
    );
}

#[test]
fn memory_retrieval_result_rejects_matching_query_with_workspace_scope_mismatch_k9_006() {
    let request = retrieval_request(MemoryQuery::by_workflow(workflow_id()));
    let error = MemoryRetrievalResult::new(
        request,
        vec![memory_record_with(
            "memory.record-0003",
            scoped_ownership("CX-WS-000002", "CX-PROJ-000001", None),
            workflow_memory_provenance(workflow_id()),
        )],
    )
    .expect_err("workspace mismatch must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory retrieval result contains a record outside the requested scope or query"
        )
    );
}

#[test]
fn memory_retrieval_result_rejects_matching_query_with_project_scope_mismatch_k9_006() {
    let request = retrieval_request(MemoryQuery::by_workflow(workflow_id()));
    let error = MemoryRetrievalResult::new(
        request,
        vec![memory_record_with(
            "memory.record-0004",
            scoped_ownership("CX-WS-000001", "CX-PROJ-000002", None),
            workflow_memory_provenance(workflow_id()),
        )],
    )
    .expect_err("project mismatch must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory retrieval result contains a record outside the requested scope or query"
        )
    );
}

#[test]
fn memory_retrieval_result_rejects_matching_query_with_organizational_unit_scope_mismatch_k9_006() {
    let request = MemoryRetrievalRequest::new(
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        scoped_ownership("CX-WS-000001", "CX-PROJ-000001", Some("CX-OU-000001")),
        MemoryQuery::by_workflow(workflow_id()),
    )
    .expect("request");
    let error = MemoryRetrievalResult::new(
        request,
        vec![memory_record_with(
            "memory.record-0005",
            scoped_ownership("CX-WS-000001", "CX-PROJ-000001", Some("CX-OU-000002")),
            workflow_memory_provenance(workflow_id()),
        )],
    )
    .expect_err("organizational unit mismatch must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory retrieval result contains a record outside the requested scope or query"
        )
    );
}

#[test]
fn memory_retrieval_result_is_deterministic_k9_006() {
    let query = MemoryQuery::by_task(task_instance_reference());
    let left = MemoryRetrievalResult::new(
        retrieval_request(query.clone()),
        vec![memory_record("memory.record-0001")],
    )
    .expect("left");
    let right = MemoryRetrievalResult::new(
        retrieval_request(query),
        vec![memory_record("memory.record-0001")],
    )
    .expect("right");
    assert_eq!(left, right);
}

#[test]
fn memory_query_result_preserves_projection_order_k9_007() {
    let query = MemoryQuery::by_runtime(runtime_id());
    let result = MemoryQueryResult::new(
        query,
        vec![
            memory_projection("memory.record-0001"),
            memory_projection("memory.record-0002"),
        ],
    )
    .expect("query result");
    assert_eq!(
        result.memory_projections()[1]
            .memory_record_reference()
            .memory_record_id()
            .as_str(),
        "memory.record-0002"
    );
}

#[test]
fn memory_query_result_rejects_projection_mismatch_k9_007() {
    let error = MemoryQueryResult::new(
        MemoryQuery::by_record_references(vec![memory_record_reference("memory.record-0001")])
            .expect("query"),
        vec![memory_projection("memory.record-0002")],
    )
    .expect_err("mismatched projection must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory query result contains a projection outside the requested query"
        )
    );
}
