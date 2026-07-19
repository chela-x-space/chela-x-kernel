use crate::service_test_support::{
    application_api_version, service_api_version, service_capability_declaration, service_identity,
};
use crate::{ServiceErrorCode, ServiceIntentKind, ServiceStatusSnapshot, ServiceValidationStatus};
use kernel_application::ApplicationValidationStatus;

#[test]
fn service_status_snapshot_preserves_validation_status_k13_008() {
    let snapshot = crate::service_test_support::service_status_snapshot();
    assert_eq!(
        snapshot.service_validation_status(),
        ServiceValidationStatus::Validated
    );
}

#[test]
fn service_status_snapshot_requires_compatibility_references_k13_010() {
    let error = ServiceStatusSnapshot::new(
        service_api_version(),
        service_identity(),
        service_capability_declaration(),
        vec![ServiceIntentKind::Command, ServiceIntentKind::Query],
        vec![],
        application_api_version(),
        ApplicationValidationStatus::Validated,
        ServiceValidationStatus::Validated,
        "2026-07-19T00:10:00Z",
    )
    .expect_err("compatibility references must be present");
    assert_eq!(error.code(), ServiceErrorCode::CompatibilityMismatch);
}
