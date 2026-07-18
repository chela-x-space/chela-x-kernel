use super::{TaskDependency, TaskDependencySet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyBlocker {
    RequiredCompletionMissing,
    RequiredSuccessMissing,
    RequiredEvidenceMissing,
    RequiredOutputMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyUnresolvedReason {
    MissingDependencyFact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyRejectionReason {
    DuplicateDependency,
    DependencyCycle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyValidationAccepted {
    task_dependency_set: TaskDependencySet,
}

impl TaskDependencyValidationAccepted {
    pub(crate) fn new(task_dependency_set: TaskDependencySet) -> Self {
        Self {
            task_dependency_set,
        }
    }
    pub fn task_dependency_set(&self) -> &TaskDependencySet {
        &self.task_dependency_set
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyValidationNoOp {
    task_dependency_set: TaskDependencySet,
}

impl TaskDependencyValidationNoOp {
    pub(crate) fn new(task_dependency_set: TaskDependencySet) -> Self {
        Self {
            task_dependency_set,
        }
    }
    pub fn task_dependency_set(&self) -> &TaskDependencySet {
        &self.task_dependency_set
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyValidationRejected {
    task_dependency_set: TaskDependencySet,
    requested_task_dependency: TaskDependency,
    reason: TaskDependencyRejectionReason,
}

impl TaskDependencyValidationRejected {
    pub(crate) fn new(
        task_dependency_set: TaskDependencySet,
        requested_task_dependency: TaskDependency,
        reason: TaskDependencyRejectionReason,
    ) -> Self {
        Self {
            task_dependency_set,
            requested_task_dependency,
            reason,
        }
    }
    pub fn task_dependency_set(&self) -> &TaskDependencySet {
        &self.task_dependency_set
    }
    pub fn requested_task_dependency(&self) -> &TaskDependency {
        &self.requested_task_dependency
    }
    pub fn reason(&self) -> TaskDependencyRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskDependencyValidation {
    Accepted(TaskDependencyValidationAccepted),
    Rejected(TaskDependencyValidationRejected),
    NoOp(TaskDependencyValidationNoOp),
}
