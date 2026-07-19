use crate::studio_test_support::{
    memory_projection, ownership_path, runtime_state_snapshot, studio_audit_reference,
    task_state_snapshot, workflow_state_snapshot,
};
use crate::StudioDigitalTwinProjection;

#[test]
fn studio_digital_twin_projection_preserves_governed_state_k11_002() {
    let projection = StudioDigitalTwinProjection::new(
        ownership_path(),
        vec![runtime_state_snapshot()],
        vec![workflow_state_snapshot()],
        vec![task_state_snapshot()],
        vec![memory_projection()],
        kernel_domain::TimeReference::new("2026-07-19T00:05:00Z").expect("time"),
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(
        projection.workflow_state_snapshots()[0]
            .workflow_id()
            .as_str(),
        "CX-WF-000001"
    );
}
