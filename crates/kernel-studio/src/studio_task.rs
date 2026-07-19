use kernel_domain::{
    ExecutionSessionId, TaskAssignee, TaskFailureReference, TaskInstanceReference, TaskOwner,
    TaskPriority, TaskStateSnapshot, TaskWorkflowReference,
};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioTaskProjection {
    task_state_snapshot: TaskStateSnapshot,
    task_priority: Option<TaskPriority>,
    task_owner: Option<TaskOwner>,
    task_assignee: Option<TaskAssignee>,
    task_workflow_reference: Option<TaskWorkflowReference>,
    execution_session_id: Option<ExecutionSessionId>,
    task_failure_reference: Option<TaskFailureReference>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioTaskProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_state_snapshot: TaskStateSnapshot,
        task_priority: Option<TaskPriority>,
        task_owner: Option<TaskOwner>,
        task_assignee: Option<TaskAssignee>,
        task_workflow_reference: Option<TaskWorkflowReference>,
        execution_session_id: Option<ExecutionSessionId>,
        task_failure_reference: Option<TaskFailureReference>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if let Some(task_priority) = &task_priority {
            if task_priority.task_instance_reference()
                != task_state_snapshot.task_instance_reference()
            {
                return Err(StudioError::new(
                    StudioErrorCode::TaskProjectionMismatch,
                    "studio task priority must match the projected task identity",
                )?);
            }
        }
        Ok(Self {
            task_state_snapshot,
            task_priority,
            task_owner,
            task_assignee,
            task_workflow_reference,
            execution_session_id,
            task_failure_reference,
            studio_audit_reference,
        })
    }

    pub fn task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.task_state_snapshot
    }
    pub fn task_priority(&self) -> Option<&TaskPriority> {
        self.task_priority.as_ref()
    }
    pub fn task_owner(&self) -> Option<&TaskOwner> {
        self.task_owner.as_ref()
    }
    pub fn task_assignee(&self) -> Option<&TaskAssignee> {
        self.task_assignee.as_ref()
    }
    pub fn task_workflow_reference(&self) -> Option<&TaskWorkflowReference> {
        self.task_workflow_reference.as_ref()
    }
    pub fn execution_session_id(&self) -> Option<&ExecutionSessionId> {
        self.execution_session_id.as_ref()
    }
    pub fn task_failure_reference(&self) -> Option<&TaskFailureReference> {
        self.task_failure_reference.as_ref()
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        self.task_state_snapshot.task_instance_reference()
    }
}
