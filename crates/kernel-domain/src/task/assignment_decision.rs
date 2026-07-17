use crate::authorization::AuthorizationDecisionOutcome;

use super::{
    subject::TaskAssignee, TaskAssignment, TaskAssignmentAuthority, TaskAssignmentReason,
    TaskAssignmentRejectionReason,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignmentRequest {
    current_assignment: TaskAssignment,
    requested_assignee: Option<TaskAssignee>,
    task_assignment_authority: Option<TaskAssignmentAuthority>,
    task_assignment_reason: Option<TaskAssignmentReason>,
    authorization_outcome: Option<AuthorizationDecisionOutcome>,
}

impl TaskAssignmentRequest {
    pub fn new(
        current_assignment: TaskAssignment,
        requested_assignee: Option<TaskAssignee>,
        task_assignment_authority: Option<TaskAssignmentAuthority>,
        task_assignment_reason: Option<TaskAssignmentReason>,
        authorization_outcome: Option<AuthorizationDecisionOutcome>,
    ) -> Self {
        Self {
            current_assignment,
            requested_assignee,
            task_assignment_authority,
            task_assignment_reason,
            authorization_outcome,
        }
    }

    pub fn current_assignment(&self) -> &TaskAssignment {
        &self.current_assignment
    }
    pub fn requested_assignee(&self) -> Option<&TaskAssignee> {
        self.requested_assignee.as_ref()
    }
    pub fn task_assignment_authority(&self) -> Option<&TaskAssignmentAuthority> {
        self.task_assignment_authority.as_ref()
    }
    pub fn task_assignment_reason(&self) -> Option<&TaskAssignmentReason> {
        self.task_assignment_reason.as_ref()
    }
    pub fn authorization_outcome(&self) -> Option<AuthorizationDecisionOutcome> {
        self.authorization_outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignmentChange {
    previous_assignment: TaskAssignment,
    current_assignment: TaskAssignment,
}

impl TaskAssignmentChange {
    pub(crate) fn new(
        previous_assignment: TaskAssignment,
        current_assignment: TaskAssignment,
    ) -> Self {
        Self {
            previous_assignment,
            current_assignment,
        }
    }

    pub fn previous_assignment(&self) -> &TaskAssignment {
        &self.previous_assignment
    }
    pub fn current_assignment(&self) -> &TaskAssignment {
        &self.current_assignment
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignmentRejection {
    current_assignment: TaskAssignment,
    reason: TaskAssignmentRejectionReason,
}

impl TaskAssignmentRejection {
    pub(crate) fn new(
        current_assignment: TaskAssignment,
        reason: TaskAssignmentRejectionReason,
    ) -> Self {
        Self {
            current_assignment,
            reason,
        }
    }

    pub fn current_assignment(&self) -> &TaskAssignment {
        &self.current_assignment
    }
    pub fn reason(&self) -> TaskAssignmentRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignmentNoOp {
    current_assignment: TaskAssignment,
}

impl TaskAssignmentNoOp {
    pub(crate) fn new(current_assignment: TaskAssignment) -> Self {
        Self { current_assignment }
    }

    pub fn current_assignment(&self) -> &TaskAssignment {
        &self.current_assignment
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskAssignmentDecision {
    Updated(TaskAssignmentChange),
    Rejected(TaskAssignmentRejection),
    NoOp(TaskAssignmentNoOp),
}
