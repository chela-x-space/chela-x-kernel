#![forbid(unsafe_code)]

pub mod adapter;
pub mod adapter_capability;
pub mod adapter_command;
pub mod adapter_context;
pub mod adapter_error;
pub mod adapter_identity;
pub mod adapter_query;
pub mod adapter_request;
pub mod adapter_response;
pub mod adapter_status;
pub mod adapter_validation;

pub use adapter::{
    AdapterApiVersion, AdapterIntentKind, AdapterKind, AdapterResponseKind,
    ADAPTER_COMMAND_CAPABILITY, ADAPTER_QUERY_CAPABILITY,
};
pub use adapter_capability::{AdapterCapabilityDeclaration, AdapterCapabilityReference};
pub use adapter_command::AdapterCommandIntent;
pub use adapter_context::{AdapterRequestContext, AdapterRequestId};
pub use adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};
pub use adapter_identity::{AdapterIdentity, AdapterIdentityKind};
pub use adapter_query::AdapterQueryIntent;
pub use adapter_request::AdapterRequestEnvelope;
pub use adapter_response::{AdapterResponseEnvelope, AdapterResponseStatusReference};
pub use adapter_status::{
    AdapterCompatibilityReference, AdapterStatusSnapshot, AdapterValidationStatus,
};

#[cfg(test)]
mod adapter_command_tests;
#[cfg(test)]
mod adapter_conformance_tests;
#[cfg(test)]
mod adapter_context_tests;
#[cfg(test)]
mod adapter_contract_tests;
#[cfg(test)]
mod adapter_query_tests;
#[cfg(test)]
mod adapter_response_tests;
#[cfg(test)]
mod adapter_separation_tests;
#[cfg(test)]
mod adapter_status_tests;
#[cfg(test)]
mod adapter_test_support;
