use crate::adapter_test_support::{
    adapter_api_version, adapter_capability_declaration, adapter_identity, service_api_version,
    service_status_snapshot,
};
use crate::{
    AdapterCompatibilityReference, AdapterErrorCode, AdapterIntentKind, AdapterStatusSnapshot,
    AdapterValidationStatus,
};

#[test]
fn adapter_status_snapshot_preserves_validation_status_k14_009() {
    let snapshot = crate::adapter_test_support::adapter_status_snapshot();
    assert_eq!(
        snapshot.adapter_validation_status(),
        AdapterValidationStatus::Validated
    );
}

#[test]
fn adapter_status_snapshot_requires_compatibility_references_k14_010() {
    let error = AdapterStatusSnapshot::new(
        adapter_api_version(),
        adapter_identity(),
        adapter_capability_declaration(),
        vec![AdapterIntentKind::Command, AdapterIntentKind::Query],
        vec![],
        service_api_version(),
        service_status_snapshot(),
        AdapterValidationStatus::Validated,
        "2026-07-19T00:10:00Z",
    )
    .expect_err("compatibility references must be present");
    assert_eq!(error.code(), AdapterErrorCode::ServiceCompatibilityMismatch);
}

#[test]
fn adapter_status_snapshot_rejects_pending_service_validation_k14_010() {
    let error = AdapterStatusSnapshot::new(
        adapter_api_version(),
        adapter_identity(),
        adapter_capability_declaration(),
        vec![AdapterIntentKind::Command, AdapterIntentKind::Query],
        vec![
            AdapterCompatibilityReference::new("adapter.compatibility.current")
                .expect("compatibility"),
        ],
        service_api_version(),
        kernel_service::ServiceStatusSnapshot::new(
            crate::adapter_test_support::service_api_version(),
            crate::adapter_test_support::service_identity(),
            crate::adapter_test_support::service_capability_declaration(),
            vec![
                kernel_service::ServiceIntentKind::Command,
                kernel_service::ServiceIntentKind::Query,
            ],
            vec![
                kernel_service::ServiceDependencyCompatibilityReference::new(
                    "service.compatibility.current",
                )
                .expect("compatibility"),
            ],
            crate::adapter_test_support::application_api_version(),
            kernel_application::ApplicationValidationStatus::Validated,
            kernel_service::ServiceValidationStatus::ValidationPending,
            "2026-07-19T00:10:00Z",
        )
        .expect("service status"),
        AdapterValidationStatus::Validated,
        "2026-07-19T00:10:00Z",
    )
    .expect_err("validated adapter status requires validated service status");
    assert_eq!(error.code(), AdapterErrorCode::ServiceCompatibilityMismatch);
}
