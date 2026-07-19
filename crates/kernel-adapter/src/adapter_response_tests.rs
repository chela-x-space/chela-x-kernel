use crate::adapter_test_support::{
    adapter_command_envelope, adapter_query_envelope, service_query_response_envelope,
};
use crate::{
    AdapterErrorCode, AdapterResponseEnvelope, AdapterResponseKind, AdapterResponseStatusReference,
};

#[test]
fn adapter_response_envelope_preserves_service_response_k14_008() {
    let response = crate::adapter_test_support::adapter_response_envelope();
    assert_eq!(response.adapter_response_kind(), AdapterResponseKind::View);
}

#[test]
fn adapter_response_envelope_rejects_request_response_mismatch_k14_010() {
    let error = AdapterResponseEnvelope::new(
        &adapter_query_envelope(),
        crate::adapter_test_support::service_command_response_envelope(),
        AdapterResponseStatusReference::new("adapter.response.complete").expect("status"),
        "2026-07-19T00:10:00Z",
    )
    .expect_err("mismatched responses must fail");
    assert_eq!(error.code(), AdapterErrorCode::ResponseRequestMismatch);
}

#[test]
fn adapter_response_envelope_rejects_response_kind_mismatch_k14_010() {
    let error = AdapterResponseEnvelope::new(
        &adapter_command_envelope(),
        service_query_response_envelope(),
        AdapterResponseStatusReference::new("adapter.response.complete").expect("status"),
        "2026-07-19T00:10:00Z",
    )
    .expect_err("query responses must not satisfy command envelopes");
    assert_eq!(error.code(), AdapterErrorCode::ResponseRequestMismatch);
}
