use super::{
    TaskCompletionControl, TaskCompletionOutcome, TaskCompletionRejectionReason, TaskEvidenceSet,
    TaskOutput, TaskOutputBinding, TaskOutputReference, TaskState,
};

#[test]
fn task_completion_result_preserves_bindings_and_order() {
    let result = super::outcome_test_support::completion_result(
        super::outcome_test_support::valid_outputs(),
        super::outcome_test_support::evidence_set(Some(
            super::outcome_test_support::evidence_requirement(),
        )),
        vec![super::outcome_test_support::completion_requirement()],
    );

    assert_eq!(
        result.task_instance_reference(),
        &super::outcome_test_support::task_instance_reference()
    );
    assert_eq!(
        result.task_definition_snapshot_reference(),
        super::outcome_test_support::task_instance().task_definition_snapshot_reference()
    );
    assert_eq!(
        result.task_outputs(),
        &super::outcome_test_support::valid_outputs()
    );
}

#[test]
fn task_completion_rejects_duplicate_output_reference() {
    let duplicate = TaskOutput::new(
        TaskOutputReference::new("task.output.primary").expect("output reference"),
        TaskOutputBinding::new(super::outcome_test_support::output_contract_primary()),
    );
    let error = super::TaskCompletionResult::new(
        super::outcome_test_support::task_instance_reference(),
        super::outcome_test_support::task_instance()
            .task_definition_snapshot_reference()
            .clone(),
        vec![super::outcome_test_support::completion_requirement()],
        vec![duplicate.clone(), duplicate],
        super::outcome_test_support::evidence_set(Some(
            super::outcome_test_support::evidence_requirement(),
        )),
        None,
        None,
    )
    .expect_err("duplicate output reference must reject");

    assert_eq!(
        error,
        crate::errors::DomainError::InvalidTaskCompletion("duplicate task output reference")
    );
}

#[test]
fn task_completion_accepts_valid_explicit_result() {
    let decision = TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(TaskState::InProgress),
    );

    assert!(matches!(decision, TaskCompletionOutcome::Accepted(_)));
}

#[test]
fn task_completion_rejects_missing_required_output() {
    let request = super::TaskCompletionValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(TaskState::InProgress),
        super::outcome_test_support::completion_result(
            vec![super::outcome_test_support::valid_outputs()[0].clone()],
            super::outcome_test_support::evidence_set(Some(
                super::outcome_test_support::evidence_requirement(),
            )),
            vec![super::outcome_test_support::completion_requirement()],
        ),
        None,
    );

    let decision = TaskCompletionControl::evaluate(&request);
    assert!(matches!(
        decision,
        TaskCompletionOutcome::Rejected(rejected)
            if rejected.reason() == TaskCompletionRejectionReason::MissingRequiredOutput
    ));
}

#[test]
fn task_completion_rejects_undeclared_output() {
    let undeclared_output = TaskOutput::new(
        TaskOutputReference::new("task.output.undeclared").expect("output reference"),
        TaskOutputBinding::new(
            super::TaskOutputContract::new("task.output.undeclared").expect("output contract"),
        ),
    );
    let request = super::TaskCompletionValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(TaskState::InProgress),
        super::outcome_test_support::completion_result(
            vec![
                super::outcome_test_support::valid_outputs()[0].clone(),
                undeclared_output,
            ],
            super::outcome_test_support::evidence_set(Some(
                super::outcome_test_support::evidence_requirement(),
            )),
            vec![super::outcome_test_support::completion_requirement()],
        ),
        None,
    );

    let decision = TaskCompletionControl::evaluate(&request);
    assert!(matches!(
        decision,
        TaskCompletionOutcome::Rejected(rejected)
            if rejected.reason() == TaskCompletionRejectionReason::UndeclaredOutput
    ));
}

#[test]
fn task_completion_rejects_missing_required_evidence() {
    let request = super::TaskCompletionValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(TaskState::InProgress),
        super::outcome_test_support::completion_result(
            super::outcome_test_support::valid_outputs(),
            TaskEvidenceSet::new(
                super::outcome_test_support::task_instance_reference(),
                Vec::new(),
            )
            .expect("empty evidence set"),
            vec![super::outcome_test_support::completion_requirement()],
        ),
        None,
    );

    let decision = TaskCompletionControl::evaluate(&request);
    assert!(matches!(
        decision,
        TaskCompletionOutcome::Rejected(rejected)
            if rejected.reason() == TaskCompletionRejectionReason::MissingRequiredEvidence
    ));
}

#[test]
fn task_completion_rejects_after_terminal_failure_without_recovery() {
    let decision = TaskCompletionControl::evaluate(
        &super::outcome_test_support::completion_request(TaskState::Failed),
    );

    assert!(matches!(
        decision,
        TaskCompletionOutcome::Rejected(rejected)
            if rejected.reason()
                == TaskCompletionRejectionReason::CompletionAfterFailureWithoutRecovery
    ));
}

#[test]
fn task_completion_allows_recovery_bound_completion_after_failure() {
    let request = super::TaskCompletionValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::state_snapshot(TaskState::Failed),
        super::outcome_test_support::completion_result(
            super::outcome_test_support::valid_outputs(),
            super::outcome_test_support::evidence_set(Some(
                super::outcome_test_support::evidence_requirement(),
            )),
            vec![super::outcome_test_support::completion_requirement()],
        ),
        Some(super::outcome_test_support::recovery_reference()),
    );

    assert!(matches!(
        TaskCompletionControl::evaluate(&request),
        TaskCompletionOutcome::Accepted(_)
    ));
}
