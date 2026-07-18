use super::{
    TaskCompletion, TaskEvidenceControl, TaskEvidenceValidation, TaskEvidenceValidationRequest,
    TaskFailure, TaskFailurePolicyReference, TaskInstance, TaskInstanceReference,
    TaskStateSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFailureRejectionReason {
    TaskInstanceMismatch,
    FailurePolicyMismatch,
    UndeclaredEvidence,
    CompletionFailureConflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureRejected {
    task_failure: TaskFailure,
    reason: TaskFailureRejectionReason,
}

impl TaskFailureRejected {
    fn new(task_failure: TaskFailure, reason: TaskFailureRejectionReason) -> Self {
        Self {
            task_failure,
            reason,
        }
    }

    pub fn task_failure(&self) -> &TaskFailure {
        &self.task_failure
    }
    pub fn reason(&self) -> TaskFailureRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskFailureOutcome {
    Accepted(TaskFailure),
    Rejected(TaskFailureRejected),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureValidationRequest {
    task_instance: TaskInstance,
    task_state_snapshot: TaskStateSnapshot,
    task_failure: TaskFailure,
    task_completion: Option<TaskCompletion>,
}

impl TaskFailureValidationRequest {
    pub fn new(
        task_instance: TaskInstance,
        task_state_snapshot: TaskStateSnapshot,
        task_failure: TaskFailure,
        task_completion: Option<TaskCompletion>,
    ) -> Self {
        Self {
            task_instance,
            task_state_snapshot,
            task_failure,
            task_completion,
        }
    }

    pub fn task_instance(&self) -> &TaskInstance {
        &self.task_instance
    }
    pub fn task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.task_state_snapshot
    }
    pub fn task_failure(&self) -> &TaskFailure {
        &self.task_failure
    }
    pub fn task_completion(&self) -> Option<&TaskCompletion> {
        self.task_completion.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskFailureControl;

impl TaskFailureControl {
    pub fn evaluate(request: &TaskFailureValidationRequest) -> TaskFailureOutcome {
        if request.task_completion().is_some() {
            return rejected(
                request,
                TaskFailureRejectionReason::CompletionFailureConflict,
            );
        }
        let task_instance_reference =
            TaskInstanceReference::new(request.task_instance().task_instance_id().clone());
        if request.task_state_snapshot().task_instance_reference() != &task_instance_reference
            || request.task_failure().task_instance_reference() != &task_instance_reference
            || request
                .task_failure()
                .task_failure_evidence_set()
                .task_instance_reference()
                != &task_instance_reference
        {
            return rejected(request, TaskFailureRejectionReason::TaskInstanceMismatch);
        }
        if failure_policy_mismatch(
            request
                .task_instance()
                .task_definition()
                .task_failure_policy_reference(),
            request.task_failure().task_failure_policy_reference(),
        ) {
            return rejected(request, TaskFailureRejectionReason::FailurePolicyMismatch);
        }
        match TaskEvidenceControl::validate(&TaskEvidenceValidationRequest::new(
            request.task_instance().clone(),
            request.task_failure().task_failure_evidence_set().clone(),
        )) {
            TaskEvidenceValidation::Accepted(_) => {
                TaskFailureOutcome::Accepted(request.task_failure().clone())
            }
            TaskEvidenceValidation::Rejected(_) => {
                rejected(request, TaskFailureRejectionReason::UndeclaredEvidence)
            }
        }
    }
}

fn failure_policy_mismatch(
    expected: Option<&TaskFailurePolicyReference>,
    actual: Option<&TaskFailurePolicyReference>,
) -> bool {
    match (expected, actual) {
        (Some(expected), Some(actual)) => expected != actual,
        (None, None) => false,
        _ => true,
    }
}

fn rejected(
    request: &TaskFailureValidationRequest,
    reason: TaskFailureRejectionReason,
) -> TaskFailureOutcome {
    TaskFailureOutcome::Rejected(TaskFailureRejected::new(
        request.task_failure().clone(),
        reason,
    ))
}
