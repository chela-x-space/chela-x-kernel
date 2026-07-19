use crate::adapter_test_support::{
    adapter_api_version, adapter_command_capability, adapter_query_capability,
};
use crate::{
    AdapterCapabilityDeclaration, AdapterErrorCode, AdapterIdentity, AdapterIdentityKind,
    AdapterKind,
};

#[test]
fn adapter_api_version_constructs_transport_neutral_contract_k14_001() {
    let version = adapter_api_version();
    assert_eq!(version.as_str(), "2026.07.19");
}

#[test]
fn adapter_api_version_rejects_transport_specific_marker_k14_001() {
    let error = crate::AdapterApiVersion::new("http/v1")
        .expect_err("transport-specific version markers must fail");
    assert_eq!(error.code(), AdapterErrorCode::UnsupportedAdapterVersion);
}

#[test]
fn adapter_identity_constructs_valid_namespaced_contract_k14_002() {
    let identity = AdapterIdentity::new(
        "adapter.integration.primary",
        AdapterIdentityKind::ExternalAdapter,
        AdapterKind::ExternalAdapter,
        "adapter.integration",
        "2026.07.19",
        vec![adapter_command_capability(), adapter_query_capability()],
        Some("adapter.environment.primary".to_owned()),
    )
    .expect("identity");
    assert_eq!(identity.adapter_identifier(), "adapter.integration.primary");
}

#[test]
fn adapter_identity_rejects_missing_namespace_k14_002() {
    let error = AdapterIdentity::new(
        "adapter",
        AdapterIdentityKind::ExternalAdapter,
        AdapterKind::ExternalAdapter,
        "adapter.integration",
        "2026.07.19",
        vec![adapter_command_capability()],
        None,
    )
    .expect_err("namespace-free identifiers must fail");
    assert_eq!(error.code(), AdapterErrorCode::InvalidAdapterIdentity);
}

#[test]
fn adapter_capability_declaration_rejects_missing_canonical_query_capability_k14_003() {
    let error = AdapterCapabilityDeclaration::new(
        adapter_api_version(),
        vec![adapter_command_capability()],
        true,
        true,
    )
    .expect_err("query admission requires the canonical query capability");
    assert_eq!(error.code(), AdapterErrorCode::CapabilityMismatch);
}
