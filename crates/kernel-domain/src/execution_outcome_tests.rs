use crate::execution_test_support::{
    accepted_completion, accepted_failure, execution_session, task_instance_reference,
    transition_reason,
};
use crate::{ExecutionOutcome, ExecutionTermination};

#[test]
fn execution_outcome_succeeded_preserves_completion_identity_k8_004() {
    let outcome = ExecutionOutcome::succeeded(
        execution_session(),
        accepted_completion(),
        crate::request::TimeReference::new("2026-07-18T00:30:00Z").expect("time"),
    )
    .expect("succeeded");
    assert_eq!(
        outcome.execution_termination(),
        ExecutionTermination::Succeeded
    );
}

#[test]
fn execution_outcome_failed_preserves_failure_identity_k8_004() {
    let outcome = ExecutionOutcome::failed(
        execution_session(),
        accepted_failure(),
        crate::request::TimeReference::new("2026-07-18T00:31:00Z").expect("time"),
    )
    .expect("failed");
    assert_eq!(
        outcome.execution_termination(),
        ExecutionTermination::Failed
    );
}

#[test]
fn execution_outcome_rejects_succeeded_termination_without_completion_k8_004() {
    let error = ExecutionOutcome::terminated(
        execution_session(),
        ExecutionTermination::Succeeded,
        transition_reason(),
        crate::request::TimeReference::new("2026-07-18T00:32:00Z").expect("time"),
    )
    .expect_err("succeeded termination without completion must fail");
    assert!(error
        .to_string()
        .contains("explicit completion or failure facts"));
}

#[test]
fn execution_outcome_rejects_cross_task_completion_mismatch_k8_004() {
    let other_completion = accepted_completion();
    assert_eq!(
        other_completion
            .task_completion_result()
            .task_instance_reference(),
        &task_instance_reference()
    );
    let other_task_reference = crate::TaskInstanceReference::new(
        crate::TaskInstanceId::new("task.instance-0002").expect("task"),
    );
    let readiness = crate::TaskReadinessControl::evaluate(&crate::TaskReadinessInput::new(
        other_task_reference.clone(),
        crate::TaskState::InProgress,
        None,
        None,
        None,
        vec![crate::TaskReadinessRequirement::DependenciesComplete],
        vec![crate::TaskReadinessEvidence::DependenciesComplete],
        None,
    ));
    let cloned = crate::ExecutionSession::new(
        crate::ExecutionRequest::new(
            crate::ExecutionSessionId::new("execution.session-0002").expect("id"),
            other_task_reference.clone(),
            crate::TaskStateSnapshot::new(
                other_task_reference.clone(),
                crate::TaskState::InProgress,
                crate::state::StateSequence::new(1).expect("seq"),
            ),
            readiness,
            crate::execution_test_support::authorization_reference(
                crate::authorization::AuthorizationDecisionOutcome::Allow,
            ),
            crate::request::TimeReference::new("2026-07-18T00:20:00Z").expect("time"),
        )
        .expect("request"),
        crate::ExecutionContext::new(
            crate::ExecutionSessionId::new("execution.session-0002").expect("id"),
            other_task_reference.clone(),
            crate::execution_test_support::runtime_state_snapshot(),
            None,
            None,
            None,
            vec![],
        )
        .expect("context"),
        crate::ExecutionEvidenceBinding::new(
            crate::ExecutionSessionId::new("execution.session-0002").expect("id"),
            other_task_reference,
            vec![crate::TaskEvidenceReference::new(
                crate::TaskEvidenceId::new("CX-TEVID-000003").expect("id"),
            )],
            vec![crate::TaskOutputReference::new("execution.output.reference").expect("output")],
            vec![crate::execution_test_support::transition_evidence()],
        )
        .expect("binding"),
        crate::ExecutionAuditReference::new(
            crate::ExecutionSessionId::new("execution.session-0002").expect("id"),
            None,
            vec![crate::AuditEvidenceId::new("CX-AUD-000003").expect("audit")],
        )
        .expect("audit"),
        crate::request::TimeReference::new("2026-07-18T00:21:00Z").expect("time"),
    )
    .expect("session");
    let error = ExecutionOutcome::succeeded(
        cloned,
        other_completion,
        crate::request::TimeReference::new("2026-07-18T00:30:00Z").expect("time"),
    )
    .expect_err("cross-task completion must fail");
    assert!(error.to_string().contains("task instance continuity"));
}
