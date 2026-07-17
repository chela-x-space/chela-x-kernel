use crate::errors::{DomainError, DomainResult};

use super::instance_validation::validate_task_instance_bindings;
use super::{
    TaskCreationContext, TaskDefinition, TaskDefinitionReference, TaskDefinitionSnapshotReference,
    TaskInstanceId, TaskOutputBinding, TaskState, TaskStepBinding, TaskWorkflowBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskInstance {
    task_instance_id: TaskInstanceId,
    task_definition: TaskDefinition,
    task_definition_snapshot_reference: TaskDefinitionSnapshotReference,
    task_creation_context: TaskCreationContext,
    task_output_bindings: Vec<TaskOutputBinding>,
    task_workflow_binding: Option<TaskWorkflowBinding>,
    task_step_binding: Option<TaskStepBinding>,
    task_state: TaskState,
}

impl TaskInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_instance_id: TaskInstanceId,
        task_definition: TaskDefinition,
        task_creation_context: TaskCreationContext,
        task_output_bindings: Vec<TaskOutputBinding>,
        task_workflow_binding: Option<TaskWorkflowBinding>,
        task_step_binding: Option<TaskStepBinding>,
        task_state: TaskState,
    ) -> DomainResult<Self> {
        if task_state != TaskState::Pending {
            return Err(DomainError::InvalidTaskInstance(
                "task instance initial state must be Pending",
            ));
        }

        validate_task_instance_bindings(
            &task_definition,
            &task_creation_context,
            &task_output_bindings,
            task_workflow_binding.as_ref(),
            task_step_binding.as_ref(),
        )?;

        let task_definition_snapshot_reference = TaskDefinitionSnapshotReference::new(
            TaskDefinitionReference::new(task_definition.task_definition_id().clone()),
            task_definition.task_definition_version().clone(),
        );

        Ok(Self {
            task_instance_id,
            task_definition,
            task_definition_snapshot_reference,
            task_creation_context,
            task_output_bindings,
            task_workflow_binding,
            task_step_binding,
            task_state,
        })
    }

    pub fn task_instance_id(&self) -> &TaskInstanceId {
        &self.task_instance_id
    }

    pub fn task_definition(&self) -> &TaskDefinition {
        &self.task_definition
    }

    pub fn task_definition_snapshot_reference(&self) -> &TaskDefinitionSnapshotReference {
        &self.task_definition_snapshot_reference
    }

    pub fn task_creation_context(&self) -> &TaskCreationContext {
        &self.task_creation_context
    }

    pub fn task_output_bindings(&self) -> &[TaskOutputBinding] {
        &self.task_output_bindings
    }

    pub fn task_workflow_binding(&self) -> Option<&TaskWorkflowBinding> {
        self.task_workflow_binding.as_ref()
    }

    pub fn task_step_binding(&self) -> Option<&TaskStepBinding> {
        self.task_step_binding.as_ref()
    }

    pub fn task_state(&self) -> TaskState {
        self.task_state
    }
}
