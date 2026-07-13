use crate::errors::{DomainError, DomainResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterpriseLifecycle {
    Proposed,
    Active,
    Suspended,
    Dissolved,
}

impl EnterpriseLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Dissolved)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceLifecycle {
    Planned,
    Provisioning,
    Active,
    Archived,
    Retired,
}

impl WorkspaceLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectLifecycle {
    Draft,
    Approved,
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl ProjectLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationalUnitLifecycle {
    Proposed,
    Established,
    Operating,
    Merged,
    Closed,
}

impl OrganizationalUnitLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Merged | Self::Closed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipLifecycle {
    Draft,
    Active,
    Transferred,
    Revoked,
    Expired,
}

impl OwnershipLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Revoked | Self::Expired)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanLifecycle {
    Candidate,
    Evaluation,
    Training,
    Certification,
    Registration,
    Active,
    Promotion,
    Transfer,
    Maintenance,
    Retirement,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentLifecycle {
    Created,
    Registered,
    Verified,
    Approved,
    Active,
    Paused,
    Suspended,
    Recovering,
    Retired,
    Deleted,
}

impl AgentLifecycle {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Retired | Self::Deleted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionRecordStatus {
    Draft,
    PendingReview,
    PendingApproval,
    Approved,
    Rejected,
    Executed,
    Superseded,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegationLifecycle {
    Draft,
    Requested,
    PolicyValidated,
    AuthorizationValidated,
    PendingAcceptance,
    Accepted,
    Active,
    Suspended,
    Revoked,
    Expired,
    Completed,
    Rejected,
    Archived,
}

impl DelegationLifecycle {
    pub fn can_transition_to(self, target: Self) -> DomainResult<()> {
        let allowed = matches!(
            (self, target),
            (Self::Draft, Self::Requested)
                | (Self::Requested, Self::PolicyValidated | Self::Rejected)
                | (
                    Self::PolicyValidated,
                    Self::AuthorizationValidated | Self::Rejected
                )
                | (
                    Self::AuthorizationValidated,
                    Self::PendingAcceptance | Self::Active | Self::Rejected
                )
                | (Self::PendingAcceptance, Self::Accepted | Self::Rejected)
                | (Self::Accepted, Self::Active | Self::Rejected)
                | (
                    Self::Active,
                    Self::Suspended | Self::Revoked | Self::Expired | Self::Completed
                )
                | (
                    Self::Suspended,
                    Self::Active | Self::Revoked | Self::Expired
                )
                | (Self::Completed, Self::Archived)
                | (Self::Rejected, Self::Archived)
                | (Self::Revoked, Self::Archived)
                | (Self::Expired, Self::Archived)
        );
        if allowed {
            Ok(())
        } else {
            Err(DomainError::InvalidLifecycleTransition {
                lifecycle: "DelegationLifecycle",
                from: self.as_str(),
                to: target.as_str(),
            })
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Requested => "Requested",
            Self::PolicyValidated => "PolicyValidated",
            Self::AuthorizationValidated => "AuthorizationValidated",
            Self::PendingAcceptance => "PendingAcceptance",
            Self::Accepted => "Accepted",
            Self::Active => "Active",
            Self::Suspended => "Suspended",
            Self::Revoked => "Revoked",
            Self::Expired => "Expired",
            Self::Completed => "Completed",
            Self::Rejected => "Rejected",
            Self::Archived => "Archived",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowState {
    Draft,
    Defined,
    Validated,
    Approved,
    Ready,
    Running,
    Paused,
    Waiting,
    Completed,
    Failed,
    Cancelled,
    Archived,
}

impl WorkflowState {
    pub fn can_transition_to(self, target: Self) -> DomainResult<()> {
        let allowed = matches!(
            (self, target),
            (Self::Draft, Self::Defined)
                | (Self::Defined, Self::Validated | Self::Archived)
                | (
                    Self::Validated,
                    Self::Approved | Self::Defined | Self::Archived
                )
                | (Self::Approved, Self::Ready | Self::Archived)
                | (
                    Self::Ready,
                    Self::Running | Self::Cancelled | Self::Archived
                )
                | (
                    Self::Running,
                    Self::Paused | Self::Waiting | Self::Completed | Self::Failed | Self::Cancelled
                )
                | (
                    Self::Paused,
                    Self::Running | Self::Cancelled | Self::Archived
                )
                | (
                    Self::Waiting,
                    Self::Running | Self::Failed | Self::Cancelled | Self::Archived
                )
                | (Self::Failed, Self::Ready | Self::Archived)
                | (Self::Completed, Self::Archived)
                | (Self::Cancelled, Self::Archived)
        );
        if allowed {
            Ok(())
        } else {
            Err(DomainError::InvalidLifecycleTransition {
                lifecycle: "WorkflowState",
                from: self.as_str(),
                to: target.as_str(),
            })
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Defined => "Defined",
            Self::Validated => "Validated",
            Self::Approved => "Approved",
            Self::Ready => "Ready",
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::Waiting => "Waiting",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::Archived => "Archived",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentLifecycle, DelegationLifecycle, WorkflowState};

    #[test]
    fn delegation_allows_specified_transition_ces_b0_029_11() {
        assert!(DelegationLifecycle::Draft
            .can_transition_to(DelegationLifecycle::Requested)
            .is_ok());
    }

    #[test]
    fn delegation_rejects_unspecified_transition_ces_b0_029_11() {
        assert!(DelegationLifecycle::Draft
            .can_transition_to(DelegationLifecycle::Active)
            .is_err());
    }

    #[test]
    fn workflow_terminal_state_rejects_reactivation_ces_b0_030_9() {
        assert!(WorkflowState::Archived
            .can_transition_to(WorkflowState::Running)
            .is_err());
    }

    #[test]
    fn workflow_allows_documented_transition_ces_b0_030_9() {
        assert!(WorkflowState::Ready
            .can_transition_to(WorkflowState::Running)
            .is_ok());
    }

    #[test]
    fn agent_terminal_states_are_explicit_ces_b0_027_7() {
        assert!(AgentLifecycle::Deleted.is_terminal());
        assert!(AgentLifecycle::Retired.is_terminal());
        assert!(!AgentLifecycle::Active.is_terminal());
    }
}
