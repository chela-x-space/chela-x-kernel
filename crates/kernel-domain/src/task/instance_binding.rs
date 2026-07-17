use crate::errors::{DomainError, DomainResult};
use crate::state::TransitionAuthorityReference;
use crate::workflow::{WorkflowDefinition, WorkflowInstance};

use super::{TaskInputBinding, TaskStepBinding};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCreationContext {
    task_input_bindings: Vec<TaskInputBinding>,
    task_creation_authority: Option<TransitionAuthorityReference>,
}

impl TaskCreationContext {
    pub fn new(
        task_input_bindings: Vec<TaskInputBinding>,
        task_creation_authority: Option<TransitionAuthorityReference>,
    ) -> DomainResult<Self> {
        for (index, binding) in task_input_bindings.iter().enumerate() {
            if task_input_bindings[..index]
                .iter()
                .any(|prior| prior == binding)
            {
                return Err(DomainError::InvalidTaskInstance(
                    "duplicate task input binding",
                ));
            }
        }

        Ok(Self {
            task_input_bindings,
            task_creation_authority,
        })
    }

    pub fn task_input_bindings(&self) -> &[TaskInputBinding] {
        &self.task_input_bindings
    }
    pub fn task_creation_authority(&self) -> Option<&TransitionAuthorityReference> {
        self.task_creation_authority.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskWorkflowBinding {
    WorkflowDefinition(Box<WorkflowDefinition>),
    WorkflowInstance(Box<WorkflowInstance>),
}

impl TaskWorkflowBinding {
    pub fn from_workflow_definition(workflow_definition: WorkflowDefinition) -> Self {
        Self::WorkflowDefinition(Box::new(workflow_definition))
    }

    pub fn from_workflow_instance(workflow_instance: WorkflowInstance) -> Self {
        Self::WorkflowInstance(Box::new(workflow_instance))
    }

    pub fn workflow_definition(&self) -> Option<&WorkflowDefinition> {
        match self {
            Self::WorkflowDefinition(workflow_definition) => Some(workflow_definition),
            Self::WorkflowInstance(_) => None,
        }
    }

    pub fn workflow_instance(&self) -> Option<&WorkflowInstance> {
        match self {
            Self::WorkflowDefinition(_) => None,
            Self::WorkflowInstance(workflow_instance) => Some(workflow_instance),
        }
    }

    pub fn workflow_id(&self) -> &crate::WorkflowId {
        match self {
            Self::WorkflowDefinition(workflow_definition) => workflow_definition.workflow_id(),
            Self::WorkflowInstance(workflow_instance) => workflow_instance.workflow_id(),
        }
    }
}

pub fn validate_step_binding_presence(
    task_workflow_binding: Option<&TaskWorkflowBinding>,
    task_step_binding: Option<&TaskStepBinding>,
) -> DomainResult<()> {
    if task_step_binding.is_some() && task_workflow_binding.is_none() {
        return Err(DomainError::InvalidTaskInstance(
            "task step binding requires workflow binding",
        ));
    }

    Ok(())
}
