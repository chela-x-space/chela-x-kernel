use crate::authorization::AuthorizationDecisionOutcome;
use crate::memory_test_support::{
    authorization_reference, memory_projection, memory_record, retrieval_request, workflow_id,
};
use crate::{
    MemoryCaptureDecision, MemoryCaptureRequest, MemoryQuery, MemoryQueryResult,
    MemoryRetentionDecision, MemoryRetrievalResult,
};

#[test]
fn memory_capture_evaluation_does_not_mutate_memory_record_k9_008() {
    let record = memory_record("memory.record-capture-separation");
    let original = record.clone();
    let request = MemoryCaptureRequest::new(
        record.clone(),
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        "capture record",
    )
    .expect("request");
    let _ = MemoryCaptureDecision::evaluate(&request);
    assert_eq!(record, original);
}

#[test]
fn memory_retention_evaluation_does_not_mutate_memory_record_k9_008() {
    let record = memory_record("memory.record-retention-separation");
    let original = record.clone();
    let _ = MemoryRetentionDecision::evaluate(
        &record,
        &authorization_reference(AuthorizationDecisionOutcome::Allow),
    );
    assert_eq!(record, original);
}

#[test]
fn memory_retrieval_and_projection_remain_read_only_k9_008() {
    let query = MemoryQuery::by_workflow(workflow_id());
    let request = retrieval_request(query.clone());
    let record = memory_record("memory.record-read-only");
    let projection = memory_projection("memory.record-read-only");
    let retrieval = MemoryRetrievalResult::new(request, vec![record.clone()]).expect("retrieval");
    let result = MemoryQueryResult::new(query, vec![projection.clone()]).expect("query result");
    assert_eq!(retrieval.memory_records(), &[record]);
    assert_eq!(result.memory_projections(), &[projection]);
}
