use crate::application_test_support::{
    application_api_version, application_identity, application_request_id,
};
use crate::{ApplicationErrorCode, ApplicationIdentity, ApplicationIdentityKind};

#[test]
fn application_identity_constructs_valid_namespaced_contract_k12_001() {
    let identity = application_identity();
    assert_eq!(
        identity.application_identifier().as_str(),
        "application.integration.primary"
    );
}

#[test]
fn application_identity_rejects_missing_namespace_k12_001() {
    let error = ApplicationIdentity::new(
        "application",
        ApplicationIdentityKind::ExternalApplication,
        "application.integration",
        "2026.07.19",
        vec![],
        None,
    )
    .expect_err("missing namespace must fail");
    assert_eq!(
        error.code(),
        ApplicationErrorCode::InvalidApplicationIdentity
    );
}

#[test]
fn application_api_version_preserves_caller_supplied_value_k12_001() {
    let version = application_api_version();
    assert_eq!(version.as_str(), "2026.07.19");
    assert_eq!(
        application_request_id().as_str(),
        "application.request.000001"
    );
}
