use super::{
    integration_test_support, TaskAssignment, TaskAssignmentDecision, TaskCompletionControl,
    TaskCompletionOutcome, TaskCompletionValidationRequest, TaskDependencyCoordinationDecision,
    TaskFailureControl, TaskFailureOutcome, TaskFailureValidationRequest, TaskOwnership,
    TaskPriority, TaskReadinessDecision, TaskTransitionControl, TaskTransitionDecision,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CompletionFlow {
    pub(super) task_instance: super::TaskInstance,
    pub(super) task_ownership: TaskOwnership,
    pub(super) assignment_decision: TaskAssignmentDecision,
    pub(super) accepted_assignment: TaskAssignment,
    pub(super) task_priority: TaskPriority,
    pub(super) dependency_decision: TaskDependencyCoordinationDecision,
    pub(super) readiness_decision: TaskReadinessDecision,
    pub(super) start_decision: TaskTransitionDecision,
    pub(super) completion_outcome: TaskCompletionOutcome,
    pub(super) completed_decision: TaskTransitionDecision,
    pub(super) archived_decision: TaskTransitionDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FailureFlow {
    pub(super) task_instance: super::TaskInstance,
    pub(super) accepted_assignment: TaskAssignment,
    pub(super) readiness_decision: TaskReadinessDecision,
    pub(super) start_decision: TaskTransitionDecision,
    pub(super) failure_outcome: TaskFailureOutcome,
    pub(super) failed_decision: TaskTransitionDecision,
    pub(super) archived_decision: TaskTransitionDecision,
}

pub(super) fn completion_happy_path() -> CompletionFlow {
    let task_instance = super::outcome_test_support::task_instance();
    let task_ownership = integration_test_support::ownership();
    let assignment_decision = integration_test_support::assigned_decision();
    let accepted_assignment = integration_test_support::accepted_assignment();
    let task_priority = integration_test_support::priority();
    let dependency_decision = integration_test_support::satisfied_dependency_decision();
    let readiness_decision = integration_test_support::ready_readiness_decision(
        Some(task_ownership.clone()),
        Some(accepted_assignment.clone()),
        Some(task_priority.clone()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::DependenciesComplete,
            super::TaskReadinessEvidence::AuthorizationAllowed,
            super::TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
    );
    let start_decision = TaskTransitionControl::evaluate(&integration_test_support::start_request(
        integration_test_support::pending_snapshot(),
        readiness_decision.clone(),
        accepted_assignment.clone(),
        true,
    ));
    let in_progress_snapshot = integration_test_support::allowed_snapshot(&start_decision);
    let completion_outcome =
        TaskCompletionControl::evaluate(&TaskCompletionValidationRequest::new(
            task_instance.clone(),
            in_progress_snapshot.clone(),
            super::outcome_test_support::completion_result(
                super::outcome_test_support::valid_outputs(),
                super::outcome_test_support::required_evidence_set(
                    super::outcome_test_support::evidence_requirement(),
                ),
                vec![super::outcome_test_support::completion_requirement()],
            ),
            None,
        ));
    let completed_decision = TaskTransitionControl::evaluate(
        &integration_test_support::complete_request(in_progress_snapshot),
    );
    let archived_decision =
        TaskTransitionControl::evaluate(&integration_test_support::archive_request(
            integration_test_support::allowed_snapshot(&completed_decision),
        ));

    CompletionFlow {
        task_instance,
        task_ownership,
        assignment_decision,
        accepted_assignment,
        task_priority,
        dependency_decision,
        readiness_decision,
        start_decision,
        completion_outcome,
        completed_decision,
        archived_decision,
    }
}

pub(super) fn failure_happy_path() -> FailureFlow {
    let task_instance = super::outcome_test_support::task_instance();
    let accepted_assignment = integration_test_support::accepted_assignment();
    let readiness_decision = integration_test_support::ready_readiness_decision(
        Some(integration_test_support::ownership()),
        Some(accepted_assignment.clone()),
        Some(integration_test_support::priority()),
        vec![
            super::TaskReadinessEvidence::RequiredInputAvailable,
            super::TaskReadinessEvidence::DependenciesComplete,
            super::TaskReadinessEvidence::AuthorizationAllowed,
            super::TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
    );
    let start_decision = TaskTransitionControl::evaluate(&integration_test_support::start_request(
        integration_test_support::pending_snapshot(),
        readiness_decision.clone(),
        accepted_assignment.clone(),
        true,
    ));
    let in_progress_snapshot = integration_test_support::allowed_snapshot(&start_decision);
    let failure_outcome = TaskFailureControl::evaluate(&TaskFailureValidationRequest::new(
        task_instance.clone(),
        in_progress_snapshot.clone(),
        super::outcome_test_support::failure(
            super::outcome_test_support::failure_evidence_set(),
            Some(
                super::TaskFailurePolicyReference::new("task.failure.policy.demo").expect("policy"),
            ),
        ),
        None,
    ));
    let failed_decision = TaskTransitionControl::evaluate(
        &integration_test_support::failure_request(in_progress_snapshot),
    );
    let archived_decision =
        TaskTransitionControl::evaluate(&integration_test_support::archive_request(
            integration_test_support::allowed_snapshot(&failed_decision),
        ));

    FailureFlow {
        task_instance,
        accepted_assignment,
        readiness_decision,
        start_decision,
        failure_outcome,
        failed_decision,
        archived_decision,
    }
}
