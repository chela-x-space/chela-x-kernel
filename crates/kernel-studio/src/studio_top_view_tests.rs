use crate::studio_test_support::{
    ownership_path, runtime_id, studio_audit_reference, task_instance_reference, workflow_id,
};
use crate::{StudioAttentionState, StudioTopViewProjection};
use kernel_domain::{AgentId, ExecutionSessionId};

#[test]
fn studio_top_view_projection_preserves_high_level_subjects_k11_001() {
    let projection = StudioTopViewProjection::new(
        ownership_path(),
        vec![AgentId::new("CX-AGT-000001").expect("agent")],
        vec![runtime_id()],
        vec![workflow_id()],
        vec![task_instance_reference()],
        vec![ExecutionSessionId::new("execution.session-0001").expect("execution")],
        StudioAttentionState::NeedsAttention,
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(projection.workflow_ids()[0].as_str(), "CX-WF-000001");
}
