use crate::authorization::AuthorizationDecisionOutcome;
use crate::identity::IdentityReference;
use crate::lifecycle::{AgentLifecycle, HumanLifecycle};

use super::{subject::TaskAssignee, TaskAssignmentRejectionReason};

pub(super) fn validate_assignment_eligibility(
    task_assignee: &TaskAssignee,
    authorization_outcome: Option<AuthorizationDecisionOutcome>,
) -> Result<(), TaskAssignmentRejectionReason> {
    if authorization_outcome.is_some_and(AuthorizationDecisionOutcome::is_denied) {
        return Err(TaskAssignmentRejectionReason::AuthorizationDenied);
    }

    match task_assignee.identity_reference() {
        Some(IdentityReference::Human(identity))
            if matches!(
                identity.lifecycle(),
                HumanLifecycle::Retirement | HumanLifecycle::Archive
            ) =>
        {
            Err(TaskAssignmentRejectionReason::IneligibleAssignee)
        }
        Some(IdentityReference::Agent(identity))
            if matches!(
                identity.lifecycle(),
                AgentLifecycle::Paused
                    | AgentLifecycle::Suspended
                    | AgentLifecycle::Recovering
                    | AgentLifecycle::Retired
                    | AgentLifecycle::Deleted
            ) =>
        {
            Err(TaskAssignmentRejectionReason::IneligibleAssignee)
        }
        _ => Ok(()),
    }
}
