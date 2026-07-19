use crate::studio_test_support::{studio_audit_reference, task_priority, task_state_snapshot};
use crate::StudioTaskProjection;

#[test]
fn studio_task_projection_preserves_task_priority_k11_005() {
    let projection = StudioTaskProjection::new(
        task_state_snapshot(),
        Some(task_priority()),
        None,
        None,
        None,
        None,
        None,
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(
        projection
            .task_priority()
            .expect("priority")
            .task_priority_value()
            .value(),
        5
    );
}
