use crate::errors::{DomainError, DomainResult};
use crate::identifier::NonEmptyText;
use crate::state::TransitionAuthorityReference;

use super::{subject::TaskAssignee, TaskInstanceReference};

pub type TaskAssignmentAuthority = TransitionAuthorityReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskAssignmentStatus {
    Unassigned,
    Assigned,
    Accepted,
    Rejected,
    Released,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignmentReason(NonEmptyText);

impl TaskAssignmentReason {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_assignment_reason", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskAssignmentRejectionReason {
    MissingAuthority,
    MissingReason,
    AuthorizationDenied,
    IneligibleAssignee,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignment {
    task_instance_reference: TaskInstanceReference,
    task_assignee: Option<TaskAssignee>,
    task_assignment_status: TaskAssignmentStatus,
    task_assignment_authority: Option<TaskAssignmentAuthority>,
    task_assignment_reason: Option<TaskAssignmentReason>,
}

impl TaskAssignment {
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_assignee: Option<TaskAssignee>,
        task_assignment_status: TaskAssignmentStatus,
        task_assignment_authority: Option<TaskAssignmentAuthority>,
        task_assignment_reason: Option<TaskAssignmentReason>,
    ) -> DomainResult<Self> {
        match task_assignment_status {
            TaskAssignmentStatus::Unassigned
                if task_assignee.is_some()
                    || task_assignment_authority.is_some()
                    || task_assignment_reason.is_some() =>
            {
                return Err(DomainError::InvalidTaskAssignment(
                    "unassigned task must not retain assignee, authority, or reason",
                ));
            }
            TaskAssignmentStatus::Assigned | TaskAssignmentStatus::Accepted
                if task_assignee.is_none() || task_assignment_authority.is_none() =>
            {
                return Err(DomainError::InvalidTaskAssignment(
                    "assigned task requires assignee and authority",
                ));
            }
            TaskAssignmentStatus::Rejected
            | TaskAssignmentStatus::Released
            | TaskAssignmentStatus::Revoked
                if task_assignee.is_some()
                    || task_assignment_authority.is_none()
                    || task_assignment_reason.is_none() =>
            {
                return Err(DomainError::InvalidTaskAssignment(
                    "closed assignment state requires cleared assignee, authority, and reason",
                ));
            }
            _ => {}
        }

        Ok(Self {
            task_instance_reference,
            task_assignee,
            task_assignment_status,
            task_assignment_authority,
            task_assignment_reason,
        })
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_assignee(&self) -> Option<&TaskAssignee> {
        self.task_assignee.as_ref()
    }
    pub fn task_assignment_status(&self) -> TaskAssignmentStatus {
        self.task_assignment_status
    }
    pub fn task_assignment_authority(&self) -> Option<&TaskAssignmentAuthority> {
        self.task_assignment_authority.as_ref()
    }
    pub fn task_assignment_reason(&self) -> Option<&TaskAssignmentReason> {
        self.task_assignment_reason.as_ref()
    }
}
