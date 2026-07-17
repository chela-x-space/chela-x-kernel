use crate::errors::{DomainError, DomainResult};

use super::{
    TaskCreationContext, TaskDefinition, TaskOutputBinding, TaskStepBinding, TaskWorkflowBinding,
};

pub(super) fn validate_task_instance_bindings(
    task_definition: &TaskDefinition,
    task_creation_context: &TaskCreationContext,
    task_output_bindings: &[TaskOutputBinding],
    task_workflow_binding: Option<&TaskWorkflowBinding>,
    task_step_binding: Option<&TaskStepBinding>,
) -> DomainResult<()> {
    super::instance_binding::validate_step_binding_presence(
        task_workflow_binding,
        task_step_binding,
    )?;

    for required_input in task_definition.task_input_contracts() {
        if !task_creation_context
            .task_input_bindings()
            .iter()
            .any(|binding| binding.task_input_contract() == required_input)
        {
            return Err(DomainError::InvalidTaskInstance(
                "missing required task input binding",
            ));
        }
    }

    for binding in task_creation_context.task_input_bindings() {
        if !task_definition
            .task_input_contracts()
            .iter()
            .any(|contract| contract == binding.task_input_contract())
        {
            return Err(DomainError::InvalidTaskInstance(
                "task input binding is not declared by task definition",
            ));
        }
    }

    for (index, binding) in task_output_bindings.iter().enumerate() {
        if task_output_bindings[..index]
            .iter()
            .any(|prior| prior == binding)
        {
            return Err(DomainError::InvalidTaskInstance(
                "duplicate task output binding",
            ));
        }
        if !task_definition
            .task_output_contracts()
            .iter()
            .any(|contract| contract == binding.task_output_contract())
        {
            return Err(DomainError::InvalidTaskInstance(
                "task output binding is not declared by task definition",
            ));
        }
    }

    if let (Some(definition_reference), Some(binding)) = (
        task_definition.task_workflow_reference(),
        task_workflow_binding,
    ) {
        if binding.workflow_id() != definition_reference.workflow_id() {
            return Err(DomainError::InvalidTaskInstance(
                "task workflow binding does not match task definition workflow binding",
            ));
        }
    }

    if let (Some(definition_reference), Some(binding)) =
        (task_definition.task_step_reference(), task_step_binding)
    {
        if binding.task_step_reference() != definition_reference {
            return Err(DomainError::InvalidTaskInstance(
                "task step binding does not match task definition step binding",
            ));
        }
    }

    Ok(())
}
