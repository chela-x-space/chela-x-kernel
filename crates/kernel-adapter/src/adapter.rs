use kernel_service::ServiceResponseKind;

use crate::adapter_error::{AdapterErrorCode, AdapterResult};
use crate::adapter_request::AdapterRequestEnvelope;
use crate::adapter_validation::validate_version_reference;

pub const ADAPTER_COMMAND_CAPABILITY: &str = "adapter.command";
pub const ADAPTER_QUERY_CAPABILITY: &str = "adapter.query";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterKind {
    ExternalSystem,
    ExternalAdapter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterIntentKind {
    Command,
    Query,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterResponseKind {
    View,
    Command,
    Error,
}

impl From<ServiceResponseKind> for AdapterResponseKind {
    fn from(value: ServiceResponseKind) -> Self {
        match value {
            ServiceResponseKind::View => Self::View,
            ServiceResponseKind::Command => Self::Command,
            ServiceResponseKind::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterApiVersion(String);

impl AdapterApiVersion {
    pub fn new(value: impl Into<String>) -> AdapterResult<Self> {
        Ok(Self(validate_version_reference(
            value,
            AdapterErrorCode::UnsupportedAdapterVersion,
            "adapter API version must be namespace-safe and transport-neutral",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AdapterRequestEnvelope {
    pub fn adapter_api_version(&self) -> &crate::AdapterApiVersion {
        match self {
            Self::Command(intent) => intent.adapter_request_context().adapter_api_version(),
            Self::Query(intent) => intent.adapter_request_context().adapter_api_version(),
        }
    }
}
