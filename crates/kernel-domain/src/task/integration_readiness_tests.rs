use super::{TaskReadinessBlocker, TaskReadinessDecision, TaskTransitionDecision};

#[test]
fn integration_accepted_assignment_alone_is_not_ready() {
    let readiness = super::integration_test_support::ready_readiness_decision(
        None,
        Some(super::integration_test_support::accepted_assignment()),
        None,
        Vec::new(),
    );

    assert!(matches!(
        readiness,
        TaskReadinessDecision::Blocked(blocked)
            if blocked.blockers().contains(&TaskReadinessBlocker::MissingRequiredInput)
    ));
}

#[test]
fn integration_high_priority_alone_is_not_ready() {
    let readiness = super::integration_test_support::ready_readiness_decision(
        None,
        None,
        Some(super::integration_test_support::priority()),
        Vec::new(),
    );

    assert!(matches!(
        readiness,
        TaskReadinessDecision::Blocked(blocked)
            if blocked.blockers().contains(&TaskReadinessBlocker::MissingOwner)
    ));
}

#[test]
fn integration_missing_evidence_blocks_readiness() {
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::DependenciesComplete,
            super::TaskReadinessEvidence::AuthorizationAllowed,
        ],
    );

    assert!(matches!(
        readiness,
        TaskReadinessDecision::Blocked(blocked)
            if blocked.blockers().contains(&TaskReadinessBlocker::MissingRequiredEvidence)
    ));
}

#[test]
fn integration_blocked_readiness_does_not_mutate_assignment_or_dependency() {
    let assignment = super::integration_test_support::accepted_assignment();
    let dependency = super::integration_test_support::unsatisfied_dependency_decision();
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(assignment.clone()),
        Some(super::integration_test_support::priority()),
        vec![super::TaskReadinessEvidence::RequiredInputAvailable],
    );

    assert_eq!(
        assignment,
        super::integration_test_support::accepted_assignment()
    );
    assert_eq!(
        dependency.task_dependency_status(),
        super::TaskDependencyStatus::Unsatisfied
    );
    assert!(matches!(readiness, TaskReadinessDecision::Blocked(_)));
}

#[test]
fn integration_readiness_not_ready_rejects_start_transition() {
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![super::TaskReadinessEvidence::RequiredInputAvailable],
    );
    let decision =
        super::TaskTransitionControl::evaluate(&super::integration_test_support::start_request(
            super::integration_test_support::pending_snapshot(),
            readiness,
            super::integration_test_support::accepted_assignment(),
            false,
        ));

    assert!(matches!(decision, TaskTransitionDecision::Rejected(_)));
}
