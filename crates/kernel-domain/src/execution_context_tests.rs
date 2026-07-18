use crate::execution_test_support::{
    execution_session_id, runtime_state_snapshot, task_instance, task_instance_reference,
};
use crate::runtime::{LeaseValidity, PresenceState};
use crate::workflow::WorkflowStepReference;
use crate::{ExecutionContext, TaskStepReference, TaskWorkflowReference, WorkflowId};

#[test]
fn execution_context_preserves_runtime_and_inputs_k8_002() {
    let context = crate::execution_test_support::execution_context();
    assert_eq!(context.execution_session_id(), &execution_session_id());
    assert_eq!(
        context.task_instance_reference(),
        &task_instance_reference()
    );
    assert_eq!(
        context
            .runtime_state_snapshot()
            .lease_assessment()
            .validity(),
        LeaseValidity::Valid
    );
}

#[test]
fn execution_context_rejects_step_without_workflow_k8_002() {
    let error = ExecutionContext::new(
        execution_session_id(),
        task_instance_reference(),
        runtime_state_snapshot(),
        None,
        None,
        Some(TaskStepReference::new(
            WorkflowStepReference::new("step.execute").expect("step"),
        )),
        task_instance()
            .task_creation_context()
            .task_input_bindings()
            .to_vec(),
    )
    .expect_err("step without workflow must fail");
    assert!(error.to_string().contains("workflow reference"));
}

#[test]
fn execution_context_rejects_duplicate_input_binding_k8_002() {
    let binding = task_instance()
        .task_creation_context()
        .task_input_bindings()[0]
        .clone();
    let error = ExecutionContext::new(
        execution_session_id(),
        task_instance_reference(),
        runtime_state_snapshot(),
        None,
        Some(TaskWorkflowReference::new(
            WorkflowId::new("CX-WF-000001").expect("wf"),
        )),
        None,
        vec![binding.clone(), binding],
    )
    .expect_err("duplicate inputs must fail");
    assert!(error.to_string().contains("duplicate task input binding"));
}

#[test]
fn runtime_snapshot_fixture_is_operational_for_execution_k8_002() {
    let snapshot = runtime_state_snapshot();
    assert!(!matches!(
        snapshot.presence(),
        PresenceState::Offline | PresenceState::Retired
    ));
}
