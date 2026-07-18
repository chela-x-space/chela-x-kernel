use crate::errors::{DomainError, DomainResult};
use crate::request::TimeReference;
use crate::{
    ExecutionAuditReference, ExecutionContext, ExecutionEvidenceBinding, ExecutionRequest,
    ExecutionSessionId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSession {
    execution_session_id: ExecutionSessionId,
    execution_request: ExecutionRequest,
    execution_context: ExecutionContext,
    execution_evidence_binding: ExecutionEvidenceBinding,
    execution_audit_reference: ExecutionAuditReference,
    started_at: TimeReference,
}

impl ExecutionSession {
    pub fn new(
        execution_request: ExecutionRequest,
        execution_context: ExecutionContext,
        execution_evidence_binding: ExecutionEvidenceBinding,
        execution_audit_reference: ExecutionAuditReference,
        started_at: TimeReference,
    ) -> DomainResult<Self> {
        let execution_session_id = execution_request.execution_session_id().clone();
        if execution_context.execution_session_id() != &execution_session_id
            || execution_evidence_binding.execution_session_id() != &execution_session_id
            || execution_audit_reference.execution_session_id() != &execution_session_id
        {
            return Err(DomainError::InvalidExecution(
                "execution session identifiers must remain continuous across request, context, evidence, and audit",
            ));
        }
        if execution_context.task_instance_reference()
            != execution_request.task_instance_reference()
            || execution_evidence_binding.task_instance_reference()
                != execution_request.task_instance_reference()
        {
            return Err(DomainError::InvalidExecution(
                "execution session task instance references must remain continuous",
            ));
        }
        Ok(Self {
            execution_session_id,
            execution_request,
            execution_context,
            execution_evidence_binding,
            execution_audit_reference,
            started_at,
        })
    }

    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
    pub fn execution_request(&self) -> &ExecutionRequest {
        &self.execution_request
    }
    pub fn execution_context(&self) -> &ExecutionContext {
        &self.execution_context
    }
    pub fn execution_evidence_binding(&self) -> &ExecutionEvidenceBinding {
        &self.execution_evidence_binding
    }
    pub fn execution_audit_reference(&self) -> &ExecutionAuditReference {
        &self.execution_audit_reference
    }
    pub fn started_at(&self) -> &TimeReference {
        &self.started_at
    }
}
