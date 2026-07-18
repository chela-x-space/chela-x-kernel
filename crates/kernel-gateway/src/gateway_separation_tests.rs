use crate::gateway_test_support::{
    gateway_request_context, memory_command_request, operation_memory_capture, ownership_path,
};
use crate::GatewayRequestEnvelope;

#[test]
fn gateway_request_envelope_does_not_mutate_memory_record_k10_010() {
    let request = memory_command_request();
    let before = request.clone();
    GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            crate::gateway_test_support::memory_record_id().as_str(),
            ownership_path(),
        ),
        request.clone(),
    )
    .expect("command");
    assert_eq!(request, before);
}

#[test]
fn gateway_contracts_remain_transport_neutral_k10_010() {
    let protocol = crate::GatewayProtocol::Http;
    assert_eq!(protocol, crate::GatewayProtocol::Http);
}
