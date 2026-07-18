use super::{
    TaskDependencyControl, TaskDependencyRequirement, TaskDependencyStatus, TaskDependencyType,
    TaskState,
};

#[test]
fn task_dependency_evaluation_does_not_mutate_task_state_or_readiness() {
    let snapshot = super::lifecycle_test_support::task_state_snapshot(TaskState::Pending);
    let readiness = super::readiness_test_support::ready_input();
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.sep",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );

    let _decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![dependency],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Completed,
                Vec::new(),
                Vec::new(),
            )],
        ));

    assert_eq!(snapshot.task_state(), TaskState::Pending);
    assert_eq!(readiness.task_state(), TaskState::Pending);
}

#[test]
fn task_dependency_evaluation_does_not_change_assignment_or_start_work() {
    let assignment = super::lifecycle_test_support::accepted_assignment();
    let assignment_before = assignment.clone();
    let decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![super::dependency_test_support::task_dependency(
                "task.dependency.sep2",
                "task.instance.predecessor",
                "task.instance.dependent",
                TaskDependencyType::Completion,
                TaskDependencyRequirement::AnyTerminal,
            )],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Completed,
                Vec::new(),
                Vec::new(),
            )],
        ));

    assert_eq!(assignment, assignment_before);
    assert_eq!(
        decision.task_dependency_status(),
        TaskDependencyStatus::Satisfied
    );
}
