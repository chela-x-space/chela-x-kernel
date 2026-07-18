use crate::gateway_test_support::{gateway_api_version, operation_memory_capture};
use crate::{GatewayApiVersion, GatewayOperationKind, GatewayOperationReference};

#[test]
fn gateway_api_version_constructs_with_transport_neutral_value_k10_001() {
    let version = gateway_api_version();
    assert_eq!(version.as_str(), "2026.07.18");
}

#[test]
fn gateway_api_version_rejects_transport_path_value_k10_001() {
    let error = GatewayApiVersion::new("/v1").expect_err("transport path must fail");
    assert_eq!(
        error.code(),
        crate::GatewayErrorCode::UnsupportedGatewayVersion
    );
}

#[test]
fn gateway_operation_reference_preserves_kind_and_namespace_k10_001() {
    let operation = operation_memory_capture();
    assert_eq!(operation.operation_kind(), GatewayOperationKind::Command);
    assert_eq!(operation.resource_segment(), "memory");
}

#[test]
fn gateway_operation_reference_requires_namespace_shape_k10_001() {
    let error = GatewayOperationReference::new(GatewayOperationKind::Command, "memory")
        .expect_err("non-namespaced operation must fail");
    assert_eq!(error.code(), crate::GatewayErrorCode::UnsupportedOperation);
}
