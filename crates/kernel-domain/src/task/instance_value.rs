use super::{
    TaskDefinitionReference, TaskDefinitionVersion, TaskInputContract, TaskOutputContract,
    TaskStepReference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Archived,
}

impl TaskState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::InProgress => "InProgress",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::Archived => "Archived",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDefinitionSnapshotReference {
    task_definition_reference: TaskDefinitionReference,
    task_definition_version: TaskDefinitionVersion,
}

impl TaskDefinitionSnapshotReference {
    pub fn new(
        task_definition_reference: TaskDefinitionReference,
        task_definition_version: TaskDefinitionVersion,
    ) -> Self {
        Self {
            task_definition_reference,
            task_definition_version,
        }
    }

    pub fn task_definition_reference(&self) -> &TaskDefinitionReference {
        &self.task_definition_reference
    }

    pub fn task_definition_version(&self) -> &TaskDefinitionVersion {
        &self.task_definition_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskInputBinding(TaskInputContract);

impl TaskInputBinding {
    pub fn new(task_input_contract: TaskInputContract) -> Self {
        Self(task_input_contract)
    }

    pub fn task_input_contract(&self) -> &TaskInputContract {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskOutputBinding(TaskOutputContract);

impl TaskOutputBinding {
    pub fn new(task_output_contract: TaskOutputContract) -> Self {
        Self(task_output_contract)
    }

    pub fn task_output_contract(&self) -> &TaskOutputContract {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStepBinding {
    task_step_reference: TaskStepReference,
}

impl TaskStepBinding {
    pub fn new(task_step_reference: TaskStepReference) -> Self {
        Self {
            task_step_reference,
        }
    }

    pub fn task_step_reference(&self) -> &TaskStepReference {
        &self.task_step_reference
    }
}
