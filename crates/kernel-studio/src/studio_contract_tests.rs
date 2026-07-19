use crate::studio_test_support::{correlation_id, gateway_audit_reference, studio_audit_reference};
use crate::{StudioApiVersion, StudioAuditReference, StudioViewKind, StudioViewReference};
use kernel_domain::{AuditEvidenceId, EventTraceReference};

#[test]
fn studio_api_version_preserves_namespace_safe_value_k11_001() {
    let version = StudioApiVersion::new("2026.07.19").expect("version");
    assert_eq!(version.as_str(), "2026.07.19");
}

#[test]
fn studio_view_reference_rejects_missing_namespace_k11_001() {
    let error = StudioViewReference::new(StudioViewKind::TopView, "topview")
        .expect_err("view name must be namespaced");
    assert_eq!(error.code(), crate::StudioErrorCode::UnsupportedView);
}

#[test]
fn studio_audit_reference_rejects_gateway_correlation_mismatch_k11_010() {
    let error = StudioAuditReference::new(
        EventTraceReference::new("studio.audit.trace.999999").expect("trace"),
        Some(correlation_id()),
        vec![AuditEvidenceId::new("CX-AUD-999999").expect("audit")],
        Some(
            kernel_gateway::GatewayAuditReference::new(
                EventTraceReference::new("gateway.audit.trace.999999").expect("trace"),
                None,
                vec![AuditEvidenceId::new("CX-AUD-999998").expect("audit")],
            )
            .expect("audit"),
        ),
    )
    .expect_err("gateway correlation mismatch must fail");
    assert_eq!(error.code(), crate::StudioErrorCode::AuditReferenceMismatch);
    let _ = gateway_audit_reference();
    let _ = studio_audit_reference();
}
