use super::{
    TaskDependencyControl, TaskDependencyRejectionReason, TaskDependencyRequirement,
    TaskDependencyStatus, TaskDependencyType, TaskState,
};

#[test]
fn task_dependency_all_dependencies_satisfied_aggregate_to_satisfied() {
    let request = super::dependency_test_support::coordination_request(
        vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.one",
                "task.instance.a",
                "task.instance.z",
                TaskDependencyType::Success,
                TaskDependencyRequirement::SuccessfulCompletion,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.two",
                "task.instance.b",
                "task.instance.z",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ],
        vec![
            super::dependency_test_support::task_dependency_fact(
                "task.instance.a",
                TaskState::Completed,
                Vec::new(),
                Vec::new(),
            ),
            super::dependency_test_support::task_dependency_fact(
                "task.instance.b",
                TaskState::Cancelled,
                Vec::new(),
                Vec::new(),
            ),
        ],
    );

    assert_eq!(
        TaskDependencyControl::evaluate(&request).task_dependency_status(),
        TaskDependencyStatus::Satisfied
    );
}

#[test]
fn task_dependency_unsatisfied_dependency_preserves_blocker_ordering() {
    let request = super::dependency_test_support::coordination_request(
        vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.first",
                "task.instance.a",
                "task.instance.z",
                TaskDependencyType::Success,
                TaskDependencyRequirement::SuccessfulCompletion,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.second",
                "task.instance.b",
                "task.instance.z",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ],
        vec![
            super::dependency_test_support::task_dependency_fact(
                "task.instance.a",
                TaskState::Failed,
                Vec::new(),
                Vec::new(),
            ),
            super::dependency_test_support::task_dependency_fact(
                "task.instance.b",
                TaskState::Pending,
                Vec::new(),
                Vec::new(),
            ),
        ],
    );
    let decision = TaskDependencyControl::evaluate(&request);

    assert_eq!(
        decision.task_dependency_status(),
        TaskDependencyStatus::Unsatisfied
    );
    assert_eq!(
        decision.task_dependency_decisions()[0]
            .task_dependency()
            .task_dependency_reference()
            .task_dependency_id()
            .as_str(),
        "task.dependency.first"
    );
}

#[test]
fn task_dependency_unresolved_dependency_aggregates_to_unresolved() {
    let request = super::dependency_test_support::coordination_request(
        vec![
            super::dependency_test_support::task_dependency(
                "task.dependency.one",
                "task.instance.a",
                "task.instance.z",
                TaskDependencyType::Success,
                TaskDependencyRequirement::SuccessfulCompletion,
            ),
            super::dependency_test_support::task_dependency(
                "task.dependency.two",
                "task.instance.b",
                "task.instance.z",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            ),
        ],
        vec![super::dependency_test_support::task_dependency_fact(
            "task.instance.a",
            TaskState::Completed,
            Vec::new(),
            Vec::new(),
        )],
    );

    assert_eq!(
        TaskDependencyControl::evaluate(&request).task_dependency_status(),
        TaskDependencyStatus::Unresolved
    );
}

#[test]
fn task_dependency_structurally_invalid_set_is_rejected() {
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
        "task.instance.a",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let decision = TaskDependencyControl::evaluate(
        &super::dependency_test_support::coordination_request(vec![first, second], Vec::new()),
    );

    assert_eq!(
        decision.task_dependency_status(),
        TaskDependencyStatus::Rejected
    );
    assert_eq!(
        decision.task_dependency_rejection_reason(),
        Some(TaskDependencyRejectionReason::DependencyCycle)
    );
}

#[test]
fn task_dependency_duplicate_fact_is_rejected_at_request_construction() {
    let fact = super::dependency_test_support::task_dependency_fact(
        "task.instance.a",
        TaskState::Completed,
        Vec::new(),
        Vec::new(),
    );
    let error = super::TaskDependencyCoordinationRequest::new(
        super::dependency_test_support::task_dependency_set(Vec::new()),
        vec![fact.clone(), fact],
    )
    .expect_err("duplicate fact must fail");

    assert_eq!(
        error.to_string(),
        "invalid task dependency: duplicate task dependency fact for predecessor task"
    );
}
