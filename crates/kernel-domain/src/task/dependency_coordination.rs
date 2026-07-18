use super::{
    TaskDependency, TaskDependencyGraphReference, TaskDependencyRejectionReason,
    TaskDependencyStatus, TaskDependencyUnresolvedReason,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyDecision {
    task_dependency: TaskDependency,
    task_dependency_status: TaskDependencyStatus,
    task_dependency_blocker: Option<super::TaskDependencyBlocker>,
    task_dependency_unresolved_reason: Option<TaskDependencyUnresolvedReason>,
}

impl TaskDependencyDecision {
    pub(crate) fn new(
        task_dependency: TaskDependency,
        task_dependency_status: TaskDependencyStatus,
        task_dependency_blocker: Option<super::TaskDependencyBlocker>,
        task_dependency_unresolved_reason: Option<TaskDependencyUnresolvedReason>,
    ) -> Self {
        Self {
            task_dependency,
            task_dependency_status,
            task_dependency_blocker,
            task_dependency_unresolved_reason,
        }
    }

    pub fn task_dependency(&self) -> &TaskDependency {
        &self.task_dependency
    }
    pub fn task_dependency_status(&self) -> TaskDependencyStatus {
        self.task_dependency_status
    }
    pub fn task_dependency_blocker(&self) -> Option<super::TaskDependencyBlocker> {
        self.task_dependency_blocker
    }
    pub fn task_dependency_unresolved_reason(&self) -> Option<TaskDependencyUnresolvedReason> {
        self.task_dependency_unresolved_reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyCoordinationDecision {
    task_dependency_graph_reference: TaskDependencyGraphReference,
    task_dependency_status: TaskDependencyStatus,
    task_dependency_decisions: Vec<TaskDependencyDecision>,
    task_dependency_rejection_reason: Option<TaskDependencyRejectionReason>,
}

impl TaskDependencyCoordinationDecision {
    pub(crate) fn new(
        task_dependency_graph_reference: TaskDependencyGraphReference,
        task_dependency_status: TaskDependencyStatus,
        task_dependency_decisions: Vec<TaskDependencyDecision>,
        task_dependency_rejection_reason: Option<TaskDependencyRejectionReason>,
    ) -> Self {
        Self {
            task_dependency_graph_reference,
            task_dependency_status,
            task_dependency_decisions,
            task_dependency_rejection_reason,
        }
    }

    pub fn task_dependency_graph_reference(&self) -> &TaskDependencyGraphReference {
        &self.task_dependency_graph_reference
    }
    pub fn task_dependency_status(&self) -> TaskDependencyStatus {
        self.task_dependency_status
    }
    pub fn task_dependency_decisions(&self) -> &[TaskDependencyDecision] {
        &self.task_dependency_decisions
    }
    pub fn task_dependency_rejection_reason(&self) -> Option<TaskDependencyRejectionReason> {
        self.task_dependency_rejection_reason
    }
}
