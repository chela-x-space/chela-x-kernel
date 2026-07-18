use crate::errors::{DomainError, DomainResult};
use crate::identifier::NonEmptyText;

use super::{TaskEvidenceReference, TaskInstanceReference, TaskOutputContract};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyGraphReference(NonEmptyText);

impl TaskDependencyGraphReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_dependency_graph_reference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskDependencySource(TaskInstanceReference);

impl TaskDependencySource {
    pub fn new(task_instance_reference: TaskInstanceReference) -> Self {
        Self(task_instance_reference)
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskDependencyTarget(TaskInstanceReference);

impl TaskDependencyTarget {
    pub fn new(task_instance_reference: TaskInstanceReference) -> Self {
        Self(task_instance_reference)
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyType {
    Completion,
    Success,
    Evidence,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskDependencyRequirement {
    AnyTerminal,
    SuccessfulCompletion,
    Evidence(TaskEvidenceReference),
    Output(TaskOutputContract),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyStatus {
    Satisfied,
    Unsatisfied,
    Unresolved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependency {
    task_dependency_reference: super::TaskDependencyReference,
    task_dependency_source: TaskDependencySource,
    task_dependency_target: TaskDependencyTarget,
    task_dependency_type: TaskDependencyType,
    task_dependency_requirement: TaskDependencyRequirement,
}

impl TaskDependency {
    pub fn new(
        task_dependency_reference: super::TaskDependencyReference,
        task_dependency_source: TaskDependencySource,
        task_dependency_target: TaskDependencyTarget,
        task_dependency_type: TaskDependencyType,
        task_dependency_requirement: TaskDependencyRequirement,
    ) -> DomainResult<Self> {
        if task_dependency_source.task_instance_reference()
            == task_dependency_target.task_instance_reference()
        {
            return Err(DomainError::InvalidTaskDependency(
                "task dependency must not reference the same source and target task",
            ));
        }
        if !matches_requirement(task_dependency_type, &task_dependency_requirement) {
            return Err(DomainError::InvalidTaskDependency(
                "task dependency requirement does not match dependency type",
            ));
        }

        Ok(Self {
            task_dependency_reference,
            task_dependency_source,
            task_dependency_target,
            task_dependency_type,
            task_dependency_requirement,
        })
    }

    pub fn task_dependency_reference(&self) -> &super::TaskDependencyReference {
        &self.task_dependency_reference
    }
    pub fn task_dependency_source(&self) -> &TaskDependencySource {
        &self.task_dependency_source
    }
    pub fn task_dependency_target(&self) -> &TaskDependencyTarget {
        &self.task_dependency_target
    }
    pub fn task_dependency_type(&self) -> TaskDependencyType {
        self.task_dependency_type
    }
    pub fn task_dependency_requirement(&self) -> &TaskDependencyRequirement {
        &self.task_dependency_requirement
    }
}

fn matches_requirement(
    task_dependency_type: TaskDependencyType,
    task_dependency_requirement: &TaskDependencyRequirement,
) -> bool {
    matches!(
        (task_dependency_type, task_dependency_requirement),
        (
            TaskDependencyType::Completion,
            TaskDependencyRequirement::AnyTerminal
                | TaskDependencyRequirement::SuccessfulCompletion
        ) | (
            TaskDependencyType::Success,
            TaskDependencyRequirement::SuccessfulCompletion
        ) | (
            TaskDependencyType::Evidence,
            TaskDependencyRequirement::Evidence(_)
        ) | (
            TaskDependencyType::Output,
            TaskDependencyRequirement::Output(_)
        )
    )
}
