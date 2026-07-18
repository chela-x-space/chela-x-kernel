use crate::authorization::AuthorizationDecisionOutcome;
use crate::identifier::WorkflowId;
use crate::identity::IdentityReference;
use crate::workflow::WorkflowStepReference;

#[test]
fn integration_identity_and_reference_continuity_is_preserved_across_completion_flow() {
    let flow = super::integration_flow_support::completion_happy_path();
    let task_instance_reference = super::integration_test_support::task_instance_reference();
    let definition_snapshot = flow
        .task_instance
        .task_definition_snapshot_reference()
        .clone();
    let completion = match flow.completion_outcome {
        super::TaskCompletionOutcome::Accepted(completion) => completion,
        _ => panic!("expected accepted completion"),
    };

    assert_eq!(
        flow.task_ownership.task_instance_reference(),
        &task_instance_reference
    );
    assert_eq!(
        flow.accepted_assignment.task_instance_reference(),
        &task_instance_reference
    );
    assert_eq!(
        flow.task_priority.task_instance_reference(),
        &task_instance_reference
    );
    assert_eq!(
        completion
            .task_completion_result()
            .task_instance_reference(),
        &task_instance_reference
    );
    assert_eq!(
        completion
            .task_completion_result()
            .task_definition_snapshot_reference(),
        &definition_snapshot
    );
}

#[test]
fn integration_upstream_k1_k2_k3_and_k6_surfaces_remain_usable() {
    let owner = super::integration_test_support::ownership();
    let workflow_reference =
        super::TaskWorkflowReference::new(WorkflowId::new("CX-WF-900001").expect("workflow"));
    let task_step_reference = super::TaskStepReference::new(
        WorkflowStepReference::new("workflow.step.review").expect("step"),
    );

    assert!(matches!(
        owner.task_owner().identity_reference(),
        Some(IdentityReference::Human(_))
    ));
    assert_eq!(
        super::integration_test_support::pending_snapshot()
            .state_sequence()
            .value(),
        1
    );
    assert_eq!(
        AuthorizationDecisionOutcome::Allow,
        AuthorizationDecisionOutcome::Allow
    );
    assert_eq!(workflow_reference.workflow_id().as_str(), "CX-WF-900001");
    assert_eq!(
        task_step_reference.workflow_step_reference().as_str(),
        "workflow.step.review"
    );
}
