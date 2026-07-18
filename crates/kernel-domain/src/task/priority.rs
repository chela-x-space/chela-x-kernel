use crate::errors::{DomainError, DomainResult};

use super::TaskInstanceReference;

const EXPLICIT_PRIORITY_CLASS: &str = "Explicit";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriorityClass {
    Explicit,
}

impl TaskPriorityClass {
    pub fn new(value: &str) -> DomainResult<Self> {
        if value == EXPLICIT_PRIORITY_CLASS {
            Ok(Self::Explicit)
        } else {
            Err(DomainError::InvalidTaskPriority(
                "unsupported task priority class",
            ))
        }
    }

    pub fn as_str(self) -> &'static str {
        EXPLICIT_PRIORITY_CLASS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskPriorityValue(u8);

impl TaskPriorityValue {
    pub fn new(value: u8) -> DomainResult<Self> {
        if value == 0 {
            Err(DomainError::InvalidTaskPriority(
                "task priority value must be greater than zero",
            ))
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskPriority {
    task_instance_reference: TaskInstanceReference,
    task_priority_class: TaskPriorityClass,
    task_priority_value: TaskPriorityValue,
}

impl TaskPriority {
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_priority_class: TaskPriorityClass,
        task_priority_value: TaskPriorityValue,
    ) -> Self {
        Self {
            task_instance_reference,
            task_priority_class,
            task_priority_value,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }

    pub fn task_priority_class(&self) -> TaskPriorityClass {
        self.task_priority_class
    }

    pub fn task_priority_value(&self) -> TaskPriorityValue {
        self.task_priority_value
    }
}
