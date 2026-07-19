use crate::adapter_test_support::{
    adapter_api_version, adapter_capability_declaration, adapter_identity,
    command_service_request_context,
};
use crate::{AdapterErrorCode, AdapterRequestContext, AdapterRequestId};

#[test]
fn adapter_request_context_preserves_service_boundary_k14_006() {
    let context = AdapterRequestContext::new(
        adapter_api_version(),
        AdapterRequestId::new("adapter.request.command").expect("request"),
        adapter_identity(),
        adapter_capability_declaration(),
        command_service_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect("context");
    assert_eq!(
        context.adapter_request_id().as_str(),
        "adapter.request.command"
    );
}

#[test]
fn adapter_request_context_rejects_capability_version_mismatch_k14_009() {
    let error = AdapterRequestContext::new(
        adapter_api_version(),
        AdapterRequestId::new("adapter.request.command").expect("request"),
        adapter_identity(),
        crate::AdapterCapabilityDeclaration::new(
            crate::AdapterApiVersion::new("2026.07.18").expect("version"),
            vec![
                crate::adapter_test_support::adapter_command_capability(),
                crate::adapter_test_support::adapter_query_capability(),
            ],
            true,
            true,
        )
        .expect("capability"),
        command_service_request_context(),
        "2026-07-19T00:00:00Z",
    )
    .expect_err("capability version mismatch must fail");
    assert_eq!(error.code(), AdapterErrorCode::CapabilityMismatch);
}
