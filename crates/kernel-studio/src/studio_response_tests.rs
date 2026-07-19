use crate::studio_test_support::{
    correlation_id, gateway_status_envelope, gateway_status_response_envelope, ownership_path,
    studio_api_version, studio_audit_reference, studio_selection_context, studio_view_reference,
};
use crate::{
    StudioAttentionState, StudioTopViewProjection, StudioViewKind, StudioViewProjection,
    StudioViewRequest, StudioViewResponse,
};
use kernel_domain::{AgentId, ExecutionSessionId};

#[test]
fn studio_view_response_preserves_correlation_k11_009() {
    let request = StudioViewRequest::new(
        studio_api_version(),
        studio_view_reference(StudioViewKind::TopView),
        studio_selection_context(),
        None,
        None,
        correlation_id(),
        crate::studio_test_support::time_reference(),
        vec![gateway_status_envelope()],
        studio_audit_reference(),
    )
    .expect("request");
    let projection = StudioViewProjection::TopView(Box::new(
        StudioTopViewProjection::new(
            ownership_path(),
            vec![AgentId::new("CX-AGT-000001").expect("agent")],
            vec![crate::studio_test_support::runtime_id()],
            vec![crate::studio_test_support::workflow_id()],
            vec![crate::studio_test_support::task_instance_reference()],
            vec![ExecutionSessionId::new("execution.session-0001").expect("execution")],
            StudioAttentionState::Nominal,
            studio_audit_reference(),
        )
        .expect("projection"),
    ));
    let response = StudioViewResponse::new(
        &request,
        correlation_id(),
        projection,
        vec![gateway_status_response_envelope()],
        studio_audit_reference(),
        crate::studio_test_support::later_time_reference(),
    )
    .expect("response");
    let _ = response;
}
