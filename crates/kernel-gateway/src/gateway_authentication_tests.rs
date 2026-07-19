use crate::gateway_test_support::{authentication_context, principal_reference, requested_at};
use crate::GatewayAuthenticationContext;

#[test]
fn gateway_authentication_context_preserves_authenticated_identity_k10_002() {
    let context = authentication_context();
    assert_eq!(context.authenticated_principal(), &principal_reference());
    assert_eq!(context.authentication_method_reference(), "auth.password");
}

#[test]
fn gateway_authentication_context_rejects_missing_method_reference_k10_002() {
    let error = GatewayAuthenticationContext::new(
        principal_reference(),
        "",
        kernel_domain::EventTraceReference::new("gateway.auth.trace.999999").expect("trace"),
        requested_at(),
        None,
        None,
    )
    .expect_err("empty authentication method must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::InvalidRequest);
}
