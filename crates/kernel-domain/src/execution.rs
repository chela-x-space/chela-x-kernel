use crate::errors::{DomainError, DomainResult};
use crate::event::EventTraceReference;
use crate::identifier::{AuditEvidenceId, CorrelationId};
use crate::state::TransitionEvidenceReference;
use crate::{TaskEvidenceReference, TaskInstanceReference, TaskOutputReference};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionSessionId(EventTraceReference);

impl ExecutionSessionId {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        EventTraceReference::new(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEvidenceBinding {
    execution_session_id: ExecutionSessionId,
    task_instance_reference: TaskInstanceReference,
    task_evidence_references: Vec<TaskEvidenceReference>,
    task_output_references: Vec<TaskOutputReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
}

impl ExecutionEvidenceBinding {
    pub fn new(
        execution_session_id: ExecutionSessionId,
        task_instance_reference: TaskInstanceReference,
        task_evidence_references: Vec<TaskEvidenceReference>,
        task_output_references: Vec<TaskOutputReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
    ) -> DomainResult<Self> {
        if task_evidence_references.is_empty()
            && task_output_references.is_empty()
            && transition_evidence_references.is_empty()
        {
            return Err(DomainError::InvalidExecution(
                "execution evidence binding requires at least one evidence or output reference",
            ));
        }
        reject_duplicates(
            &task_evidence_references,
            "duplicate task evidence reference",
        )?;
        reject_duplicates(&task_output_references, "duplicate task output reference")?;
        reject_duplicates(
            &transition_evidence_references,
            "duplicate transition evidence reference",
        )?;
        Ok(Self {
            execution_session_id,
            task_instance_reference,
            task_evidence_references,
            task_output_references,
            transition_evidence_references,
        })
    }

    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_evidence_references(&self) -> &[TaskEvidenceReference] {
        &self.task_evidence_references
    }
    pub fn task_output_references(&self) -> &[TaskOutputReference] {
        &self.task_output_references
    }
    pub fn transition_evidence_references(&self) -> &[TransitionEvidenceReference] {
        &self.transition_evidence_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAuditReference {
    execution_session_id: ExecutionSessionId,
    correlation_id: Option<CorrelationId>,
    audit_evidence_ids: Vec<AuditEvidenceId>,
}

impl ExecutionAuditReference {
    pub fn new(
        execution_session_id: ExecutionSessionId,
        correlation_id: Option<CorrelationId>,
        audit_evidence_ids: Vec<AuditEvidenceId>,
    ) -> DomainResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(DomainError::InvalidExecution(
                "execution audit reference requires audit evidence identifiers",
            ));
        }
        reject_duplicates(&audit_evidence_ids, "duplicate audit evidence identity")?;
        Ok(Self {
            execution_session_id,
            correlation_id,
            audit_evidence_ids,
        })
    }

    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }
    pub fn audit_evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.audit_evidence_ids
    }
}

fn reject_duplicates<T: PartialEq>(values: &[T], message: &'static str) -> DomainResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(DomainError::InvalidExecution(message));
    }
    Ok(())
}
