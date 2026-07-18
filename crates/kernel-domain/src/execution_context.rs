use crate::delegation::DelegationReference;
use crate::errors::{DomainError, DomainResult};
use crate::runtime::{LeaseValidity, PresenceState, RuntimeStateSnapshot};
use crate::{
    ExecutionSessionId, TaskInputBinding, TaskInstanceReference, TaskStepReference,
    TaskWorkflowReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionContext {
    execution_session_id: ExecutionSessionId,
    task_instance_reference: TaskInstanceReference,
    runtime_state_snapshot: RuntimeStateSnapshot,
    delegation_reference: Option<DelegationReference>,
    task_workflow_reference: Option<TaskWorkflowReference>,
    task_step_reference: Option<TaskStepReference>,
    task_input_bindings: Vec<TaskInputBinding>,
}

impl ExecutionContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        execution_session_id: ExecutionSessionId,
        task_instance_reference: TaskInstanceReference,
        runtime_state_snapshot: RuntimeStateSnapshot,
        delegation_reference: Option<DelegationReference>,
        task_workflow_reference: Option<TaskWorkflowReference>,
        task_step_reference: Option<TaskStepReference>,
        task_input_bindings: Vec<TaskInputBinding>,
    ) -> DomainResult<Self> {
        if task_step_reference.is_some() && task_workflow_reference.is_none() {
            return Err(DomainError::InvalidExecution(
                "execution step reference requires workflow reference",
            ));
        }
        if matches!(
            runtime_state_snapshot.presence(),
            PresenceState::Offline | PresenceState::Retired
        ) || runtime_state_snapshot.lease_assessment().validity() != LeaseValidity::Valid
        {
            return Err(DomainError::InvalidExecution(
                "execution context requires an operational runtime with a valid lease",
            ));
        }
        reject_duplicates(&task_input_bindings, "duplicate task input binding")?;
        Ok(Self {
            execution_session_id,
            task_instance_reference,
            runtime_state_snapshot,
            delegation_reference,
            task_workflow_reference,
            task_step_reference,
            task_input_bindings,
        })
    }

    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn runtime_state_snapshot(&self) -> &RuntimeStateSnapshot {
        &self.runtime_state_snapshot
    }
    pub fn delegation_reference(&self) -> Option<&DelegationReference> {
        self.delegation_reference.as_ref()
    }
    pub fn task_workflow_reference(&self) -> Option<&TaskWorkflowReference> {
        self.task_workflow_reference.as_ref()
    }
    pub fn task_step_reference(&self) -> Option<&TaskStepReference> {
        self.task_step_reference.as_ref()
    }
    pub fn task_input_bindings(&self) -> &[TaskInputBinding] {
        &self.task_input_bindings
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
