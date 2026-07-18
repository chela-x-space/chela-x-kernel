use kernel_domain::{AuthorizationDecisionOutcome, AuthorizationRequestId};

use crate::gateway_test_support::{
    authorization_binding, authorization_request_record, ownership_path,
};
use crate::GatewayAuthorizationBinding;

#[test]
fn gateway_authorization_binding_preserves_request_and_decision_identity_k10_003() {
    let binding = authorization_binding(
        "memory",
        "gateway.memory.record.000001",
        ownership_path(),
        AuthorizationDecisionOutcome::Allow,
    );
    assert_eq!(
        binding.authorization_request_record().request_id(),
        binding.authorization_decision_reference().request_id()
    );
}

#[test]
fn gateway_authorization_binding_rejects_request_decision_mismatch_k10_003() {
    let request = authorization_request_record(
        "memory",
        "gateway.memory.record.000001",
        ownership_path(),
        AuthorizationRequestId::new("CX-AUTHREQ-000111").expect("request"),
    );
    let decision = crate::gateway_test_support::authorization_binding(
        "memory",
        "gateway.memory.record.000001",
        ownership_path(),
        AuthorizationDecisionOutcome::Allow,
    )
    .authorization_decision_reference()
    .clone();
    let error =
        GatewayAuthorizationBinding::new(request, decision).expect_err("mismatched ids must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidRequest);
}
