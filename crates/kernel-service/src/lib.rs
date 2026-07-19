#![forbid(unsafe_code)]

pub mod service;
pub mod service_capability;
pub mod service_command;
pub mod service_context;
pub mod service_error;
pub mod service_identity;
pub mod service_query;
pub mod service_response;
pub mod service_status;
pub mod service_validation;

pub use service::{
    ServiceApiVersion, ServiceIntentKind, ServiceResponseKind, SERVICE_COMMAND_CAPABILITY,
    SERVICE_QUERY_CAPABILITY,
};
pub use service_capability::{ServiceCapabilityDeclaration, ServiceCapabilityReference};
pub use service_command::ServiceCommandIntent;
pub use service_context::{ServiceRequestContext, ServiceRequestId};
pub use service_error::{ServiceError, ServiceErrorCode, ServiceResult};
pub use service_identity::{ServiceIdentity, ServiceIdentityKind};
pub use service_query::ServiceQueryIntent;
pub use service_response::{ServiceResponseEnvelope, ServiceResponseStatusReference};
pub use service_status::{
    ServiceDependencyCompatibilityReference, ServiceStatusSnapshot, ServiceValidationStatus,
};

#[cfg(test)]
mod service_command_tests;
#[cfg(test)]
mod service_conformance_tests;
#[cfg(test)]
mod service_context_tests;
#[cfg(test)]
mod service_contract_tests;
#[cfg(test)]
mod service_query_tests;
#[cfg(test)]
mod service_response_tests;
#[cfg(test)]
mod service_status_tests;
#[cfg(test)]
mod service_test_support;
