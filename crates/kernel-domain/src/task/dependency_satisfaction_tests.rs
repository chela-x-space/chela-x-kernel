use super::{
    TaskDependencyBlocker, TaskDependencyControl, TaskDependencyRequirement, TaskDependencyStatus,
    TaskDependencyType, TaskEvidenceId, TaskEvidenceReference, TaskOutputContract, TaskState,
};

#[test]
fn task_dependency_completion_requirement_is_satisfied_by_terminal_state() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.completion",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Completion,
        TaskDependencyRequirement::AnyTerminal,
    );
    let decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![dependency],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Cancelled,
                Vec::new(),
                Vec::new(),
            )],
        ));

    assert_eq!(
        decision.task_dependency_status(),
        TaskDependencyStatus::Satisfied
    );
}

#[test]
fn task_dependency_success_requirement_rejects_failed_predecessor_as_unsatisfied() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.success",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );
    let decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![dependency],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Failed,
                Vec::new(),
                Vec::new(),
            )],
        ));

    let dependency_decision = &decision.task_dependency_decisions()[0];
    assert_eq!(
        dependency_decision.task_dependency_status(),
        TaskDependencyStatus::Unsatisfied
    );
    assert_eq!(
        dependency_decision.task_dependency_blocker(),
        Some(TaskDependencyBlocker::RequiredSuccessMissing)
    );
}

#[test]
fn task_dependency_evidence_requirement_is_satisfied() {
    let evidence = TaskEvidenceReference::new(
        TaskEvidenceId::new("task.evidence.required").expect("evidence id"),
    );
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.evidence",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Evidence,
        TaskDependencyRequirement::Evidence(evidence.clone()),
    );
    let decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![dependency],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Completed,
                vec![evidence],
                Vec::new(),
            )],
        ));

    assert_eq!(
        decision.task_dependency_status(),
        TaskDependencyStatus::Satisfied
    );
}

#[test]
fn task_dependency_output_requirement_is_unsatisfied_when_missing() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.output",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Output,
        TaskDependencyRequirement::Output(
            TaskOutputContract::new("output.review").expect("output"),
        ),
    );
    let decision =
        TaskDependencyControl::evaluate(&super::dependency_test_support::coordination_request(
            vec![dependency],
            vec![super::dependency_test_support::task_dependency_fact(
                "task.instance.predecessor",
                TaskState::Completed,
                Vec::new(),
                Vec::new(),
            )],
        ));

    let dependency_decision = &decision.task_dependency_decisions()[0];
    assert_eq!(
        dependency_decision.task_dependency_status(),
        TaskDependencyStatus::Unsatisfied
    );
    assert_eq!(
        dependency_decision.task_dependency_blocker(),
        Some(TaskDependencyBlocker::RequiredOutputMissing)
    );
}

#[test]
fn task_dependency_missing_fact_is_unresolved() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.unresolved",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );
    let decision = TaskDependencyControl::evaluate(
        &super::dependency_test_support::coordination_request(vec![dependency], Vec::new()),
    );

    let dependency_decision = &decision.task_dependency_decisions()[0];
    assert_eq!(
        dependency_decision.task_dependency_status(),
        TaskDependencyStatus::Unresolved
    );
    assert_eq!(
        dependency_decision.task_dependency_unresolved_reason(),
        Some(super::TaskDependencyUnresolvedReason::MissingDependencyFact)
    );
}

#[test]
fn task_dependency_same_input_produces_same_decision() {
    let dependency = super::dependency_test_support::task_dependency(
        "task.dependency.same",
        "task.instance.predecessor",
        "task.instance.dependent",
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    );
    let request = super::dependency_test_support::coordination_request(
        vec![dependency],
        vec![super::dependency_test_support::task_dependency_fact(
            "task.instance.predecessor",
            TaskState::Completed,
            Vec::new(),
            Vec::new(),
        )],
    );

    assert_eq!(
        TaskDependencyControl::evaluate(&request),
        TaskDependencyControl::evaluate(&request)
    );
}
