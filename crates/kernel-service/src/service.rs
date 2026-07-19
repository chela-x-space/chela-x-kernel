use kernel_application::ApplicationResponseKind;

use crate::service_error::{ServiceErrorCode, ServiceResult};
use crate::service_validation::validate_version_reference;

pub const SERVICE_COMMAND_CAPABILITY: &str = "service.command";
pub const SERVICE_QUERY_CAPABILITY: &str = "service.query";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceIntentKind {
    Command,
    Query,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceResponseKind {
    View,
    Command,
    Error,
}

impl From<ApplicationResponseKind> for ServiceResponseKind {
    fn from(value: ApplicationResponseKind) -> Self {
        match value {
            ApplicationResponseKind::View => Self::View,
            ApplicationResponseKind::Command => Self::Command,
            ApplicationResponseKind::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceApiVersion(String);

impl ServiceApiVersion {
    pub fn new(value: impl Into<String>) -> ServiceResult<Self> {
        Ok(Self(validate_version_reference(
            value,
            ServiceErrorCode::UnsupportedServiceVersion,
            "service API version must be namespace-safe and transport-neutral",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
