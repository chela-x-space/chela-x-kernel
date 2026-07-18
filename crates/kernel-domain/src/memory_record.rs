use crate::authorization::AuthorizationDecisionReference;
use crate::errors::{DomainError, DomainResult};
use crate::event::EventClassification;
use crate::identifier::{EventId, NonEmptyText, PolicyId, RuntimeId, WorkflowId};
use crate::memory::{MemoryAuditReference, MemoryRecordReference};
use crate::memory_validation::{require_allowed, scopes_are_compatible};
use crate::ownership::OwnershipPath;
use crate::request::TimeReference;
use crate::{ExecutionSessionId, TaskEvidenceReference, TaskInstanceReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryClassification(EventClassification);

impl MemoryClassification {
    pub fn new(value: impl AsRef<str>) -> DomainResult<Self> {
        EventClassification::new(value).map(Self)
    }
    pub const fn event_classification(self) -> EventClassification {
        self.0
    }
    pub const fn as_str(self) -> &'static str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryRetentionPolicyReference {
    policy_id: PolicyId,
}

impl MemoryRetentionPolicyReference {
    pub fn new(policy_id: PolicyId) -> Self {
        Self { policy_id }
    }
    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryProvenance {
    event_id: EventId,
    workflow_id: Option<WorkflowId>,
    task_instance_reference: Option<TaskInstanceReference>,
    execution_session_id: Option<ExecutionSessionId>,
    runtime_id: Option<RuntimeId>,
    authorization_decision_reference: Option<AuthorizationDecisionReference>,
    task_evidence_reference: Option<TaskEvidenceReference>,
}

impl MemoryProvenance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_id: EventId,
        workflow_id: Option<WorkflowId>,
        task_instance_reference: Option<TaskInstanceReference>,
        execution_session_id: Option<ExecutionSessionId>,
        runtime_id: Option<RuntimeId>,
        authorization_decision_reference: Option<AuthorizationDecisionReference>,
        task_evidence_reference: Option<TaskEvidenceReference>,
    ) -> DomainResult<Self> {
        Ok(Self {
            event_id,
            workflow_id,
            task_instance_reference,
            execution_session_id,
            runtime_id,
            authorization_decision_reference,
            task_evidence_reference,
        })
    }

    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }
    pub fn workflow_id(&self) -> Option<&WorkflowId> {
        self.workflow_id.as_ref()
    }
    pub fn task_instance_reference(&self) -> Option<&TaskInstanceReference> {
        self.task_instance_reference.as_ref()
    }
    pub fn execution_session_id(&self) -> Option<&ExecutionSessionId> {
        self.execution_session_id.as_ref()
    }
    pub fn runtime_id(&self) -> Option<&RuntimeId> {
        self.runtime_id.as_ref()
    }
    pub fn authorization_decision_reference(&self) -> Option<&AuthorizationDecisionReference> {
        self.authorization_decision_reference.as_ref()
    }
    pub fn task_evidence_reference(&self) -> Option<&TaskEvidenceReference> {
        self.task_evidence_reference.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRecord {
    memory_record_reference: MemoryRecordReference,
    ownership_path: OwnershipPath,
    memory_summary: NonEmptyText,
    memory_classification: MemoryClassification,
    memory_provenance: MemoryProvenance,
    memory_retention_policy_reference: MemoryRetentionPolicyReference,
    memory_audit_reference: MemoryAuditReference,
    captured_at: TimeReference,
}

impl MemoryRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        memory_record_reference: MemoryRecordReference,
        ownership_path: OwnershipPath,
        memory_summary: impl Into<String>,
        memory_classification: MemoryClassification,
        memory_provenance: MemoryProvenance,
        memory_retention_policy_reference: MemoryRetentionPolicyReference,
        memory_audit_reference: MemoryAuditReference,
        captured_at: TimeReference,
    ) -> DomainResult<Self> {
        if ownership_path.contains_repeated_elements() {
            return Err(DomainError::InvalidMemory(
                "memory record ownership path cannot contain repeated scope identifiers",
            ));
        }
        if memory_audit_reference.memory_record_id() != memory_record_reference.memory_record_id() {
            return Err(DomainError::InvalidMemory(
                "memory audit reference must preserve memory record identity",
            ));
        }
        Ok(Self {
            memory_record_reference,
            ownership_path,
            memory_summary: NonEmptyText::new("memory_summary", memory_summary)?,
            memory_classification,
            memory_provenance,
            memory_retention_policy_reference,
            memory_audit_reference,
            captured_at,
        })
    }

    pub fn memory_record_reference(&self) -> &MemoryRecordReference {
        &self.memory_record_reference
    }
    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn memory_summary(&self) -> &str {
        self.memory_summary.as_str()
    }
    pub fn memory_classification(&self) -> MemoryClassification {
        self.memory_classification
    }
    pub fn memory_provenance(&self) -> &MemoryProvenance {
        &self.memory_provenance
    }
    pub fn memory_retention_policy_reference(&self) -> &MemoryRetentionPolicyReference {
        &self.memory_retention_policy_reference
    }
    pub fn memory_audit_reference(&self) -> &MemoryAuditReference {
        &self.memory_audit_reference
    }
    pub fn captured_at(&self) -> &TimeReference {
        &self.captured_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRejectionReason {
    AuthorizationDenied,
    DuplicateMemoryRecord,
    MissingProvenance,
    MissingRetentionPolicy,
    ScopeMismatch,
    SelfRelationship,
    UnsupportedRelationshipTarget,
    QueryResultMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryCaptureRequest {
    memory_record: MemoryRecord,
    authorization_decision_reference: AuthorizationDecisionReference,
    capture_reason: NonEmptyText,
}

impl MemoryCaptureRequest {
    pub fn new(
        memory_record: MemoryRecord,
        authorization_decision_reference: AuthorizationDecisionReference,
        capture_reason: impl Into<String>,
    ) -> DomainResult<Self> {
        Ok(Self {
            memory_record,
            authorization_decision_reference,
            capture_reason: NonEmptyText::new("memory_capture_reason", capture_reason)?,
        })
    }
    pub fn memory_record(&self) -> &MemoryRecord {
        &self.memory_record
    }
    pub fn authorization_decision_reference(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision_reference
    }
    pub fn capture_reason(&self) -> &str {
        self.capture_reason.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryCaptureDecision {
    Accepted(Box<MemoryRecord>),
    Rejected(MemoryRejectionReason),
}

impl MemoryCaptureDecision {
    pub fn evaluate(memory_capture_request: &MemoryCaptureRequest) -> Self {
        if memory_capture_request
            .authorization_decision_reference()
            .outcome()
            .is_denied()
        {
            return Self::Rejected(MemoryRejectionReason::AuthorizationDenied);
        }
        Self::Accepted(Box::new(memory_capture_request.memory_record().clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryRetentionDecision {
    Retained {
        memory_record_reference: MemoryRecordReference,
        memory_retention_policy_reference: MemoryRetentionPolicyReference,
    },
    Rejected(MemoryRejectionReason),
}

impl MemoryRetentionDecision {
    pub fn evaluate(
        memory_record: &MemoryRecord,
        authorization_decision_reference: &AuthorizationDecisionReference,
    ) -> Self {
        if authorization_decision_reference.outcome().is_denied() {
            return Self::Rejected(MemoryRejectionReason::AuthorizationDenied);
        }
        Self::Retained {
            memory_record_reference: memory_record.memory_record_reference().clone(),
            memory_retention_policy_reference: memory_record
                .memory_retention_policy_reference()
                .clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRelationship {
    source_memory_record_reference: MemoryRecordReference,
    target_memory_record_reference: MemoryRecordReference,
    relationship_kind: crate::identifier::EnglishNamespace,
}

impl MemoryRelationship {
    pub fn new(
        source_memory_record_reference: MemoryRecordReference,
        target_memory_record_reference: MemoryRecordReference,
        relationship_kind: impl Into<String>,
    ) -> DomainResult<Self> {
        if source_memory_record_reference == target_memory_record_reference {
            return Err(DomainError::InvalidMemory(
                "memory relationships must not relate a record to itself",
            ));
        }
        Ok(Self {
            source_memory_record_reference,
            target_memory_record_reference,
            relationship_kind: crate::identifier::EnglishNamespace::new(
                "MemoryRelationshipKind",
                relationship_kind,
            )?,
        })
    }

    pub fn source_memory_record_reference(&self) -> &MemoryRecordReference {
        &self.source_memory_record_reference
    }
    pub fn target_memory_record_reference(&self) -> &MemoryRecordReference {
        &self.target_memory_record_reference
    }
    pub fn relationship_kind(&self) -> &str {
        self.relationship_kind.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRelationshipRequest {
    source_memory_record: MemoryRecord,
    target_memory_record: MemoryRecord,
    memory_relationship: MemoryRelationship,
    authorization_decision_reference: AuthorizationDecisionReference,
}

impl MemoryRelationshipRequest {
    pub fn new(
        source_memory_record: MemoryRecord,
        target_memory_record: MemoryRecord,
        memory_relationship: MemoryRelationship,
        authorization_decision_reference: AuthorizationDecisionReference,
    ) -> DomainResult<Self> {
        require_allowed(&authorization_decision_reference)?;
        if memory_relationship.source_memory_record_reference()
            != source_memory_record.memory_record_reference()
            || memory_relationship.target_memory_record_reference()
                != target_memory_record.memory_record_reference()
        {
            return Err(DomainError::InvalidMemory(
                "memory relationship request must preserve source and target record identity",
            ));
        }
        if !scopes_are_compatible(
            source_memory_record.ownership_path(),
            target_memory_record.ownership_path(),
        ) {
            return Err(DomainError::InvalidMemory(
                "memory relationship request requires compatible enterprise and workspace scope",
            ));
        }
        Ok(Self {
            source_memory_record,
            target_memory_record,
            memory_relationship,
            authorization_decision_reference,
        })
    }

    pub fn source_memory_record(&self) -> &MemoryRecord {
        &self.source_memory_record
    }
    pub fn target_memory_record(&self) -> &MemoryRecord {
        &self.target_memory_record
    }
    pub fn memory_relationship(&self) -> &MemoryRelationship {
        &self.memory_relationship
    }
    pub fn authorization_decision_reference(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision_reference
    }
}
