use crate::authorization::AuthorizationDecisionOutcome;
use crate::identifier::{AgentId, EnglishNamespace, EnterpriseId, HumanId, StableVersion};
use crate::identity::{AgentIdentity, HumanIdentity, IdentityReference};
use crate::lifecycle::{AgentLifecycle, HumanLifecycle};
use crate::state::TransitionAuthorityReference;

use super::{
    TaskAssignee, TaskAssignment, TaskAssignmentControl, TaskAssignmentDecision,
    TaskAssignmentReason, TaskAssignmentRejectionReason, TaskAssignmentRequest,
    TaskAssignmentStatus, TaskInstanceId, TaskInstanceReference,
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
fn task_assignment_same_assignee_request_is_deterministic_no_op() {
    let assignee = human_assignee(HumanLifecycle::Active);
    let assignment = TaskAssignment::new(
        task_instance_reference(),
        Some(assignee.clone()),
        TaskAssignmentStatus::Assigned,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("assignment");
    let request = TaskAssignmentRequest::new(
        assignment.clone(),
        Some(assignee),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    assert!(matches!(
        TaskAssignmentControl::evaluate(&request),
        TaskAssignmentDecision::NoOp(_)
    ));
}

#[test]
fn task_assignment_reassignment_preserves_history_in_outcome() {
    let current = TaskAssignment::new(
        task_instance_reference(),
        Some(human_assignee(HumanLifecycle::Active)),
        TaskAssignmentStatus::Assigned,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("assignment");
    let request = TaskAssignmentRequest::new(
        current.clone(),
        Some(agent_assignee(AgentLifecycle::Active)),
        Some(TransitionAuthorityReference::new("authority.reassign").expect("authority")),
        Some(TaskAssignmentReason::new("reassignment").expect("reason")),
        Some(AuthorizationDecisionOutcome::Allow),
    );

    let decision = TaskAssignmentControl::evaluate(&request);
    let TaskAssignmentDecision::Updated(change) = decision else {
        panic!("expected update")
    };
    assert_eq!(change.previous_assignment(), &current);
}

#[test]
fn task_assignment_unassignment_is_deterministic() {
    let current = TaskAssignment::new(
        task_instance_reference(),
        Some(human_assignee(HumanLifecycle::Active)),
        TaskAssignmentStatus::Assigned,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("assignment");
    let request = TaskAssignmentRequest::new(
        current,
        None,
        Some(TransitionAuthorityReference::new("authority.release").expect("authority")),
        Some(TaskAssignmentReason::new("released").expect("reason")),
        None,
    );

    let TaskAssignmentDecision::Updated(change) = TaskAssignmentControl::evaluate(&request) else {
        panic!("expected update");
    };
    assert_eq!(
        change.current_assignment().task_assignment_status(),
        TaskAssignmentStatus::Released
    );
}

#[test]
fn task_assignment_missing_authority_is_rejected() {
    let request = TaskAssignmentRequest::new(
        unassigned_assignment(),
        Some(human_assignee(HumanLifecycle::Active)),
        None,
        None,
        Some(AuthorizationDecisionOutcome::Allow),
    );

    let TaskAssignmentDecision::Rejected(rejection) = TaskAssignmentControl::evaluate(&request)
    else {
        panic!("expected rejection");
    };
    assert_eq!(
        rejection.reason(),
        TaskAssignmentRejectionReason::MissingAuthority
    );
}

#[test]
fn task_assignment_unassignment_without_reason_is_rejected() {
    let request = TaskAssignmentRequest::new(
        TaskAssignment::new(
            task_instance_reference(),
            Some(human_assignee(HumanLifecycle::Active)),
            TaskAssignmentStatus::Assigned,
            Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
            None,
        )
        .expect("assignment"),
        None,
        Some(TransitionAuthorityReference::new("authority.release").expect("authority")),
        None,
        None,
    );

    let TaskAssignmentDecision::Rejected(rejection) = TaskAssignmentControl::evaluate(&request)
    else {
        panic!("expected rejection");
    };
    assert_eq!(
        rejection.reason(),
        TaskAssignmentRejectionReason::MissingReason
    );
}
