use crate::service_test_support::{
    service_api_version, service_command_capability, service_query_capability,
};
use crate::{
    ServiceCapabilityDeclaration, ServiceErrorCode, ServiceIdentity, ServiceIdentityKind,
};

#[test]
fn service_identity_constructs_valid_namespaced_contract_k13_002() {
    let identity = ServiceIdentity::new(
        "service.integration.primary",
        ServiceIdentityKind::ExternalService,
        "service.integration",
        "2026.07.19",
        vec![service_command_capability(), service_query_capability()],
        Some("service.environment.primary".to_owned()),
    )
    .expect("identity");
    assert_eq!(identity.service_identifier(), "service.integration.primary");
}

#[test]
fn service_identity_rejects_missing_namespace_k13_002() {
    let error = ServiceIdentity::new(
        "service",
        ServiceIdentityKind::ExternalService,
        "service.integration",
        "2026.07.19",
        vec![service_command_capability()],
        None,
    )
    .expect_err("namespace-free identifiers must fail");
    assert_eq!(error.code(), ServiceErrorCode::InvalidServiceIdentity);
}

#[test]
fn service_capability_declaration_rejects_missing_canonical_query_capability_k13_003() {
    let error = ServiceCapabilityDeclaration::new(
        service_api_version(),
        vec![service_command_capability()],
        true,
        true,
    )
    .expect_err("query admission requires the canonical query capability");
    assert_eq!(error.code(), ServiceErrorCode::CapabilityMismatch);
}
