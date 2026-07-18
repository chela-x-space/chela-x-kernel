use crate::errors::{DomainError, DomainResult};
use crate::identifier::{RuntimeId, WorkflowId};
use crate::memory_record::{
    MemoryClassification, MemoryProvenance, MemoryRecord, MemoryRetentionPolicyReference,
};
use crate::{
    ExecutionSessionId, MemoryAuditReference, MemoryRecordReference, TaskInstanceReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryProjection {
    memory_record_reference: MemoryRecordReference,
    memory_summary: String,
    memory_classification: MemoryClassification,
    memory_provenance: MemoryProvenance,
    memory_retention_policy_reference: MemoryRetentionPolicyReference,
    memory_audit_reference: MemoryAuditReference,
}

impl MemoryProjection {
    pub fn new(memory_record: &MemoryRecord) -> Self {
        Self {
            memory_record_reference: memory_record.memory_record_reference().clone(),
            memory_summary: memory_record.memory_summary().to_owned(),
            memory_classification: memory_record.memory_classification(),
            memory_provenance: memory_record.memory_provenance().clone(),
            memory_retention_policy_reference: memory_record
                .memory_retention_policy_reference()
                .clone(),
            memory_audit_reference: memory_record.memory_audit_reference().clone(),
        }
    }

    pub fn memory_record_reference(&self) -> &MemoryRecordReference {
        &self.memory_record_reference
    }
    pub fn memory_summary(&self) -> &str {
        &self.memory_summary
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowMemoryProjection {
    memory_projection: MemoryProjection,
    workflow_id: WorkflowId,
}

impl WorkflowMemoryProjection {
    pub fn new(memory_record: &MemoryRecord) -> DomainResult<Self> {
        let memory_projection = MemoryProjection::new(memory_record);
        let Some(workflow_id) = memory_projection.memory_provenance().workflow_id().cloned() else {
            return Err(DomainError::InvalidMemory(
                "workflow memory projection requires workflow provenance",
            ));
        };
        Ok(Self {
            memory_projection,
            workflow_id,
        })
    }
    pub fn memory_projection(&self) -> &MemoryProjection {
        &self.memory_projection
    }
    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskMemoryProjection {
    memory_projection: MemoryProjection,
    task_instance_reference: TaskInstanceReference,
}

impl TaskMemoryProjection {
    pub fn new(memory_record: &MemoryRecord) -> DomainResult<Self> {
        let memory_projection = MemoryProjection::new(memory_record);
        let Some(task_instance_reference) = memory_projection
            .memory_provenance()
            .task_instance_reference()
            .cloned()
        else {
            return Err(DomainError::InvalidMemory(
                "task memory projection requires task provenance",
            ));
        };
        Ok(Self {
            memory_projection,
            task_instance_reference,
        })
    }
    pub fn memory_projection(&self) -> &MemoryProjection {
        &self.memory_projection
    }
    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionMemoryProjection {
    memory_projection: MemoryProjection,
    execution_session_id: ExecutionSessionId,
}

impl ExecutionMemoryProjection {
    pub fn new(memory_record: &MemoryRecord) -> DomainResult<Self> {
        let memory_projection = MemoryProjection::new(memory_record);
        let Some(execution_session_id) = memory_projection
            .memory_provenance()
            .execution_session_id()
            .cloned()
        else {
            return Err(DomainError::InvalidMemory(
                "execution memory projection requires execution provenance",
            ));
        };
        Ok(Self {
            memory_projection,
            execution_session_id,
        })
    }
    pub fn memory_projection(&self) -> &MemoryProjection {
        &self.memory_projection
    }
    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeMemoryProjection {
    memory_projection: MemoryProjection,
    runtime_id: RuntimeId,
}

impl RuntimeMemoryProjection {
    pub fn new(memory_record: &MemoryRecord) -> DomainResult<Self> {
        let memory_projection = MemoryProjection::new(memory_record);
        let Some(runtime_id) = memory_projection.memory_provenance().runtime_id().cloned() else {
            return Err(DomainError::InvalidMemory(
                "runtime memory projection requires runtime provenance",
            ));
        };
        Ok(Self {
            memory_projection,
            runtime_id,
        })
    }
    pub fn memory_projection(&self) -> &MemoryProjection {
        &self.memory_projection
    }
    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }
}
