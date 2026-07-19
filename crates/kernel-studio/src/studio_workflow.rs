use kernel_domain::{
    ExecutionSessionId, TaskInstanceReference, WorkflowFailureCode, WorkflowStateSnapshot,
    WorkflowStepReference,
};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};
use crate::studio_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioWorkflowProjection {
    workflow_state_snapshot: WorkflowStateSnapshot,
    current_step_reference: Option<WorkflowStepReference>,
    completed_step_references: Vec<WorkflowStepReference>,
    blocked_step_references: Vec<WorkflowStepReference>,
    task_instance_references: Vec<TaskInstanceReference>,
    execution_session_ids: Vec<ExecutionSessionId>,
    workflow_failure_code: Option<WorkflowFailureCode>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioWorkflowProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_state_snapshot: WorkflowStateSnapshot,
        current_step_reference: Option<WorkflowStepReference>,
        completed_step_references: Vec<WorkflowStepReference>,
        blocked_step_references: Vec<WorkflowStepReference>,
        task_instance_references: Vec<TaskInstanceReference>,
        execution_session_ids: Vec<ExecutionSessionId>,
        workflow_failure_code: Option<WorkflowFailureCode>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        reject_duplicates(
            &completed_step_references,
            StudioErrorCode::ProjectionMismatch,
            "duplicate completed workflow step in studio workflow projection",
        )?;
        reject_duplicates(
            &blocked_step_references,
            StudioErrorCode::ProjectionMismatch,
            "duplicate blocked workflow step in studio workflow projection",
        )?;
        if let Some(current_step_reference) = &current_step_reference {
            if completed_step_references.contains(current_step_reference)
                || blocked_step_references.contains(current_step_reference)
            {
                return Err(StudioError::new(
                    StudioErrorCode::WorkflowProjectionMismatch,
                    "current workflow step cannot also be completed or blocked",
                )?);
            }
        }
        Ok(Self {
            workflow_state_snapshot,
            current_step_reference,
            completed_step_references,
            blocked_step_references,
            task_instance_references,
            execution_session_ids,
            workflow_failure_code,
            studio_audit_reference,
        })
    }

    pub fn workflow_state_snapshot(&self) -> &WorkflowStateSnapshot {
        &self.workflow_state_snapshot
    }
    pub fn current_step_reference(&self) -> Option<&WorkflowStepReference> {
        self.current_step_reference.as_ref()
    }
    pub fn completed_step_references(&self) -> &[WorkflowStepReference] {
        &self.completed_step_references
    }
    pub fn blocked_step_references(&self) -> &[WorkflowStepReference] {
        &self.blocked_step_references
    }
    pub fn task_instance_references(&self) -> &[TaskInstanceReference] {
        &self.task_instance_references
    }
    pub fn execution_session_ids(&self) -> &[ExecutionSessionId] {
        &self.execution_session_ids
    }
    pub fn workflow_failure_code(&self) -> Option<&WorkflowFailureCode> {
        self.workflow_failure_code.as_ref()
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
