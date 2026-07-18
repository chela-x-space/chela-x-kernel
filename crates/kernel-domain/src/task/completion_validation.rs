use super::{
    completion_rules, TaskCompletion, TaskCompletionResult, TaskCompletionValidationRequest,
    TaskEvidenceControl, TaskEvidenceRejectionReason, TaskEvidenceValidation,
    TaskEvidenceValidationRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskCompletionRejectionReason {
    DefinitionMismatch,
    TaskInstanceMismatch,
    MissingCompletionRequirement,
    MissingRequiredOutput,
    UndeclaredOutput,
    DuplicateOutput,
    MissingRequiredEvidence,
    UndeclaredEvidence,
    CompletionAfterFailureWithoutRecovery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCompletionRejected {
    task_completion_result: TaskCompletionResult,
    reason: TaskCompletionRejectionReason,
}

impl TaskCompletionRejected {
    fn new(
        task_completion_result: TaskCompletionResult,
        reason: TaskCompletionRejectionReason,
    ) -> Self {
        Self {
            task_completion_result,
            reason,
        }
    }

    pub fn task_completion_result(&self) -> &TaskCompletionResult {
        &self.task_completion_result
    }
    pub fn reason(&self) -> TaskCompletionRejectionReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskCompletionOutcome {
    Accepted(TaskCompletion),
    Rejected(TaskCompletionRejected),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskCompletionControl;

impl TaskCompletionControl {
    pub fn evaluate(request: &TaskCompletionValidationRequest) -> TaskCompletionOutcome {
        if !completion_rules::task_instance_matches(request) {
            return rejected(request, TaskCompletionRejectionReason::TaskInstanceMismatch);
        }
        if let Some(reason) = completion_rules::definition_mismatch(request) {
            return rejected(request, reason);
        }
        if let Some(reason) = completion_rules::completion_after_failure(request) {
            return rejected(request, reason);
        }
        if let Some(reason) = completion_rules::missing_required_completion_requirement(request) {
            return rejected(request, reason);
        }
        if let Some(reason) = completion_rules::validate_outputs(request) {
            return rejected(request, reason);
        }
        match TaskEvidenceControl::validate(&TaskEvidenceValidationRequest::new(
            request.task_instance().clone(),
            request.task_completion_result().task_evidence_set().clone(),
        )) {
            TaskEvidenceValidation::Accepted(_) => {}
            TaskEvidenceValidation::Rejected(rejected_evidence) => {
                let reason = match rejected_evidence.reason() {
                    TaskEvidenceRejectionReason::TaskInstanceMismatch => {
                        TaskCompletionRejectionReason::TaskInstanceMismatch
                    }
                    TaskEvidenceRejectionReason::UndeclaredEvidence => {
                        TaskCompletionRejectionReason::UndeclaredEvidence
                    }
                };
                return rejected(request, reason);
            }
        }
        if request
            .task_instance()
            .task_definition()
            .task_evidence_requirements()
            .iter()
            .any(|requirement| {
                !request
                    .task_completion_result()
                    .task_evidence_set()
                    .task_evidences()
                    .iter()
                    .any(|evidence| {
                        evidence
                            .task_evidence_metadata()
                            .task_evidence_requirement()
                            == Some(requirement)
                    })
            })
        {
            return rejected(
                request,
                TaskCompletionRejectionReason::MissingRequiredEvidence,
            );
        }

        TaskCompletionOutcome::Accepted(TaskCompletion::new(
            request.task_completion_result().clone(),
        ))
    }
}

fn rejected(
    request: &TaskCompletionValidationRequest,
    reason: TaskCompletionRejectionReason,
) -> TaskCompletionOutcome {
    TaskCompletionOutcome::Rejected(TaskCompletionRejected::new(
        request.task_completion_result().clone(),
        reason,
    ))
}
