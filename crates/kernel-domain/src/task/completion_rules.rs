use super::{
    TaskCompletionRejectionReason, TaskCompletionRequirement, TaskCompletionValidationRequest,
    TaskInstanceReference, TaskState,
};

pub(super) fn task_instance_matches(request: &TaskCompletionValidationRequest) -> bool {
    let task_instance_reference =
        TaskInstanceReference::new(request.task_instance().task_instance_id().clone());
    request.task_state_snapshot().task_instance_reference() == &task_instance_reference
        && request.task_completion_result().task_instance_reference() == &task_instance_reference
        && request
            .task_completion_result()
            .task_evidence_set()
            .task_instance_reference()
            == &task_instance_reference
}

pub(super) fn completion_after_failure(
    request: &TaskCompletionValidationRequest,
) -> Option<TaskCompletionRejectionReason> {
    (request.task_state_snapshot().task_state() == TaskState::Failed
        && request.task_recovery_reference().is_none())
    .then_some(TaskCompletionRejectionReason::CompletionAfterFailureWithoutRecovery)
}

pub(super) fn definition_mismatch(
    request: &TaskCompletionValidationRequest,
) -> Option<TaskCompletionRejectionReason> {
    (request
        .task_completion_result()
        .task_definition_snapshot_reference()
        != request.task_instance().task_definition_snapshot_reference()
        || request
            .task_completion_result()
            .task_completion_requirements()
            .iter()
            .any(|requirement| {
                !request
                    .task_instance()
                    .task_definition()
                    .task_completion_requirements()
                    .contains(requirement)
            }))
    .then_some(TaskCompletionRejectionReason::DefinitionMismatch)
}

pub(super) fn missing_required_completion_requirement(
    request: &TaskCompletionValidationRequest,
) -> Option<TaskCompletionRejectionReason> {
    request
        .task_instance()
        .task_definition()
        .task_completion_requirements()
        .iter()
        .any(|requirement: &TaskCompletionRequirement| {
            !request
                .task_completion_result()
                .task_completion_requirements()
                .contains(requirement)
        })
        .then_some(TaskCompletionRejectionReason::MissingCompletionRequirement)
}

pub(super) fn validate_outputs(
    request: &TaskCompletionValidationRequest,
) -> Option<TaskCompletionRejectionReason> {
    let outputs = request.task_completion_result().task_outputs();
    if outputs.iter().enumerate().any(|(index, output)| {
        outputs[..index].iter().any(|prior| {
            prior.task_output_binding().task_output_contract()
                == output.task_output_binding().task_output_contract()
        })
    }) {
        return Some(TaskCompletionRejectionReason::DuplicateOutput);
    }
    if outputs.iter().any(|output| {
        !request
            .task_instance()
            .task_definition()
            .task_output_contracts()
            .contains(output.task_output_binding().task_output_contract())
    }) {
        return Some(TaskCompletionRejectionReason::UndeclaredOutput);
    }
    request
        .task_instance()
        .task_definition()
        .task_output_contracts()
        .iter()
        .any(|contract| {
            !outputs
                .iter()
                .any(|output| output.task_output_binding().task_output_contract() == contract)
        })
        .then_some(TaskCompletionRejectionReason::MissingRequiredOutput)
}
