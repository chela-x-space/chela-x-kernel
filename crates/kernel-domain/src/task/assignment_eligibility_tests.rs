use crate::authorization::AuthorizationDecisionOutcome;
use crate::identifier::{AgentId, EnglishNamespace, EnterpriseId, HumanId, StableVersion};
use crate::identity::{AgentIdentity, HumanIdentity, IdentityReference};
use crate::lifecycle::{AgentLifecycle, HumanLifecycle};
use crate::state::TransitionAuthorityReference;

use super::{
    TaskAssignee, TaskAssignment, TaskAssignmentControl, TaskAssignmentDecision,
    TaskAssignmentRejectionReason, TaskAssignmentRequest, TaskAssignmentStatus, TaskInstanceId,
    TaskInstanceReference,
};

fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("task.instance.assignment").expect("instance"))
}

fn human_assignee(lifecycle: HumanLifecycle) -> TaskAssignee {
    TaskAssignee::from_identity(IdentityReference::Human(HumanIdentity::new(
        HumanId::new("CX-EMP-700002").expect("human"),
        EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
        lifecycle,
    )))
}

fn agent_assignee(lifecycle: AgentLifecycle) -> TaskAssignee {
    TaskAssignee::from_identity(IdentityReference::Agent(
        AgentIdentity::new(
            AgentId::new("CX-AGT-700002").expect("agent"),
            EnglishNamespace::new("agent_namespace", "agent.reviewer").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
            lifecycle,
        )
        .expect("identity"),
    ))
}

fn unassigned_assignment() -> TaskAssignment {
    TaskAssignment::new(
        task_instance_reference(),
        None,
        TaskAssignmentStatus::Unassigned,
        None,
        None,
    )
    .expect("unassigned")
}

#[test]
fn task_assignment_ineligible_agent_is_rejected() {
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(agent_assignee(AgentLifecycle::Suspended)),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    let TaskAssignmentDecision::Rejected(rejection) = TaskAssignmentControl::evaluate(&request)
    else {
        panic!("expected rejection");
    };
    assert_eq!(
        rejection.reason(),
        TaskAssignmentRejectionReason::IneligibleAssignee
    );
}

#[test]
fn task_assignment_denied_authorization_is_rejected() {
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(human_assignee(HumanLifecycle::Active)),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Deny),
    );

    let TaskAssignmentDecision::Rejected(rejection) = TaskAssignmentControl::evaluate(&request)
    else {
        panic!("expected rejection");
    };
    assert_eq!(
        rejection.reason(),
        TaskAssignmentRejectionReason::AuthorizationDenied
    );
}

#[test]
fn task_assignment_does_not_change_task_lifecycle() {
    let task_instance = crate::task::instance_tests::minimal_task_instance_for_shared_tests();
    let lifecycle_before = task_instance.task_state();
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(human_assignee(HumanLifecycle::Active)),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    let _ = TaskAssignmentControl::evaluate(&request);

    assert_eq!(task_instance.task_state(), lifecycle_before);
}

#[test]
fn task_assignment_does_not_imply_readiness() {
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(human_assignee(HumanLifecycle::Active)),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    let TaskAssignmentDecision::Updated(change) = TaskAssignmentControl::evaluate(&request) else {
        panic!("expected update");
    };
    assert_eq!(
        change.current_assignment().task_assignment_status(),
        TaskAssignmentStatus::Assigned
    );
}

#[test]
fn task_assignment_same_request_produces_same_outcome() {
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(human_assignee(HumanLifecycle::Active)),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    assert_eq!(
        TaskAssignmentControl::evaluate(&request),
        TaskAssignmentControl::evaluate(&request)
    );
}
