use crate::studio_test_support::{correlation_id, gateway_audit_reference, time_reference};
use crate::StudioAuditProjection;
use kernel_domain::{AuditEvidenceId, EventTraceReference};

#[test]
fn studio_audit_projection_preserves_correlation_continuity_k11_008() {
    let projection = StudioAuditProjection::new(
        "workflow.subject",
        Some(
            kernel_gateway::GatewayOperationReference::new(
                kernel_gateway::GatewayOperationKind::Query,
                "workflow.state",
            )
            .expect("operation"),
        ),
        None,
        correlation_id(),
        Some(EventTraceReference::new("studio.cause.000001").expect("cause")),
        time_reference(),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
        "allowed",
        Some(gateway_audit_reference()),
        None,
    )
    .expect("projection");
    assert_eq!(projection.correlation_id().as_str(), "CX-COR-000001");
}
