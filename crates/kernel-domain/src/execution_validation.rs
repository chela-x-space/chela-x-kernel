use crate::errors::{DomainError, DomainResult};
use crate::request::TimeReference;
use crate::{
    ExecutionAuditReference, ExecutionContext, ExecutionEvidenceBinding, ExecutionRequest,
    ExecutionSession, TaskFailure, TaskState, TaskStateSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExecutionValidation;

impl ExecutionValidation {
    pub fn validate_request(request: &ExecutionRequest) -> DomainResult<()> {
        if request.task_state_snapshot().task_state() != TaskState::InProgress {
            return Err(DomainError::InvalidExecution(
                "execution request requires an InProgress task state snapshot",
            ));
        }
        Ok(())
    }

    pub fn validate_context(context: &ExecutionContext) -> DomainResult<()> {
        if context
            .runtime_state_snapshot()
            .runtime_id()
            .as_str()
            .is_empty()
        {
            return Err(DomainError::InvalidExecution(
                "execution context requires a runtime identity",
            ));
        }
        Ok(())
    }

    pub fn validate_session(session: &ExecutionSession) -> DomainResult<()> {
        if session.execution_request().task_instance_reference()
            != session.execution_context().task_instance_reference()
        {
            return Err(DomainError::InvalidExecution(
                "execution session request and context must reference the same task instance",
            ));
        }
        Ok(())
    }

    pub fn validate_failed_snapshot(
        task_state_snapshot: &TaskStateSnapshot,
        task_failure: &TaskFailure,
        ended_at: &TimeReference,
    ) -> DomainResult<()> {
        if task_state_snapshot.task_instance_reference() != task_failure.task_instance_reference() {
            return Err(DomainError::InvalidExecution(
                "failed execution validation requires task snapshot and failure to match",
            ));
        }
        if ended_at.as_str().is_empty() {
            return Err(DomainError::InvalidExecution(
                "execution termination time must be explicit",
            ));
        }
        Ok(())
    }

    pub fn validate_reference_only_audit(
        execution_audit_reference: &ExecutionAuditReference,
        execution_evidence_binding: &ExecutionEvidenceBinding,
    ) -> DomainResult<()> {
        if execution_audit_reference.execution_session_id()
            != execution_evidence_binding.execution_session_id()
        {
            return Err(DomainError::InvalidExecution(
                "execution audit and evidence bindings must preserve execution session continuity",
            ));
        }
        Ok(())
    }
}
