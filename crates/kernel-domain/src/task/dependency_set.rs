use crate::errors::{DomainError, DomainResult};

use super::{TaskDependency, TaskDependencyFact, TaskDependencyGraphReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencySet {
    task_dependency_graph_reference: TaskDependencyGraphReference,
    task_dependencies: Vec<TaskDependency>,
}

impl TaskDependencySet {
    pub fn new(
        task_dependency_graph_reference: TaskDependencyGraphReference,
        task_dependencies: Vec<TaskDependency>,
    ) -> Self {
        Self {
            task_dependency_graph_reference,
            task_dependencies,
        }
    }

    pub fn task_dependency_graph_reference(&self) -> &TaskDependencyGraphReference {
        &self.task_dependency_graph_reference
    }
    pub fn task_dependencies(&self) -> &[TaskDependency] {
        &self.task_dependencies
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyValidationRequest {
    current_task_dependency_set: TaskDependencySet,
    requested_task_dependency: TaskDependency,
}

impl TaskDependencyValidationRequest {
    pub fn new(
        current_task_dependency_set: TaskDependencySet,
        requested_task_dependency: TaskDependency,
    ) -> Self {
        Self {
            current_task_dependency_set,
            requested_task_dependency,
        }
    }

    pub fn current_task_dependency_set(&self) -> &TaskDependencySet {
        &self.current_task_dependency_set
    }
    pub fn requested_task_dependency(&self) -> &TaskDependency {
        &self.requested_task_dependency
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDependencyCoordinationRequest {
    task_dependency_set: TaskDependencySet,
    task_dependency_facts: Vec<TaskDependencyFact>,
}

impl TaskDependencyCoordinationRequest {
    pub fn new(
        task_dependency_set: TaskDependencySet,
        task_dependency_facts: Vec<TaskDependencyFact>,
    ) -> DomainResult<Self> {
        if task_dependency_facts
            .iter()
            .enumerate()
            .any(|(index, fact)| {
                task_dependency_facts[..index].iter().any(|prior| {
                    prior.task_state_snapshot().task_instance_reference()
                        == fact.task_state_snapshot().task_instance_reference()
                })
            })
        {
            return Err(DomainError::InvalidTaskDependency(
                "duplicate task dependency fact for predecessor task",
            ));
        }

        Ok(Self {
            task_dependency_set,
            task_dependency_facts,
        })
    }

    pub fn task_dependency_set(&self) -> &TaskDependencySet {
        &self.task_dependency_set
    }
    pub fn task_dependency_facts(&self) -> &[TaskDependencyFact] {
        &self.task_dependency_facts
    }
}
