#![forbid(unsafe_code)]

pub mod gateway;
pub mod gateway_authentication;
pub mod gateway_authorization;
pub mod gateway_command;
pub mod gateway_contract;
pub mod gateway_error;
pub mod gateway_protocol;
pub mod gateway_query;
pub mod gateway_request;
pub mod gateway_response;
pub mod gateway_validation;

pub use gateway::{GatewayAuditReference, GatewayStatusSnapshot};
pub use gateway_authentication::GatewayAuthenticationContext;
pub use gateway_authorization::GatewayAuthorizationBinding;
pub use gateway_command::{GatewayCommandPayload, GatewayCommandRequest, GatewayCommandResponse};
pub use gateway_contract::{GatewayApiVersion, GatewayOperationKind, GatewayOperationReference};
pub use gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
pub use gateway_protocol::{GatewayProtocol, GatewayRateGovernanceReference};
pub use gateway_query::{GatewayQueryPayload, GatewayQueryRequest, GatewayQueryResponse};
pub use gateway_request::GatewayRequestContext;
pub use gateway_request::GatewayRequestEnvelope;
pub use gateway_response::GatewayResponseEnvelope;

#[cfg(test)]
mod gateway_authentication_tests;
#[cfg(test)]
mod gateway_authorization_tests;
#[cfg(test)]
mod gateway_command_tests;
#[cfg(test)]
mod gateway_conformance_tests;
#[cfg(test)]
mod gateway_contract_tests;
#[cfg(test)]
mod gateway_query_tests;
#[cfg(test)]
mod gateway_request_tests;
#[cfg(test)]
mod gateway_response_tests;
#[cfg(test)]
mod gateway_separation_tests;
#[cfg(test)]
mod gateway_test_support;
