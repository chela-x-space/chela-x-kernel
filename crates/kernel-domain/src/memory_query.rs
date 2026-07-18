use crate::authorization::AuthorizationDecisionReference;
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{RuntimeId, WorkflowId};
use crate::memory_projection::MemoryProjection;
use crate::memory_record::{MemoryClassification, MemoryRecord, MemoryRetentionPolicyReference};
use crate::memory_validation::{reject_duplicates, require_allowed, scopes_are_compatible};
use crate::ownership::OwnershipPath;
use crate::{ExecutionSessionId, MemoryRecordReference, TaskInstanceReference};

#[derive(Debug, Clone, PartialEq, Eq)]
enum MemoryQueryKind {
    RecordReferences(Vec<MemoryRecordReference>),
    Workflow(WorkflowId),
    Task(TaskInstanceReference),
    ExecutionSession(ExecutionSessionId),
    Runtime(RuntimeId),
    Classification(MemoryClassification),
    RetentionPolicy(MemoryRetentionPolicyReference),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryQuery {
    kind: MemoryQueryKind,
}

impl MemoryQuery {
    pub fn by_record_references(
        memory_record_references: Vec<MemoryRecordReference>,
    ) -> DomainResult<Self> {
        if memory_record_references.is_empty() {
            return Err(DomainError::InvalidMemory(
                "memory query by record references requires at least one reference",
            ));
        }
        reject_duplicates(
            &memory_record_references,
            "duplicate memory record reference in memory query",
        )?;
        Ok(Self {
            kind: MemoryQueryKind::RecordReferences(memory_record_references),
        })
    }
    pub fn by_workflow(workflow_id: WorkflowId) -> Self {
        Self {
            kind: MemoryQueryKind::Workflow(workflow_id),
        }
    }
    pub fn by_task(task_instance_reference: TaskInstanceReference) -> Self {
        Self {
            kind: MemoryQueryKind::Task(task_instance_reference),
        }
    }
    pub fn by_execution_session(execution_session_id: ExecutionSessionId) -> Self {
        Self {
            kind: MemoryQueryKind::ExecutionSession(execution_session_id),
        }
    }
    pub fn by_runtime(runtime_id: RuntimeId) -> Self {
        Self {
            kind: MemoryQueryKind::Runtime(runtime_id),
        }
    }
    pub fn by_classification(memory_classification: MemoryClassification) -> Self {
        Self {
            kind: MemoryQueryKind::Classification(memory_classification),
        }
    }
    pub fn by_retention_policy(
        memory_retention_policy_reference: MemoryRetentionPolicyReference,
    ) -> Self {
        Self {
            kind: MemoryQueryKind::RetentionPolicy(memory_retention_policy_reference),
        }
    }

    pub(crate) fn matches_record(&self, memory_record: &MemoryRecord) -> bool {
        match &self.kind {
            MemoryQueryKind::RecordReferences(memory_record_references) => memory_record_references
                .iter()
                .any(|reference| reference == memory_record.memory_record_reference()),
            MemoryQueryKind::Workflow(workflow_id) => {
                memory_record.memory_provenance().workflow_id() == Some(workflow_id)
            }
            MemoryQueryKind::Task(task_instance_reference) => {
                memory_record.memory_provenance().task_instance_reference()
                    == Some(task_instance_reference)
            }
            MemoryQueryKind::ExecutionSession(execution_session_id) => {
                memory_record.memory_provenance().execution_session_id()
                    == Some(execution_session_id)
            }
            MemoryQueryKind::Runtime(runtime_id) => {
                memory_record.memory_provenance().runtime_id() == Some(runtime_id)
            }
            MemoryQueryKind::Classification(memory_classification) => {
                memory_record.memory_classification() == *memory_classification
            }
            MemoryQueryKind::RetentionPolicy(memory_retention_policy_reference) => {
                memory_record.memory_retention_policy_reference()
                    == memory_retention_policy_reference
            }
        }
    }

    pub(crate) fn matches_projection(&self, memory_projection: &MemoryProjection) -> bool {
        match &self.kind {
            MemoryQueryKind::RecordReferences(memory_record_references) => memory_record_references
                .iter()
                .any(|reference| reference == memory_projection.memory_record_reference()),
            MemoryQueryKind::Workflow(workflow_id) => {
                memory_projection.memory_provenance().workflow_id() == Some(workflow_id)
            }
            MemoryQueryKind::Task(task_instance_reference) => {
                memory_projection
                    .memory_provenance()
                    .task_instance_reference()
                    == Some(task_instance_reference)
            }
            MemoryQueryKind::ExecutionSession(execution_session_id) => {
                memory_projection.memory_provenance().execution_session_id()
                    == Some(execution_session_id)
            }
            MemoryQueryKind::Runtime(runtime_id) => {
                memory_projection.memory_provenance().runtime_id() == Some(runtime_id)
            }
            MemoryQueryKind::Classification(memory_classification) => {
                memory_projection.memory_classification() == *memory_classification
            }
            MemoryQueryKind::RetentionPolicy(memory_retention_policy_reference) => {
                memory_projection.memory_retention_policy_reference()
                    == memory_retention_policy_reference
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRetrievalRequest {
    authorization_decision_reference: AuthorizationDecisionReference,
    ownership_path: OwnershipPath,
    memory_query: MemoryQuery,
}

impl MemoryRetrievalRequest {
    pub fn new(
        authorization_decision_reference: AuthorizationDecisionReference,
        ownership_path: OwnershipPath,
        memory_query: MemoryQuery,
    ) -> DomainResult<Self> {
        require_allowed(&authorization_decision_reference)?;
        Ok(Self {
            authorization_decision_reference,
            ownership_path,
            memory_query,
        })
    }
    pub fn authorization_decision_reference(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision_reference
    }
    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn memory_query(&self) -> &MemoryQuery {
        &self.memory_query
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRetrievalResult {
    memory_retrieval_request: MemoryRetrievalRequest,
    memory_records: Vec<MemoryRecord>,
}

impl MemoryRetrievalResult {
    pub fn new(
        memory_retrieval_request: MemoryRetrievalRequest,
        memory_records: Vec<MemoryRecord>,
    ) -> DomainResult<Self> {
        for memory_record in &memory_records {
            if !scopes_are_compatible(
                memory_retrieval_request.ownership_path(),
                memory_record.ownership_path(),
            ) || !memory_retrieval_request
                .memory_query()
                .matches_record(memory_record)
            {
                return Err(DomainError::InvalidMemory(
                    "memory retrieval result contains a record outside the requested scope or query",
                ));
            }
        }
        Ok(Self {
            memory_retrieval_request,
            memory_records,
        })
    }
    pub fn memory_retrieval_request(&self) -> &MemoryRetrievalRequest {
        &self.memory_retrieval_request
    }
    pub fn memory_records(&self) -> &[MemoryRecord] {
        &self.memory_records
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryQueryResult {
    memory_query: MemoryQuery,
    memory_projections: Vec<MemoryProjection>,
}

impl MemoryQueryResult {
    pub fn new(
        memory_query: MemoryQuery,
        memory_projections: Vec<MemoryProjection>,
    ) -> DomainResult<Self> {
        for memory_projection in &memory_projections {
            if !memory_query.matches_projection(memory_projection) {
                return Err(DomainError::InvalidMemory(
                    "memory query result contains a projection outside the requested query",
                ));
            }
        }
        Ok(Self {
            memory_query,
            memory_projections,
        })
    }
    pub fn memory_query(&self) -> &MemoryQuery {
        &self.memory_query
    }
    pub fn memory_projections(&self) -> &[MemoryProjection] {
        &self.memory_projections
    }
}
