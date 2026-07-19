use crate::studio_test_support::{runtime_id, runtime_state_snapshot, studio_audit_reference};
use crate::StudioRuntimeProjection;
use kernel_domain::ExecutionSessionId;

#[test]
fn studio_runtime_projection_preserves_runtime_identity_k11_003() {
    let projection = StudioRuntimeProjection::new(
        runtime_id(),
        vec![runtime_state_snapshot()],
        vec![ExecutionSessionId::new("execution.session-0001").expect("execution")],
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(projection.selected_runtime_id().as_str(), "CX-RUN-000001");
}
