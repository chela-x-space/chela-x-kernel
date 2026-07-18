use super::{
    TaskDependencyControl, TaskDependencyRejectionReason, TaskDependencyRequirement,
    TaskDependencyType, TaskDependencyValidation, TaskEvidenceId, TaskEvidenceReference,
    TaskOutputContract,
};

#[test]
fn task_dependency_valid_construction_preserves_identity_and_direction() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.valid",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );

    assert_eq!(
        dependency
            .task_dependency_reference()
            .task_dependency_id()
            .as_str(),
        "task.dependency.valid"
    );
    assert_eq!(
        dependency
            .task_dependency_source()
            .task_instance_reference()
            .task_instance_id()
            .as_str(),
        "task.instance.predecessor"
    );
    assert_eq!(
        dependency
            .task_dependency_target()
            .task_instance_reference()
            .task_instance_id()
            .as_str(),
        "task.instance.dependent"
    );
}

#[test]
fn task_dependency_same_input_produces_equal_dependency() {
    let left = super::dependency_test_support::task_dependency(
        "task.dependency.equal",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let right = super::dependency_test_support::task_dependency(
        "task.dependency.equal",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );

    assert_eq!(left, right);
}

#[test]
fn task_dependency_different_dependency_is_not_equal() {
    let left = super::dependency_test_support::task_dependency(
        "task.dependency.left",
        "task.instance.a",
        "task.instance.b",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let right = super::dependency_test_support::task_dependency(
        "task.dependency.right",
        "task.instance.a",
        "task.instance.b",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );

    assert_ne!(left, right);
}

#[test]
fn task_dependency_self_dependency_is_rejected() {
    let error = super::TaskDependency::new(
        super::TaskDependencyReference::new(
            super::TaskDependencyId::new("task.dependency.self").expect("dependency id"),
        ),
        super::TaskDependencySource::new(super::dependency_test_support::task_instance_reference(
            "task.instance.same",
        )),
        super::TaskDependencyTarget::new(super::dependency_test_support::task_instance_reference(
            "task.instance.same",
        )),
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    )
    .expect_err("self dependency must fail");

    assert_eq!(
        error.to_string(),
        "invalid task dependency: task dependency must not reference the same source and target task"
    );
}

#[test]
fn task_dependency_requirement_type_mismatch_is_rejected() {
    let error = super::TaskDependency::new(
        super::TaskDependencyReference::new(
            super::TaskDependencyId::new("task.dependency.mismatch").expect("dependency id"),
        ),
        super::TaskDependencySource::new(super::dependency_test_support::task_instance_reference(
            "task.instance.source",
        )),
        super::TaskDependencyTarget::new(super::dependency_test_support::task_instance_reference(
            "task.instance.target",
        )),
        TaskDependencyType::Output,
        TaskDependencyRequirement::SuccessfulCompletion,
    )
    .expect_err("mismatch must fail");

    assert_eq!(
        error.to_string(),
        "invalid task dependency: task dependency requirement does not match dependency type"
    );
}

#[test]
fn task_dependency_duplicate_same_identity_semantic_edge_returns_noop() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.noop",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![dependency.clone()]),
        dependency,
    );

    assert!(matches!(
        TaskDependencyControl::validate(&request),
        TaskDependencyValidation::NoOp(_)
    ));
}

#[test]
fn task_dependency_duplicate_semantic_edge_with_different_identity_is_rejected() {
    let current = super::dependency_test_support::task_dependency(
        "task.dependency.current",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Evidence,
        TaskDependencyRequirement::Evidence(TaskEvidenceReference::new(
            TaskEvidenceId::new("task.evidence.required").expect("evidence id"),
        )),
    );
    let requested = super::dependency_test_support::task_dependency(
        "task.dependency.requested",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Evidence,
        TaskDependencyRequirement::Evidence(TaskEvidenceReference::new(
            TaskEvidenceId::new("task.evidence.required").expect("evidence id"),
        )),
    );
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![current]),
        requested,
    );

    match TaskDependencyControl::validate(&request) {
        TaskDependencyValidation::Rejected(rejected) => {
            assert_eq!(
                rejected.reason(),
                TaskDependencyRejectionReason::DuplicateDependency
            );
        }
        other => panic!("expected rejected validation, got {other:?}"),
    }
}

#[test]
fn task_dependency_ordering_is_preserved_in_accepted_set() {
    let first = super::dependency_test_support::task_dependency(
        "task.dependency.first",
        "task.instance.a",
        "task.instance.b",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let second = super::dependency_test_support::task_dependency(
        "task.dependency.second",
        "task.instance.b",
        "task.instance.c",
        TaskDependencyType::Output,
        TaskDependencyRequirement::Output(
            TaskOutputContract::new("output.review").expect("output"),
        ),
    );
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![first.clone()]),
        second,
    );

    match TaskDependencyControl::validate(&request) {
        TaskDependencyValidation::Accepted(accepted) => {
            assert_eq!(accepted.task_dependency_set().task_dependencies()[0], first);
        }
        other => panic!("expected accepted validation, got {other:?}"),
    }
}
