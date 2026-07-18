use crate::memory_test_support::{
    memory_audit_reference, memory_record_id, memory_record_reference,
};
use crate::{AuditEvidenceId, DomainError, EventTraceReference, MemoryAuditReference};

#[test]
fn memory_record_id_constructs_from_valid_trace_reference_k9_001() {
    let identifier = memory_record_id("memory.record-0001");
    assert_eq!(identifier.as_str(), "memory.record-0001");
}

#[test]
fn memory_record_id_rejects_invalid_trace_reference_k9_001() {
    let error =
        crate::MemoryRecordId::new("memory record 0001").expect_err("spaces must be rejected");
    assert!(matches!(
        error,
        DomainError::InvalidIdentifier {
            kind: "EventTraceReference",
            ..
        }
    ));
}

#[test]
fn memory_record_reference_preserves_identity_k9_001() {
    let reference = memory_record_reference("memory.record-0001");
    assert_eq!(reference.memory_record_id().as_str(), "memory.record-0001");
}

#[test]
fn memory_audit_reference_rejects_duplicate_evidence_ids_k9_001() {
    let error = MemoryAuditReference::new(
        memory_record_id("memory.record-0002"),
        EventTraceReference::new("memory.audit-duplicate").expect("trace"),
        vec![
            AuditEvidenceId::new("CX-AUD-000001").expect("evidence"),
            AuditEvidenceId::new("CX-AUD-000001").expect("evidence"),
        ],
    )
    .expect_err("duplicate evidence ids must fail");
    assert_eq!(
        error,
        DomainError::InvalidMemory("duplicate memory audit evidence identifier")
    );
}

#[test]
fn memory_audit_reference_preserves_identity_and_trace_k9_001() {
    let audit_reference = memory_audit_reference("memory.record-0003");
    assert_eq!(
        audit_reference.memory_record_id().as_str(),
        "memory.record-0003"
    );
    assert_eq!(
        audit_reference.audit_trace_reference().as_str(),
        "memory.audit-0001"
    );
}
