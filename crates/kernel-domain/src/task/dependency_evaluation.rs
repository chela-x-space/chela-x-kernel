use super::{
    dependency_cycle::find_invalid_set_reason, TaskDependencyBlocker,
    TaskDependencyCoordinationDecision, TaskDependencyCoordinationRequest, TaskDependencyDecision,
    TaskDependencyFact, TaskDependencyRequirement, TaskDependencyStatus,
    TaskDependencyUnresolvedReason, TaskState,
};

pub(super) fn evaluate_coordination(
    request: &TaskDependencyCoordinationRequest,
) -> TaskDependencyCoordinationDecision {
    if let Some(reason) = find_invalid_set_reason(request.task_dependency_set().task_dependencies())
    {
        return TaskDependencyCoordinationDecision::new(
            request
                .task_dependency_set()
                .task_dependency_graph_reference()
                .clone(),
            TaskDependencyStatus::Rejected,
            Vec::new(),
            Some(reason),
        );
    }

    let task_dependency_decisions = request
        .task_dependency_set()
        .task_dependencies()
        .iter()
        .cloned()
        .map(|dependency| evaluate_dependency(dependency, request.task_dependency_facts()))
        .collect::<Vec<_>>();

    TaskDependencyCoordinationDecision::new(
        request
            .task_dependency_set()
            .task_dependency_graph_reference()
            .clone(),
        aggregate_status(&task_dependency_decisions),
        task_dependency_decisions,
        None,
    )
}

fn evaluate_dependency(
    task_dependency: super::TaskDependency,
    task_dependency_facts: &[TaskDependencyFact],
) -> TaskDependencyDecision {
    let Some(task_dependency_fact) = task_dependency_facts.iter().find(|fact| {
        fact.task_state_snapshot().task_instance_reference()
            == task_dependency
                .task_dependency_source()
                .task_instance_reference()
    }) else {
        return TaskDependencyDecision::new(
            task_dependency,
            TaskDependencyStatus::Unresolved,
            None,
            Some(TaskDependencyUnresolvedReason::MissingDependencyFact),
        );
    };

    match task_dependency.task_dependency_requirement() {
        TaskDependencyRequirement::AnyTerminal
            if matches!(
                task_dependency_fact.task_state_snapshot().task_state(),
                TaskState::Completed
                    | TaskState::Failed
                    | TaskState::Cancelled
                    | TaskState::Archived
            ) =>
        {
            satisfied(task_dependency)
        }
        TaskDependencyRequirement::SuccessfulCompletion
            if task_dependency_fact.task_state_snapshot().task_state() == TaskState::Completed =>
        {
            satisfied(task_dependency)
        }
        TaskDependencyRequirement::Evidence(task_evidence_reference)
            if task_dependency_fact
                .task_evidence_references()
                .contains(task_evidence_reference) =>
        {
            satisfied(task_dependency)
        }
        TaskDependencyRequirement::Output(task_output_contract)
            if task_dependency_fact
                .task_output_contracts()
                .contains(task_output_contract) =>
        {
            satisfied(task_dependency)
        }
        TaskDependencyRequirement::AnyTerminal => unsatisfied(
            task_dependency,
            TaskDependencyBlocker::RequiredCompletionMissing,
        ),
        TaskDependencyRequirement::SuccessfulCompletion => unsatisfied(
            task_dependency,
            TaskDependencyBlocker::RequiredSuccessMissing,
        ),
        TaskDependencyRequirement::Evidence(_) => unsatisfied(
            task_dependency,
            TaskDependencyBlocker::RequiredEvidenceMissing,
        ),
        TaskDependencyRequirement::Output(_) => unsatisfied(
            task_dependency,
            TaskDependencyBlocker::RequiredOutputMissing,
        ),
    }
}

fn satisfied(task_dependency: super::TaskDependency) -> TaskDependencyDecision {
    TaskDependencyDecision::new(task_dependency, TaskDependencyStatus::Satisfied, None, None)
}

fn unsatisfied(
    task_dependency: super::TaskDependency,
    task_dependency_blocker: TaskDependencyBlocker,
) -> TaskDependencyDecision {
    TaskDependencyDecision::new(
        task_dependency,
        TaskDependencyStatus::Unsatisfied,
        Some(task_dependency_blocker),
        None,
    )
}

fn aggregate_status(task_dependency_decisions: &[TaskDependencyDecision]) -> TaskDependencyStatus {
    if task_dependency_decisions
        .iter()
        .any(|decision| decision.task_dependency_status() == TaskDependencyStatus::Unresolved)
    {
        TaskDependencyStatus::Unresolved
    } else if task_dependency_decisions
        .iter()
        .any(|decision| decision.task_dependency_status() == TaskDependencyStatus::Unsatisfied)
    {
        TaskDependencyStatus::Unsatisfied
    } else {
        TaskDependencyStatus::Satisfied
    }
}
