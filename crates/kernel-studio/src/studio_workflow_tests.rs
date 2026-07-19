use crate::studio_test_support::{
    studio_audit_reference, task_instance_reference, workflow_state_snapshot,
    workflow_step_reference,
};
use crate::StudioWorkflowProjection;
use kernel_domain::ExecutionSessionId;

#[test]
fn studio_workflow_projection_preserves_step_state_k11_004() {
    let projection = StudioWorkflowProjection::new(
        workflow_state_snapshot(),
        Some(workflow_step_reference("step.current")),
        vec![workflow_step_reference("step.done")],
        vec![workflow_step_reference("step.blocked")],
        vec![task_instance_reference()],
        vec![ExecutionSessionId::new("execution.session-0001").expect("execution")],
        Some(kernel_domain::WorkflowFailureCode::InvalidTransition),
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(
        projection.current_step_reference().expect("step").as_str(),
        "step.current"
    );
}
