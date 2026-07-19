use crate::studio_test_support::{
    correlation_id, gateway_status_envelope, studio_api_version, studio_audit_reference,
    studio_selection_context, studio_view_reference, time_reference,
};
use crate::{StudioViewKind, StudioViewRequest};

#[test]
fn studio_view_requests_preserve_frozen_gateway_boundary_k11_010() {
    let request = StudioViewRequest::new(
        studio_api_version(),
        studio_view_reference(StudioViewKind::TopView),
        studio_selection_context(),
        None,
        None,
        correlation_id(),
        time_reference(),
        vec![gateway_status_envelope()],
        studio_audit_reference(),
    )
    .expect("request");
    assert_eq!(request.gateway_request_envelopes().len(), 1);
}
