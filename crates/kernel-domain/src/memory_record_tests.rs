use crate::authorization::AuthorizationDecisionOutcome;
use crate::memory_test_support::{
    authorization_reference, enterprise_ownership, event_only_memory_record,
    memory_audit_reference, memory_classification, memory_provenance, memory_record,
    memory_record_reference, task_instance_reference, time_reference, workflow_id,
    workspace_ownership,
};
use crate::{
    DomainError, EventClassification, EventId, MemoryCaptureDecision, MemoryCaptureRequest,
    MemoryClassification, MemoryProvenance, MemoryRecord, MemoryRejectionReason,
    MemoryRelationship, MemoryRelationshipRequest, MemoryRetentionDecision,
    MemoryRetentionPolicyReference, PolicyId,
};

#[test]
fn memory_record_constructs_with_event_only_provenance_k9_002() {
    let record = event_only_memory_record("memory.record-event-only");
    assert_eq!(
        record.memory_record_reference().memory_record_id().as_str(),
        "memory.record-event-only"
    );
    assert_eq!(
        record.memory_provenance().event_id().as_str(),
        "CX-EVT-000099"
    );
}

#[test]
fn memory_record_preserves_full_provenance_k9_002() {
    let provenance = memory_provenance();
    assert_eq!(provenance.event_id().as_str(), "CX-EVT-000001");
    assert_eq!(provenance.workflow_id(), Some(&workflow_id()));
    assert_eq!(
        provenance.task_instance_reference(),
        Some(&task_instance_reference())
    );
}

#[test]
fn memory_record_rejects_audit_reference_identity_mismatch_k9_002() {
    let error = MemoryRecord::new(
        memory_record_reference("memory.record-0001"),
        workspace_ownership("CX-WS-000001"),
        "mismatched audit identity",
        memory_classification(),
        memory_provenance(),
        MemoryRetentionPolicyReference::new(PolicyId::new("CX-POL-000020").expect("policy")),
        memory_audit_reference("memory.record-other"),
        time_reference(),
    )
    .expect_err("audit identity mismatch must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory("memory audit reference must preserve memory record identity")
    );
}

#[test]
fn memory_classification_reuses_event_classification_vocabulary_k9_003() {
    let classification = MemoryClassification::new("CONFIDENTIAL").expect("classification");
    assert_eq!(
        classification.event_classification(),
        EventClassification::Confidential
    );
    assert_eq!(classification.as_str(), "CONFIDENTIAL");
}

#[test]
fn memory_classification_rejects_invalid_value_k9_003() {
    let error = MemoryClassification::new("TOP_SECRET").expect_err("invalid classification");
    assert!(matches!(
        error,
        DomainError::InvalidIdentifier {
            kind: "EventClassification",
            ..
        }
    ));
}

#[test]
fn memory_relationship_rejects_self_relationship_k9_004() {
    let reference = memory_record_reference("memory.record-self");
    let error = MemoryRelationship::new(reference.clone(), reference, "memory.related")
        .expect_err("self relationship must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory("memory relationships must not relate a record to itself")
    );
}

#[test]
fn memory_relationship_request_rejects_scope_mismatch_k9_004() {
    let source = memory_record("memory.record-source");
    let target = MemoryRecord::new(
        memory_record_reference("memory.record-target"),
        workspace_ownership("CX-WS-000002"),
        "cross workspace target",
        memory_classification(),
        memory_provenance(),
        MemoryRetentionPolicyReference::new(PolicyId::new("CX-POL-000021").expect("policy")),
        memory_audit_reference("memory.record-target"),
        time_reference(),
    )
    .expect("target");
    let relationship = MemoryRelationship::new(
        source.memory_record_reference().clone(),
        target.memory_record_reference().clone(),
        "memory.related",
    )
    .expect("relationship");
    let error = MemoryRelationshipRequest::new(
        source,
        target,
        relationship,
        authorization_reference(AuthorizationDecisionOutcome::Allow),
    )
    .expect_err("scope mismatch must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory(
            "memory relationship request requires compatible enterprise and workspace scope"
        )
    );
}

#[test]
fn memory_retention_decision_is_deterministic_k9_005() {
    let record = memory_record("memory.record-retain");
    let left = MemoryRetentionDecision::evaluate(
        &record,
        &authorization_reference(AuthorizationDecisionOutcome::Allow),
    );
    let right = MemoryRetentionDecision::evaluate(
        &record,
        &authorization_reference(AuthorizationDecisionOutcome::Allow),
    );
    assert_eq!(left, right);
}

#[test]
fn memory_capture_decision_is_mutually_exclusive_k9_005() {
    let request = MemoryCaptureRequest::new(
        memory_record("memory.record-capture"),
        authorization_reference(AuthorizationDecisionOutcome::Deny),
        "capture memory",
    )
    .expect("request");
    assert_eq!(
        MemoryCaptureDecision::evaluate(&request),
        MemoryCaptureDecision::Rejected(MemoryRejectionReason::AuthorizationDenied)
    );
}

#[test]
fn memory_capture_decision_accepts_allowed_request_k9_005() {
    let record = memory_record("memory.record-accepted");
    let request = MemoryCaptureRequest::new(
        record.clone(),
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        "capture memory",
    )
    .expect("request");
    assert_eq!(
        MemoryCaptureDecision::evaluate(&request),
        MemoryCaptureDecision::Accepted(Box::new(record))
    );
}

#[test]
fn memory_provenance_preserves_enterprise_fact_references_k9_002() {
    let provenance = MemoryProvenance::new(
        EventId::new("CX-EVT-000777").expect("event"),
        Some(workflow_id()),
        Some(task_instance_reference()),
        None,
        None,
        Some(authorization_reference(AuthorizationDecisionOutcome::Allow)),
        None,
    )
    .expect("provenance");
    assert_eq!(provenance.event_id().as_str(), "CX-EVT-000777");
    assert_eq!(provenance.workflow_id(), Some(&workflow_id()));
}

#[test]
fn memory_record_preserves_enterprise_scope_without_mutation_k9_008() {
    let record = MemoryRecord::new(
        memory_record_reference("memory.record-enterprise"),
        enterprise_ownership(),
        "enterprise scoped memory",
        memory_classification(),
        memory_provenance(),
        MemoryRetentionPolicyReference::new(PolicyId::new("CX-POL-000030").expect("policy")),
        memory_audit_reference("memory.record-enterprise"),
        time_reference(),
    )
    .expect("record");
    assert_eq!(record.ownership_path(), &enterprise_ownership());
}
