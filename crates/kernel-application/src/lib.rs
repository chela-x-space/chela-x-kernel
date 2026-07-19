#![forbid(unsafe_code)]

pub mod application;
pub mod application_capability;
pub mod application_command;
pub mod application_context;
pub mod application_error;
pub mod application_identity;
pub mod application_navigation;
pub mod application_query;
pub mod application_response;
pub mod application_session;
pub mod application_status;
pub mod application_validation;

pub use application::{
    ApplicationApiVersion, ApplicationIntentKind, ApplicationRequestEnvelope,
    ApplicationResponseKind,
};
pub use application_capability::{
    ApplicationCapabilityDeclaration, ApplicationCapabilityReference,
};
pub use application_command::ApplicationCommandIntent;
pub use application_context::{
    ApplicationAuditReference, ApplicationRequestContext, ApplicationRequestId,
};
pub use application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
pub use application_identity::{ApplicationIdentity, ApplicationIdentityKind};
pub use application_navigation::ApplicationViewIntent;
pub use application_query::ApplicationQueryIntent;
pub use application_response::{
    ApplicationResponseEnvelope, ApplicationResponsePayload, ApplicationResponseStatusReference,
};
pub use application_session::{ApplicationSessionReference, ApplicationSessionStatusReference};
pub use application_status::{
    ApplicationDependencyCompatibilityReference, ApplicationStatusSnapshot,
    ApplicationValidationStatus,
};

#[cfg(test)]
mod application_command_tests;
#[cfg(test)]
mod application_conformance_tests;
#[cfg(test)]
mod application_context_tests;
#[cfg(test)]
mod application_contract_tests;
#[cfg(test)]
mod application_navigation_tests;
#[cfg(test)]
mod application_query_tests;
#[cfg(test)]
mod application_response_tests;
#[cfg(test)]
mod application_status_tests;
#[cfg(test)]
mod application_test_support;
