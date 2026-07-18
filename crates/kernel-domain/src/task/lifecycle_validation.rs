use super::{
    lifecycle_transition::{allow, noop, reject},
    TaskAssignmentStatus, TaskReadiness, TaskReadinessDecision, TaskState, TaskTransitionDecision,
    TaskTransitionRejectionReason, TaskTransitionRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskTransitionControl;

impl TaskTransitionControl {
    pub fn evaluate(request: &TaskTransitionRequest) -> TaskTransitionDecision {
        let current = request.current_task_state_snapshot();
        let target = request.requested_target_task_state();

        if current.task_state() == target {
            return noop(request);
        }
        if request
            .task_lifecycle_guards()
            .expected_current_sequence()
            .is_some_and(|sequence| sequence != current.state_sequence())
        {
            return reject(request, TaskTransitionRejectionReason::SequenceMismatch);
        }
        if current.task_state() == TaskState::Archived {
            return reject(request, TaskTransitionRejectionReason::TerminalState);
        }

        match (current.task_state(), target) {
            (TaskState::Pending, TaskState::InProgress) => validate_start(request),
            (TaskState::Pending, TaskState::Cancelled)
            | (TaskState::InProgress, TaskState::Cancelled) => {
                require_reason_and_authority(request)
            }
            (TaskState::InProgress, TaskState::Completed) => validate_completion(request),
            (TaskState::InProgress, TaskState::Failed) => validate_failure(request),
            (TaskState::Completed, TaskState::Archived)
            | (TaskState::Failed, TaskState::Archived)
            | (TaskState::Cancelled, TaskState::Archived) => require_authority(request),
            (TaskState::Completed, _) | (TaskState::Failed, _) | (TaskState::Cancelled, _) => {
                reject(request, TaskTransitionRejectionReason::TerminalState)
            }
            _ => reject(request, TaskTransitionRejectionReason::IllegalTransition),
        }
    }
}

fn validate_start(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    if !matches!(
        request.task_readiness_decision(),
        Some(TaskReadinessDecision::Ready(ready))
            if ready.task_readiness() == TaskReadiness::Ready
    ) {
        return reject(
            request,
            TaskTransitionRejectionReason::ReadinessNotSatisfied,
        );
    }
    if request.task_lifecycle_guards().assignment_required()
        && !request.task_assignment().is_some_and(|assignment| {
            assignment.task_assignment_status() == TaskAssignmentStatus::Accepted
        })
    {
        return reject(request, TaskTransitionRejectionReason::AssignmentRequired);
    }
    if !request.task_lifecycle_guards().authorization_allowed() {
        return reject(
            request,
            TaskTransitionRejectionReason::AuthorizationNotAllowed,
        );
    }
    if !request.task_lifecycle_guards().dependencies_satisfied() {
        return reject(
            request,
            TaskTransitionRejectionReason::DependenciesNotSatisfied,
        );
    }
    allow(request)
}

fn validate_completion(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    if !request.task_lifecycle_guards().completion_conditions_met() {
        return reject(
            request,
            TaskTransitionRejectionReason::CompletionConditionsRequired,
        );
    }
    if !request.task_lifecycle_guards().required_outputs_present() {
        return reject(
            request,
            TaskTransitionRejectionReason::RequiredOutputsMissing,
        );
    }
    if !request
        .task_lifecycle_guards()
        .required_completion_evidence_present()
    {
        return reject(request, TaskTransitionRejectionReason::MissingEvidence);
    }
    allow(request)
}

fn validate_failure(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    if request.task_lifecycle_guards().failure_code().is_none() {
        return reject(request, TaskTransitionRejectionReason::FailureCodeRequired);
    }
    if request.task_lifecycle_guards().failure_category().is_none() {
        return reject(
            request,
            TaskTransitionRejectionReason::FailureCategoryRequired,
        );
    }
    if !request
        .task_lifecycle_guards()
        .required_failure_evidence_present()
    {
        return reject(request, TaskTransitionRejectionReason::MissingEvidence);
    }
    allow(request)
}

fn require_reason_and_authority(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    if request.transition_reason_reference().is_none() {
        return reject(request, TaskTransitionRejectionReason::MissingReason);
    }
    require_authority(request)
}

fn require_authority(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    if request.transition_authority_reference().is_none() {
        return reject(request, TaskTransitionRejectionReason::MissingAuthority);
    }
    allow(request)
}
