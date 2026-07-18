use crate::gateway_test_support::{
    gateway_api_version, gateway_request_context, gateway_status_snapshot,
    operation_gateway_status, ownership_path,
};

#[test]
fn gateway_status_snapshot_preserves_contract_inventory_k10_010() {
    let snapshot = gateway_status_snapshot();
    assert_eq!(snapshot.gateway_api_version(), &gateway_api_version());
    assert!(!snapshot.supported_operations().is_empty());
}

#[test]
fn gateway_request_context_preserves_frozen_identity_values_k10_010() {
    let context = gateway_request_context(
        operation_gateway_status(),
        "gateway",
        "status",
        ownership_path(),
    );
    assert_eq!(
        context.ownership_path().enterprise_id().as_str(),
        "CX-ENT-000001"
    );
    assert_eq!(context.correlation_id().as_str(), "CX-COR-000001");
}
