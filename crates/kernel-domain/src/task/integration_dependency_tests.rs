use super::{TaskDependencyStatus, TaskReadinessBlocker, TaskReadinessDecision};

#[test]
fn integration_dependency_satisfied_supports_readiness_ready() {
    let dependency = super::integration_test_support::satisfied_dependency_decision();
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::DependenciesComplete,
            super::TaskReadinessEvidence::AuthorizationAllowed,
            super::TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
    );

    assert_eq!(
        dependency.task_dependency_status(),
        TaskDependencyStatus::Satisfied
    );
    assert!(matches!(readiness, TaskReadinessDecision::Ready(_)));
}

#[test]
fn integration_dependency_unsatisfied_blocks_readiness() {
    let dependency = super::integration_test_support::unsatisfied_dependency_decision();
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::AuthorizationAllowed,
            super::TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
    );

    assert_eq!(
        dependency.task_dependency_status(),
        TaskDependencyStatus::Unsatisfied
    );
    assert!(matches!(
        readiness,
        TaskReadinessDecision::Blocked(blocked)
            if blocked.blockers().contains(&TaskReadinessBlocker::DependencyIncomplete)
    ));
}

#[test]
fn integration_unresolved_dependency_does_not_become_satisfied() {
    let dependency = super::integration_test_support::unresolved_dependency_decision();

    assert_eq!(
        dependency.task_dependency_status(),
        TaskDependencyStatus::Unresolved
    );
    assert!(dependency
        .task_dependency_decisions()
        .iter()
        .all(|decision| decision.task_dependency_status() != TaskDependencyStatus::Satisfied));
}

#[test]
fn integration_rejected_dependency_structure_does_not_silently_map_to_readiness() {
    let duplicate_dependency = super::TaskDependency::new(
        super::TaskDependencyReference::new(
            super::TaskDependencyId::new("task.dependency.dup").expect("dependency"),
        ),
        super::TaskDependencySource::new(super::integration_test_support::task_instance_reference()),
        super::TaskDependencyTarget::new(super::integration_test_support::task_instance_reference()),
        super::TaskDependencyType::Success,
        super::TaskDependencyRequirement::SuccessfulCompletion,
    );

    assert!(duplicate_dependency.is_err());
    let readiness = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::AuthorizationAllowed,
            super::TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
    );

    assert!(matches!(
        readiness,
        TaskReadinessDecision::Blocked(blocked)
            if blocked.blockers().contains(&TaskReadinessBlocker::DependencyIncomplete)
    ));
}
