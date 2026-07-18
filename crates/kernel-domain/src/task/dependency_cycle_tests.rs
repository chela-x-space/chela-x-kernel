use super::{
    TaskDependencyControl, TaskDependencyRejectionReason, TaskDependencyRequirement,
    TaskDependencyType, TaskDependencyValidation,
};

#[test]
fn task_dependency_two_node_cycle_is_rejected() {
    let first = super::dependency_test_support::task_dependency(
        "task.dependency.one",
        "task.instance.a",
        "task.instance.b",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![first]),
        super::dependency_test_support::task_dependency(
            "task.dependency.two",
            "task.instance.b",
            "task.instance.a",
            TaskDependencyType::Completion,
            TaskDependencyRequirement::AnyTerminal,
        ),
    );

    match TaskDependencyControl::validate(&request) {
        TaskDependencyValidation::Rejected(rejected) => {
            assert_eq!(
                rejected.reason(),
                TaskDependencyRejectionReason::DependencyCycle
            );
        }
        other => panic!("expected rejected validation, got {other:?}"),
    }
}

#[test]
fn task_dependency_three_node_cycle_is_rejected() {
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.one",
                "task.instance.a",
                "task.instance.b",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.two",
                "task.instance.b",
                "task.instance.c",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ]),
        super::dependency_test_support::task_dependency(
            "task.dependency.three",
            "task.instance.c",
            "task.instance.a",
            TaskDependencyType::Completion,
            TaskDependencyRequirement::AnyTerminal,
        ),
    );

    assert!(matches!(
        TaskDependencyControl::validate(&request),
        TaskDependencyValidation::Rejected(_)
    ));
}

#[test]
fn task_dependency_acyclic_chain_is_accepted() {
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.one",
                "task.instance.a",
                "task.instance.b",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.two",
                "task.instance.b",
                "task.instance.c",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ]),
        super::dependency_test_support::task_dependency(
            "task.dependency.three",
            "task.instance.c",
            "task.instance.d",
            TaskDependencyType::Completion,
            TaskDependencyRequirement::AnyTerminal,
        ),
    );

    assert!(matches!(
        TaskDependencyControl::validate(&request),
        TaskDependencyValidation::Accepted(_)
    ));
}

#[test]
fn task_dependency_disconnected_acyclic_groups_are_accepted() {
    let request = super::TaskDependencyValidationRequest::new(
        super::dependency_test_support::task_dependency_set(vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.one",
                "task.instance.a",
                "task.instance.b",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.two",
                "task.instance.c",
                "task.instance.d",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ]),
        super::dependency_test_support::task_dependency(
            "task.dependency.three",
            "task.instance.e",
            "task.instance.f",
            TaskDependencyType::Completion,
            TaskDependencyRequirement::AnyTerminal,
        ),
    );

    assert!(matches!(
        TaskDependencyControl::validate(&request),
        TaskDependencyValidation::Accepted(_)
    ));
}
