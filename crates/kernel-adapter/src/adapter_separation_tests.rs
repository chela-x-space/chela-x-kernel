use crate::adapter_test_support::{adapter_command_envelope, adapter_query_envelope};

#[test]
fn adapter_contracts_remain_transport_neutral_k14_003() {
    let command = adapter_command_envelope();
    let query = adapter_query_envelope();

    assert_ne!(
        command.adapter_request_id().as_str(),
        command
            .service_request_context()
            .service_request_id()
            .as_str()
    );
    assert_ne!(
        query.adapter_request_id().as_str(),
        query
            .service_request_context()
            .service_request_id()
            .as_str()
    );
    assert_ne!(
        command
            .service_request_context()
            .service_request_id()
            .as_str(),
        query
            .service_request_context()
            .service_request_id()
            .as_str()
    );
}

#[test]
fn adapter_contracts_preserve_service_boundary_without_lower_layer_bypass_k14_010() {
    let adapter_intent = crate::adapter_test_support::adapter_command_intent();
    assert_eq!(
        adapter_intent
            .service_command_intent()
            .service_request_context()
            .service_identity()
            .service_identifier(),
        "service.integration.primary"
    );
}
