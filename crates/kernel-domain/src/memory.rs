use crate::errors::{DomainError, DomainResult};
use crate::event::EventTraceReference;
use crate::identifier::AuditEvidenceId;
use crate::memory_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryRecordId(EventTraceReference);

impl MemoryRecordId {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        EventTraceReference::new(value).map(Self)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryRecordReference {
    memory_record_id: MemoryRecordId,
}

impl MemoryRecordReference {
    pub fn new(memory_record_id: MemoryRecordId) -> Self {
        Self { memory_record_id }
    }
    pub fn memory_record_id(&self) -> &MemoryRecordId {
        &self.memory_record_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryAuditReference {
    memory_record_id: MemoryRecordId,
    audit_trace_reference: EventTraceReference,
    audit_evidence_ids: Vec<AuditEvidenceId>,
}

impl MemoryAuditReference {
    pub fn new(
        memory_record_id: MemoryRecordId,
        audit_trace_reference: EventTraceReference,
        audit_evidence_ids: Vec<AuditEvidenceId>,
    ) -> DomainResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(DomainError::InvalidMemory(
                "memory audit reference requires audit evidence identifiers",
            ));
        }
        reject_duplicates(
            &audit_evidence_ids,
            "duplicate memory audit evidence identifier",
        )?;
        Ok(Self {
            memory_record_id,
            audit_trace_reference,
            audit_evidence_ids,
        })
    }

    pub fn memory_record_id(&self) -> &MemoryRecordId {
        &self.memory_record_id
    }
    pub fn audit_trace_reference(&self) -> &EventTraceReference {
        &self.audit_trace_reference
    }
    pub fn audit_evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.audit_evidence_ids
    }
}
