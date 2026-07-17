use crate::identifier::WorkflowId;
use crate::workflow::WorkflowStepReference;

use super::{TaskDefinitionId, TaskDependencyId, TaskEvidenceId, TaskInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskDefinitionReference {
    task_definition_id: TaskDefinitionId,
}

impl TaskDefinitionReference {
    pub fn new(task_definition_id: TaskDefinitionId) -> Self {
        Self { task_definition_id }
    }

    pub fn task_definition_id(&self) -> &TaskDefinitionId {
        &self.task_definition_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskInstanceReference {
    task_instance_id: TaskInstanceId,
}

impl TaskInstanceReference {
    pub fn new(task_instance_id: TaskInstanceId) -> Self {
        Self { task_instance_id }
    }

    pub fn task_instance_id(&self) -> &TaskInstanceId {
        &self.task_instance_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskDependencyReference {
    task_dependency_id: TaskDependencyId,
}

impl TaskDependencyReference {
    pub fn new(task_dependency_id: TaskDependencyId) -> Self {
        Self { task_dependency_id }
    }

    pub fn task_dependency_id(&self) -> &TaskDependencyId {
        &self.task_dependency_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskEvidenceReference {
    task_evidence_id: TaskEvidenceId,
}

impl TaskEvidenceReference {
    pub fn new(task_evidence_id: TaskEvidenceId) -> Self {
        Self { task_evidence_id }
    }

    pub fn task_evidence_id(&self) -> &TaskEvidenceId {
        &self.task_evidence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskWorkflowReference {
    workflow_id: WorkflowId,
}

impl TaskWorkflowReference {
    pub fn new(workflow_id: WorkflowId) -> Self {
        Self { workflow_id }
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskStepReference {
    workflow_step_reference: WorkflowStepReference,
}

impl TaskStepReference {
    pub fn new(workflow_step_reference: WorkflowStepReference) -> Self {
        Self {
            workflow_step_reference,
        }
    }

    pub fn workflow_step_reference(&self) -> &WorkflowStepReference {
        &self.workflow_step_reference
    }
}
